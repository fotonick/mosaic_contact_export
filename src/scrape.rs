use anyhow::Result;
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};

use crate::types::Person;

pub fn scrape_people_from_str(body: &str) -> Result<Vec<Person>> {
    let mut people = Vec::new();
    let fragment = Html::parse_fragment(body);
    let row_selector =
        Selector::parse("div#button_nav_0 > div > table.table > tbody > tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    fn extract_tel(elem: ElementRef) -> Option<String> {
        let field = elem.inner_html();
        if field.len() >= 10 { Some(field) } else { None }
    }

    fn expand_addr(elem: ElementRef) -> String {
        format!("{} NE Going St, Portland, OR 97218", elem.inner_html()).to_string()
    }

    let email_selector = Selector::parse("a").unwrap();
    let extract_email = move |elem: ElementRef| {
        elem.select(&email_selector)
            .next()
            .map(|email_elem| email_elem.inner_html())
    };

    for row in fragment.select(&row_selector) {
        let (_, first_elem, last_elem, house_number_elem, tel_elem, mail_link_elem, _) = row
            .select(&col_selector)
            .collect_tuple()
            .expect("Expected exactly 7 cells per row");
        people.push(Person {
            last: last_elem.inner_html(),
            first: first_elem.inner_html(),
            addr: expand_addr(house_number_elem),
            tel: extract_tel(tel_elem),
            email: extract_email(mail_link_elem),
        })
    }

    Ok(people)
}
