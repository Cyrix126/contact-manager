#[cfg(test)]
mod tests {
    use contact_manager_lib::vcard_parser::{
        traits::HasValue,
        vcard::{
            property::{property_adr::PropertyAdrData, Property},
            value::{value_listcomponent::ValueListComponentData, Value},
        },
    };

    #[test]
    fn adr_raw2property() {
        let mut property = Property::PropertyAdr(PropertyAdrData::default());
        let raw_value = ";43;Rue de la Maladière;Dijon;Bourgogne;21000;France";

        let value = Value::ValueListComponent(
            ValueListComponentData::try_from((raw_value, ';', ',')).unwrap(),
        );

        property.set_value(value).unwrap();

        // assert!(!property_match(&p_filter, &p_vcard, false).unwrap());
    }
}
