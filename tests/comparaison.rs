#[cfg(test)]
mod tests {
    use cm_lib::vcard::property_match;
    use vcard_parser::{traits::HasName, vcard::property::Property};

    #[test]
    fn compare_properties() {
        let p_filter = Property::create_from_str("FN:Jean\n").unwrap();
        let p_vcard = Property::create_from_str("FN:Jean-Marie\n").unwrap();

        // Jean != Jean-Marie
        assert!(!property_match(&p_filter, &p_vcard, false).unwrap());
        // Jean-Marie contains Jean, return true with forgive.
        assert!(property_match(&p_filter, &p_vcard, true).unwrap());

        let p_filter = Property::create_from_str("EMAIL:\n").unwrap();
        let p_vcard =
            Property::create_from_str("EMAIL;type=INTERNET;type=HOME;type=pref:user@example.com\n")
                .unwrap();
        assert!(p_filter.name() == p_vcard.name());
        assert!(property_match(&p_filter, &p_vcard, false).unwrap());
        let p_filter = Property::create_from_str("EMAIL;type=HOME:\n").unwrap();
        assert!(property_match(&p_filter, &p_vcard, false).unwrap());
        let p_filter = Property::create_from_str("EMAIL;type=WORK:\n").unwrap();

        assert!(!property_match(&p_filter, &p_vcard, false).unwrap());
    }
}
