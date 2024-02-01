use std::{fs, path::PathBuf};

use uuid::Uuid;
use vcard_parser::{
    constants::PropertyName,
    parse::property::property,
    parse_vcards,
    traits::{HasName, HasValue},
    vcard::{
        parameter::Parameter,
        property::{property_uid::PropertyUidData, Property},
        Vcard,
    },
};

use crate::{
    paths::{book_directory, contacts_directory},
    ErrorContactManager,
};
/// Property that the user should not have write access to for simplicity.
pub const PROPERTY_NO_MODIFICATION_BY_USER: [&str; 4] = ["REV", "UID", "BEGIN", "END"];

/// finding vcards by uuids, read directly the file with the uuid name instead of parsing every contacts like read_contacts.
pub(crate) fn vcards_by_uuid(
    uuids: &Vec<Uuid>,
    app_name: &str,
) -> Result<Vec<Vcard>, ErrorContactManager> {
    let mut paths = Vec::new();
    let path_contacts = contacts_directory(app_name)?;
    for uuid in uuids {
        let mut path_contact = path_contacts.to_owned();
        path_contact.push(&format!("{}.vcf", uuid.to_string()));
        paths.push(path_contact);
    }
    check_validity_vcards(&paths)
}

pub(crate) fn filter_vcards_by_properties(
    vcards: &Vec<Vcard>,
    property_filter: &Vec<Property>,
    forgive: bool,
    lo: &LogicalOperator,
) -> Result<Vec<Vcard>, ErrorContactManager> {
    let mut vcards_output = vec![];
    match lo {
        LogicalOperator::And => {
            for vcard in vcards {
                for f in property_filter {
                    if present_property(f, vcard, forgive)?.is_empty() {
                        break;
                    }
                }
                vcards_output.push(vcard.to_owned());
            }
        }
        LogicalOperator::Or => {
            for vcard in vcards {
                for f in property_filter {
                    if !present_property(f, vcard, forgive)?.is_empty() {
                        vcards_output.push(vcard.to_owned());
                        break;
                    }
                }
            }
        }
        LogicalOperator::Not => {
            for vcard in vcards {
                for f in property_filter {
                    if !present_property(f, vcard, forgive)?.is_empty() {
                        break;
                    }
                }
                vcards_output.push(vcard.to_owned())
            }
        }
        LogicalOperator::Xor => {
            for vcard in vcards {
                let mut xor = false;
                for f in property_filter {
                    if !present_property(f, vcard, forgive)?.is_empty() {
                        if xor {
                            vcards_output.pop();
                            break;
                        }
                        xor = true;
                        vcards_output.push(vcard.to_owned());
                    }
                }
            }
        }
    }
    Ok(vcards_output)
}

pub(crate) fn properties_show_from_vcards(
    vcards: &Vec<Vcard>,
    show_properties: &Vec<Property>,
    forgive: bool,
) -> Result<Vec<(Uuid, Vec<Property>)>, ErrorContactManager> {
    let mut properties = Vec::new();
    for vcard in vcards {
        let mut properties_matched = Vec::new();
        for s in show_properties {
            properties_matched.extend(present_property(s, vcard, forgive)?);
        }
        if !properties_matched.is_empty() {
            properties.push((vcard_uuid(&vcard)?, properties_matched));
        }
    }
    Ok(properties)
}

pub(crate) fn vcard_uuid(vcard: &Vcard) -> Result<Uuid, ErrorContactManager> {
    Ok(Uuid::parse_str(
        &vcard
            .get_property_ref(&Property::PropertyUid(PropertyUidData::default()))
            .expect("this vcard don't have a UID. Can't work without a uid.")
            .get_value()
            .to_string(),
    )?)
}

pub(crate) fn read_contacts(
    book_name: Option<&str>,
    app_name: &str,
) -> Result<Vec<Vcard>, ErrorContactManager> {
    let dir = path_vcards(app_name, book_name)?;
    let files = fs::read_dir(&dir)?;
    let mut paths = Vec::new();
    for file in files {
        let p = file?.path();
        paths.push(p)
    }
    check_validity_vcards(&paths)
}
fn check_validity_vcards(paths: &Vec<PathBuf>) -> Result<Vec<Vcard>, ErrorContactManager> {
    let mut all = String::new();
    for p in paths {
        match &fs::read_to_string(&p) {
            Ok(s) => all.push_str(s),
            Err(e) => {
                eprintln!(
                    "maybe invalid link if contact was deleted manually ? {:?}, {:?}",
                    p, e
                )
            }
        };
    }

    match parse_vcards(&all) {
        Ok(vcards) => return Ok(vcards),
        Err(_) => {
            for p in paths {
                let string = &fs::read_to_string(&p)?;
                match parse_vcards(&string) {
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!(
                            "the following vcard at path {}\n is not valid:\n{}",
                            p.display(),
                            string
                        );
                        return Err(ErrorContactManager::VcardError(e));
                    }
                }
            }
            return Ok(vec![]);
        }
    };
}
#[cfg(feature = "clap")]
use clap::ValueEnum;
/// Logic Operator.
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[derive(Clone, Default, Debug)]
pub enum LogicalOperator {
    /// Property AND Property must be present
    #[default]
    Or,
    /// Property OR Property must be present
    And,
    /// Property must NOT be present.
    Not,
    /// One of the Properties must only be present.
    Xor,
}

fn get_params_from_property(pr: &Property) -> Result<Vec<Parameter>, ErrorContactManager> {
    let raw = pr.export();
    let params_raw = property(raw.as_bytes())
        .expect("Property syntax is not valid")
        .1
         .1;

    let mut params = Vec::new();
    for param_raw in params_raw {
        params.push(Parameter::try_from(param_raw)?)
    }
    Ok(params)
}

pub(crate) fn present_property(
    s: &Property,
    vcard: &Vcard,
    forgive: bool,
) -> Result<Vec<Property>, ErrorContactManager> {
    // verify that the type of Property matches
    let mut properties_matched = Vec::new();
    for v in vcard.get_properties().iter() {
        if property_match(s, v, forgive)? {
            properties_matched.push(v.to_owned());
        }
    }
    Ok(properties_matched)
}

/// fonction tool to see if a property match another, comparing the name of property, parameters and value.
pub fn property_match(
    a: &Property,
    b: &Property,
    forgive: bool,
) -> Result<bool, ErrorContactManager> {
    if a.name() != b.name() {
        return Ok(false);
    }
    let parameters_show = get_params_from_property(&a)?;
    let parameters_vcard = get_params_from_property(&b)?;
    // do not compare parameters if Property show doesn't have any.
    if !parameters_show.is_empty() {
        // for each parameter, verify that Property Vcard contains every one of them. Il will check if the value and name of the parameter is present.
        for pas in parameters_show.iter() {
            if !parameters_vcard.contains(&pas) {
                return Ok(false);
            }
        }
    }

    // if a value of Property has been given, compare it with the one of vcard.
    // remove separators if any.
    if !a
        .get_value()
        .to_string()
        .replace(",", "")
        .replace(";", "")
        .is_empty()
    {
        if forgive {
            if !b
                .get_value()
                .to_string()
                .contains(&a.get_value().to_string())
            {
                return Ok(false);
            }
        } else {
            if a.get_value() != b.get_value() {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

pub(crate) fn path_vcards(
    app_name: &str,
    book_name: Option<&str>,
) -> Result<PathBuf, ErrorContactManager> {
    if let Some(book) = book_name {
        book_directory(book, app_name)
    } else {
        contacts_directory(app_name)
    }
}

/// find uuids of vcards
pub fn uuids_from_vcards(vcards: &Vec<&Vcard>) -> Result<Vec<Uuid>, ErrorContactManager> {
    let mut uuids = vec![];
    for v in vcards.iter() {
        if let Some(p) = v.get_property_by_name(PropertyName::UID) {
            uuids.push(Uuid::parse_str(&p.get_value().to_string())?);
        } else {
            return Err(ErrorContactManager::UuidInexistant(v.to_owned().to_owned()));
        }
        //todo remove this ugly twice to_owned
    }
    Ok(uuids)
}
