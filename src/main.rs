mod mosaic_site;
mod scrape;
mod types;
mod vcard;

use anyhow::Result;
use calcard::vcard::VCardVersion;
use std::env;

use crate::mosaic_site::MosaicSite;
use crate::scrape::scrape_people_from_str;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut args = env::args();
    let url_base = args.nth(1).expect("Expect URL_BASE as first argument");
    let address_completion = args
        .next()
        .expect("Expect address completion as second argument");

    // Login
    let site = MosaicSite::new(&url_base)?;
    let logged_in_site = site.login()?;

    // Download
    let html = logged_in_site.get_contacts()?;

    // Scrape
    let members = scrape_people_from_str(&html, &address_completion)?;

    // Dump
    let mut output = String::new();
    for member in members.iter() {
        member
            .make_vcard()
            .write_to(&mut output, VCardVersion::V3_0)?;
    }
    println!("{}", output);
    Ok(())
}
