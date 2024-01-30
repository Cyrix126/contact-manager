use anyhow::Result;
use contact_manager_lib::{
    api_tools::cardinals,
    paths::{books_directory, books_names},
    vcard_parser::{
        traits::{HasParameters, HasValue},
        vcard::{property::Property, Vcard},
    },
};
use inquire::validator::Validation;

use crate::APP_SHORTNAME;

pub mod book;
pub mod contact;
pub mod custom_input;
pub mod display;
pub mod menu;

fn properties_to_add(vcard: &Vcard) -> Vec<Property> {
    let vcard_properties = vcard.get_properties();
    let mut properties_to_add = cardinals().0;
    let properties_multiples = cardinals().1;
    for vp in vcard_properties.iter() {
        properties_to_add.retain(|p| p != vp)
    }
    properties_to_add.extend(properties_multiples);
    properties_to_add
}

pub fn validator_param(raw: String) -> Result<Validation> {
    if Property::try_from(raw.as_str()).is_ok() {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(
            "Value is not correct for this Property".into(),
        ))
    }
}
pub fn validator_new_bookname(raw: &str) -> Result<Validation> {
    let names = books_names(APP_SHORTNAME)?;
    let string = raw.to_string();
    if !names.contains(&string) {
        return Ok(Validation::Valid);
    }
    Ok(Validation::Invalid(
         format!("the book name \"{raw}\" already exist in the directory {}, you must precise a non existent name.\n\nPresent book names:\n{}", books_directory(APP_SHORTNAME)?.display(), names.join("\n")).into()))
}

fn show_property(p: &Property) {
    println!("Actual Value: {}", p.get_value());
    for p in p.get_parameters() {
        println!("{}", p.get_value());
    }
}

// menu book
