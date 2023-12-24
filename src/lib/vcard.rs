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
    property_filter_type: &PropertyType,
    property_filter_parameters: Vec<Parameter>,
    property_filter_value: &str,
) -> Result<Vcard, ErrorContactManager> {
    let vcards = parse_vcards(content)?;
    for vcard in vcards {
        for property in properties_by(&vcard, &property_filter_type, &property_filter_parameters) {
            let value_present = property.get_value().to_string();
            if value_present == property_filter_value {
                return Ok(vcard);
            }
        }
    }
    Err(ErrorContactManager::Inexistant)
}
pub(crate) fn properties_by(
    vcard: &Vcard,
    property_show_type: &PropertyType,
    property_show_parameters: &Vec<Parameter>,
) -> Vec<Property> {
    let mut properties = Vec::new();
    let name_property = property_show_type.to_name();
    match property_show_type {
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
            if let Some(property) = vcard.get_property_by_name(name_property) {
                properties.push(property)
            }
        }
        _ => {
            let mut properties = vcard.get_properties_by_name(name_property);
            properties.retain(|pr| {
                for param in property_show_parameters {
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
pub(crate) fn read_contacts(
    book_name: Option<&str>,
    app_name: &str,
) -> Result<String, ErrorContactManager> {
    let dir = if let Some(book) = book_name {
        book_directory(book, app_name)?
    } else {
        contacts_directory(app_name)?
    };
    let files = fs::read_dir(dir)?;
    let mut all = String::new();
    for file in files {
        all.push_str(&fs::read_to_string(file?.path())?);
    }
    Ok(all)
}

/// reimplementation of PropertyType with a ValueEnum to integrated in clap.
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
