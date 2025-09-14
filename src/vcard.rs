use calcard::vcard::{VCard, VCardEntry, VCardKind, VCardProperty, VCardValue};

use crate::types::Person;

impl Person {
    pub fn make_entries(&self) -> Vec<VCardEntry> {
        let mut entries = Vec::new();
        entries.push(_make_name_entry(&self.first, &self.last));
        entries.push(_make_address_entry(&self.addr));
        if let Some(ref tel) = self.tel {
            entries.push(_make_tel_entry(tel));
        }
        if let Some(ref email) = self.email {
            entries.push(_make_email_entry(email));
        }
        entries
    }

    pub fn make_vcard(&self) -> VCard {
        VCard {
            entries: self.make_entries(),
        }
    }
}

fn _make_group_entry() -> VCardEntry {
    VCardEntry::new(VCardProperty::Kind).with_value(VCardValue::Kind(VCardKind::Group))
}

fn _make_name_entry(first: &str, last: &str) -> VCardEntry {
    VCardEntry::new(VCardProperty::N).with_values(
        [
            VCardValue::Text(last.to_string()),
            VCardValue::Text(first.to_string()),
        ]
        .into(),
    )
}

fn _make_tel_entry(tel: &str) -> VCardEntry {
    VCardEntry::new(VCardProperty::Tel).with_values([VCardValue::Text(tel.to_string())].into())
}

fn _make_address_entry(addr: &str) -> VCardEntry {
    VCardEntry::new(VCardProperty::Adr).with_values([VCardValue::Text(addr.to_string())].into())
}

fn _make_email_entry(email: &str) -> VCardEntry {
    VCardEntry::new(VCardProperty::Email).with_values([VCardValue::Text(email.to_string())].into())
}
