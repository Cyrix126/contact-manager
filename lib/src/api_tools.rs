use uuid::Uuid;
use vcard_parser::{
    constants::PropertyName,
    traits::{HasCardinality, HasParameters, HasValue},
    vcard::{
        parameter::{parameter_pid::ParameterPidData, Parameter},
        property::{property_uid::PropertyUidData, Property},
        value::Value,
    },
};

use crate::error::ErrorContactManager;

/// get the value of the property
pub fn value(property: &Property) -> String {
    property.get_value().to_string()
}
/// get the pid parameter of the property
pub fn pid(property: &Property) -> Option<ParameterPidData> {
    property.get_parameters();
    for param in property.get_parameters() {
        match param {
            Parameter::ParameterPid(d) => return Some(d),
            _ => continue,
        }
    }
    None
}

/// remove parameters from property
pub fn remove_parameters(property: &mut Property, params: &Vec<Parameter>) {
    let mut params_p = property.get_parameters();
    for pm in params {
        if params_p.contains(&pm) {
            params_p.retain(|p| pm != p)
        }
    }
    property.set_parameters(params_p);
}

/// return two vec of Property name, first for single cardinality and second for multiples
pub fn cardinals() -> (Vec<Property>, Vec<Property>) {
    let mut property_single = vec![];
    let mut property_multiple = vec![];
    for name in PropertyName::to_strings() {
        if Property::default(&name).cardinality() == "MULTIPLE" {
            property_multiple.push(Property::default(&name))
        } else {
            property_single.push(Property::default(&name))
        }
    }
    (property_single, property_multiple)
}

/// generate a property with a new uid
pub fn generate_uid_property() -> Result<(Property, Uuid), ErrorContactManager> {
    let uuid = Uuid::new_v4();
    let mut property_uuid = Property::PropertyUid(PropertyUidData::default());
    property_uuid.set_value(Value::ValueUri(
        vcard_parser::vcard::value::value_uri::ValueUriData {
            value: uuid.to_string(),
        },
    ))?;
    Ok((property_uuid, uuid))
}

trait HasConst {
    fn to_strings() -> Vec<String>;
}
impl HasConst for PropertyName {
    fn to_strings() -> Vec<String> {
        vec![
            PropertyName::ADR.to_string(),
            PropertyName::ANNIVERSARY.to_string(),
            PropertyName::BDAY.to_string(),
            PropertyName::BIRTHPLACE.to_string(),
            PropertyName::CALADRURI.to_string(),
            PropertyName::CALURI.to_string(),
            PropertyName::CATEGORIES.to_string(),
            PropertyName::CLIENTPIDMAP.to_string(),
            PropertyName::CONTACTURI.to_string(),
            PropertyName::DEATHDATE.to_string(),
            PropertyName::DEATHPLACE.to_string(),
            PropertyName::EMAIL.to_string(),
            PropertyName::EXPERTISE.to_string(),
            PropertyName::FBURL.to_string(),
            PropertyName::FN.to_string(),
            PropertyName::GENDER.to_string(),
            PropertyName::GEO.to_string(),
            PropertyName::HOBBY.to_string(),
            PropertyName::IMPP.to_string(),
            PropertyName::INTEREST.to_string(),
            PropertyName::KEY.to_string(),
            PropertyName::KIND.to_string(),
            PropertyName::LANG.to_string(),
            PropertyName::LOGO.to_string(),
            PropertyName::MEMBER.to_string(),
            PropertyName::NICKNAME.to_string(),
            PropertyName::NOTE.to_string(),
            PropertyName::N.to_string(),
            PropertyName::ORGDIRECTORY.to_string(),
            PropertyName::ORG.to_string(),
            PropertyName::PHOTO.to_string(),
            PropertyName::PRODID.to_string(),
            PropertyName::RELATED.to_string(),
            PropertyName::REV.to_string(),
            PropertyName::ROLE.to_string(),
            PropertyName::SOUND.to_string(),
            PropertyName::SOURCE.to_string(),
            PropertyName::TEL.to_string(),
            PropertyName::TITLE.to_string(),
            PropertyName::TZ.to_string(),
            PropertyName::UID.to_string(),
            PropertyName::URL.to_string(),
            PropertyName::XML.to_string(),
        ]
    }
}
