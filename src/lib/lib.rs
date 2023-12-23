#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

/// some tools to make life easier after calling the api functions.
pub mod api_tools;
/// Right now, you can't give the api another path to search in another directory. The library use the XDG recommendations and "cm" for the app name.
mod error;
mod paths;
mod vcard;
/// Name of the app for XDG directories.
pub const APP_SHORTNAME: &str = "cm";
use std::{
    fs::{self, remove_file},
    os::unix::fs::symlink,
};

use error::ErrorContactManager;
use paths::{book_directory, books_names, path_vcard_file_and_uid, path_vcard_file_from_uuid};
use uuid::Uuid;
use vcard::{find_first_vcard, properties_by_name, read_contacts, vcard_by_uuid, PropertyType};
use vcard_parser::{
    traits::{HasParameters, HasValue},
    vcard::{
        parameter::Parameter,
        property::{property_uid::PropertyUidData, Property},
        value::{value_text::ValueTextData, Value},
        Vcard,
    },
};

/// first function to use to access a contact.
/// The Uuid will be needed for the ohter actions.
pub fn find_uid(
    book_name: Option<&str>,
    property_find: &PropertyType,
    parameters_find: Vec<Parameter>,
    value_find: &str,
) -> Result<Uuid, ErrorContactManager> {
    let vcards_raw = read_contacts(book_name)?;
    let vcard = find_first_vcard(&vcards_raw, &property_find, parameters_find, &value_find)?;
    if let Some(p) = vcard.get_property_ref(&Property::PropertyUid(PropertyUidData::default())) {
        return Ok(Uuid::parse_str(&p.get_value().to_string())?);
    }
    Err(ErrorContactManager::Inexistant)
}

/// create a new address book with a name. The book will be empty.
/// Return an error if it already exists.
pub fn create_book(book_name: &str) -> Result<(), ErrorContactManager> {
    let path_book = book_directory(book_name)?;
    fs::create_dir_all(&path_book)?;
    Ok(())
}
/// delete an adressbook. Return an error it doesn't exists.
/// All links in the book will be removed, but no contacts will de deleted from the folder contacts.
pub fn delete_book(book_name: &str) -> Result<(), ErrorContactManager> {
    let path_book = book_directory(book_name)?;
    fs::remove_dir_all(&path_book)?;
    Ok(())
}
/// create a new contact with a fullname, will fail if contact could not be created.
/// You can't have two contacts with the same fullname.
/// Will give the uuid if the contact was successfully created.
pub fn create_contact(book_name: &str, value_fn: &str) -> Result<Uuid, ErrorContactManager> {
    // load every contacts from book
    let content = read_contacts(None)?;
    // find the vcard by comparing FullName value.
    let property_find = PropertyType::FN;
    if let Ok(_) = find_first_vcard(&content, &property_find, vec![], value_fn) {
        Err(ErrorContactManager::AlreadyExist)
    } else {
        let mut vcard = Vcard::new(value_fn);
        let uuid = Uuid::new_v4();
        let field_uuid = format!("UID:{}\n", Uuid::new_v4());
        vcard.set_property(&Property::try_from(field_uuid.as_str())?)?;
        let data = vcard.to_string();
        fs::write(path_vcard_file_and_uid(&vcard, None)?.0, data)?;
        add_to_book(book_name, &uuid)?;
        Ok(uuid)
    }
}
/// delete a contact, removing it also from any book he was.
pub fn delete_contact(uuid: Uuid) -> Result<(), ErrorContactManager> {
    fs::remove_file(path_vcard_file_from_uuid(&uuid, None)?)?;
    // remove link from all books
    for book_name in books_names()? {
        let file = path_vcard_file_from_uuid(&uuid, Some(&book_name))?;
        if file.exists() {
            remove_file(file)?
        }
    }
    Ok(())
}
/// remove a contact from a book
pub fn remove_from_book(book_name: &str, uuid: Uuid) -> Result<(), ErrorContactManager> {
    let file = path_vcard_file_from_uuid(&uuid, Some(book_name))?;
    if file.exists() {
        fs::remove_file(file)?;
        return Ok(());
    }
    Err(ErrorContactManager::Inexistant)
}
/// add a contact to a book
pub fn add_to_book(book_name: &str, uuid: &Uuid) -> Result<(), ErrorContactManager> {
    let file_path = path_vcard_file_from_uuid(uuid, None)?;
    if file_path.exists() {
        let file = format!("{}.vcf", uuid.to_string());
        let mut file_book = book_directory(book_name)?;
        file_book.push(file);
        symlink(file_path, file_book)?;
        return Ok(());
    } else {
        Err(ErrorContactManager::Inexistant)
    }
}

/// find properties of a type of first contact found by another property type and value, filterable by book.
pub fn find_properties(
    property_show: &PropertyType,
    parameters_show: Vec<Parameter>,
    uuid: &Uuid,
) -> Result<Vec<Property>, ErrorContactManager> {
    let vcard = vcard_by_uuid(&uuid)?;
    Ok(properties_by_name(&vcard, property_show, &parameters_show))
}
/// add or replace if matches a property to first contact equal with anoter property value, filterable by book.
/// you can precise the parameters
/// if the PID match, it will replace the property.
/// This function will return the set property including the pid number to allow replacing it.
pub fn add_or_replace_property(
    property_add: PropertyType,
    parameters: Vec<Parameter>,
    property_add_value: &str,
    uuid: &Uuid,
) -> Result<Property, ErrorContactManager> {
    let mut vcard = vcard_by_uuid(&uuid)?;
    let mut property = Property::default(property_add.to_name());
    property.set_parameters(parameters);
    property.set_value(Value::from(ValueTextData::from(property_add_value)))?;
    let property_with_pid = vcard.set_property(&property)?;
    fs::write(path_vcard_file_from_uuid(&uuid, None)?, vcard.to_string())?;
    Ok(property_with_pid)
}
/// delete the nb value of a property of the first contact found with a property value, filterable by book.
pub fn delete_property(property_delete: Property, uuid: &Uuid) -> Result<(), ErrorContactManager> {
    let mut vcard = vcard_by_uuid(&uuid)?;
    vcard.remove_property(&property_delete)?;
    fs::write(path_vcard_file_from_uuid(&uuid, None)?, vcard.to_string())?;
    Ok(())
}
