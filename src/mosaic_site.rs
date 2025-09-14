use std::env;

use anyhow::{Result, anyhow};
use ureq::Agent;
use ureq::config::Config;
use ureq::http::StatusCode;

pub struct MosaicSite {
    base_url: String,
    agent: Agent,
}

pub struct LoggedInMosaicSite {
    base_url: String,
    agent: Agent,
    session_id: String,
}

impl MosaicSite {
    pub fn new(base_url: &str) -> Result<Self> {
        let config = Config::builder()
            .max_redirects(0) // This bit is critical. I'm not sure why it caused a stall, even with a global timeout set, but logging in returns a 303 redirect, which we must ignore.
            .build();
        let agent = Agent::new_with_config(config);

        Ok(MosaicSite {
            base_url: base_url.to_string(),
            agent,
        })
    }
    pub fn login(self) -> Result<LoggedInMosaicSite> {
        // 1. Set JSESSIONID cookie
        self.agent.get(&format!("{}/Home", self.base_url)).call()?;
        let session_id = {
            let jar = self.agent.cookie_jar_lock();
            jar.get("cullygrove.zenlunatics.org", "/", "JSESSIONID")
                .unwrap()
                .value()
                .to_string()
        };

        // 2. Login
        let username =
            env::var("MOSAIC_USERNAME").expect("MOSAIC_USERNAME environment variable not set");
        let password =
            env::var("MOSAIC_PASSWORD").expect("MOSAIC_PASSWORD environment variable not set");
        let request = self
            .agent
            .post(&format!("{}/j_security_check", self.base_url))
            .header("cookie", &format!("JSESSIONID={}", session_id))
            .header("cookie", "setuuid=true");
        let response = request.send_form([("j_username", username), ("j_password", password)])?;
        if !matches!(response.status(), StatusCode::SEE_OTHER) {
            return Err(anyhow!("Invalid username or password"));
        }

        // 3. As long as we use the same session ID and it doesn't expire, we can do authenticated actions
        Ok(LoggedInMosaicSite {
            base_url: self.base_url,
            agent: self.agent,
            session_id,
        })
    }
}

impl LoggedInMosaicSite {
    pub fn get_contacts(&self) -> Result<String> {
        let response = self
            .agent
            .get(&format!("{}/People", self.base_url))
            .header("cookie", &format!("JSESSIONID={}", self.session_id))
            .call()?;
        let html = response.into_body().read_to_string()?;
        Ok(html)
    }
}
