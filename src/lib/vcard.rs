use std::fs;

use clap::ValueEnum;
use uuid::Uuid;
use vcard_parser::{
    parse_vcards,
    traits::{HasParameters, HasValue},
    vcard::{parameter::Parameter, property::Property, Vcard},
};

use crate::{
    paths::{book_directory, contacts_directory},
    ErrorContactManager,
};
pub(crate) fn vcard_by_uuid(uuid: &Uuid) -> Result<Vcard, ErrorContactManager> {
    let path = uuid.to_string();
    Ok(parse_vcards(&fs::read_to_string(path)?)?.swap_remove(0))
}
pub(crate) fn find_first_vcard(
    content: &str,
    property_find: &PropertyType,
    parameters_find: Vec<Parameter>,
    value_find: &str,
) -> Result<Vcard, ErrorContactManager> {
    let vcards = parse_vcards(content)?;
    for vcard in vcards {
        for property in properties_by_name(&vcard, &property_find, &parameters_find) {
            let value_present = property.get_value().to_string();
            if value_present == value_find {
                return Ok(vcard);
            }
        }
    }
    Err(ErrorContactManager::Inexistant)
}
pub(crate) fn properties_by_name(
    vcard: &Vcard,
    property_search: &PropertyType,
    parameters_find: &Vec<Parameter>,
) -> Vec<Property> {
    let mut properties = Vec::new();
    match property_search {
        PropertyType::FN
        | PropertyType::UID
        | PropertyType::N
        | PropertyType::BDAY
        | PropertyType::BIRTHPLACE
        | PropertyType::REV
        | PropertyType::PRODID
        | PropertyType::DEATHDATE
        | PropertyType::DEATHPLACE
        | PropertyType::KIND
        | PropertyType::GENDER
        | PropertyType::ANNIVERSARY => {
            if let Some(property) = vcard.get_property_by_name(property_search.to_name()) {
                properties.push(property)
            }
        }
        _ => {
            let mut properties = vcard.get_properties_by_name(property_search.to_name());
            properties.retain(|pr| {
                for param in parameters_find {
                    if !pr.get_parameters().contains(&param) {
                        return false;
                    }
                }
                true
            });
        }
    }
    properties
}

pub(crate) fn vcard_fn(vcard: &Vcard) -> String {
    vcard
        .get_property_by_name("FN")
        .unwrap()
        .get_value()
        .to_string()
}
pub(crate) fn read_contacts(book_name: Option<&str>) -> Result<String, ErrorContactManager> {
    let dir = if let Some(book) = book_name {
        book_directory(book)?
    } else {
        contacts_directory()?
    };
    let files = fs::read_dir(dir)?;
    let mut all = String::new();
    for file in files {
        all.push_str(&fs::read_to_string(file?.path())?);
    }
    Ok(all)
}
#[derive(Clone, Eq, PartialEq, ValueEnum)]
pub enum PropertyType {
    ADR,
    ANNIVERSARY,
    BDAY,
    BIRTHPLACE,
    CALADRURI,
    CALURI,
    CATEGORIES,
    CLIENTPIDMAP,
    CONTACTURI,
    DEATHDATE,
    DEATHPLACE,
    EMAIL,
    EXPERTISE,
    FBURL,
    FN,
    GENDER,
    GEO,
    HOBBY,
    IMPP,
    INTEREST,
    KEY,
    KIND,
    LANG,
    LOGO,
    MEMBER,
    NICKNAME,
    NOTE,
    N,
    ORGDIRECTORY,
    ORG,
    PHOTO,
    PRODID,
    RELATED,
    REV,
    ROLE,
    SOUND,
    SOURCE,
    TEL,
    TITLE,
    TZ,
    UID,
    URL,
    XML,
}

impl PropertyType {
    pub(crate) fn to_name(&self) -> &str {
        match self {
            Self::ADR => "ADR",
            Self::ANNIVERSARY => "ANNIVERSARY",
            Self::BDAY => "BDAY",
            Self::BIRTHPLACE => "BIRTHPLACE",
            Self::CALADRURI => "CALADRURI",
            Self::CALURI => "CALURI",
            Self::CATEGORIES => "CATEGORIES",
            Self::CLIENTPIDMAP => "CLIENTPIDMAP",
            Self::CONTACTURI => "CONTACTURI",
            Self::DEATHDATE => "DEATHDATE",
            Self::DEATHPLACE => "DEATHPLACE",
            Self::EMAIL => "EMAIL",
            Self::EXPERTISE => "EXPERTISE",
            Self::FBURL => "FBURL",
            Self::FN => "FN",
            Self::GENDER => "GENDER",
            Self::GEO => "GEO",
            Self::HOBBY => "HOBBY",
            Self::IMPP => "IMPP",
            Self::INTEREST => "INTEREST",
            Self::KEY => "KEY",
            Self::KIND => "KIND",
            Self::LANG => "LANG",
            Self::LOGO => "LOGO",
            Self::MEMBER => "MEMBER",
            Self::NICKNAME => "NICKNAME",
            Self::NOTE => "NOTE",
            Self::N => "N",
            Self::ORGDIRECTORY => "ORGDIRECTORY",
            Self::ORG => "ORG",
            Self::PHOTO => "PHOTO",
            Self::PRODID => "PRODID",
            Self::RELATED => "RELATED",
            Self::REV => "REV",
            Self::ROLE => "ROLE",
            Self::SOUND => "SOUND",
            Self::SOURCE => "SOURCE",
            Self::TEL => "TEL",
            Self::TITLE => "TITLE",
            Self::TZ => "TZ",
            Self::UID => "UID",
            Self::URL => "URL",
            Self::XML => "XML",
        }
    }
}
