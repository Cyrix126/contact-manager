#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

/// some tools to make life easier after calling the api functions.
pub mod api_tools;
/// Right now, you can't give the api another path to search in another directory. The library use the XDG recommendations and "cm" for the app name.
mod error;
/// module to manage paths.
pub mod paths;
/// reimplement PropertyType with ValueEnum.
pub mod vcard;
/// Name of the app for XDG directories.
use std::{
    fs::{self, remove_file},
    os::unix::fs::symlink,
};

use error::ErrorContactManager;
use paths::{book_directory, books_names, path_vcard_file_and_uid, path_vcard_file_from_uuid};
use uuid::Uuid;
use vcard::{find_first_vcard, properties_by, read_contacts, vcard_by_uuid, PropertyType};
use vcard_parser::{
    parse_vcards,
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
    app_name: &str,
    book_name: Option<&str>,
    property_filter_type: &PropertyType,
    property_filter_parameters: Vec<Parameter>,
    property_filter_value: &str,
) -> Result<Uuid, ErrorContactManager> {
    let vcards_raw = read_contacts(book_name, app_name)?;
    let vcard = find_first_vcard(
        &vcards_raw,
        property_filter_type,
        property_filter_parameters,
        property_filter_value,
    )?;
    if let Some(p) = vcard.get_property_ref(&Property::PropertyUid(PropertyUidData::default())) {
        return Ok(Uuid::parse_str(&p.get_value().to_string())?);
    }
    Err(ErrorContactManager::Inexistant)
}

/// create a new address book with a name. The book will be empty.
/// Return an error if it already exists.
pub fn create_book(book_name: &str, app_name: &str) -> Result<(), ErrorContactManager> {
    let path_book = book_directory(book_name, app_name)?;
    fs::create_dir_all(&path_book)?;
    Ok(())
}
/// delete an adressbook. Return an error it doesn't exists.
/// All links in the book will be removed, but no contacts will de deleted from the folder contacts.
pub fn delete_book(book_name: &str, app_name: &str) -> Result<(), ErrorContactManager> {
    let path_book = book_directory(book_name, app_name)?;
    fs::remove_dir_all(&path_book)?;
    Ok(())
}
/// create a new contact with a fullname, will fail if contact could not be created.
/// You can't have two contacts with the same fullname.
/// Will give the uuid if the contact was successfully created.
pub fn create_contact(
    app_name: &str,
    book_name: &str,
    value_fn: &str,
) -> Result<Uuid, ErrorContactManager> {
    // load every contacts from book
    let content = read_contacts(None, app_name)?;
    // find the vcard by comparing FullName value.
    if let Ok(_) = find_first_vcard(&content, &PropertyType::FN, vec![], value_fn) {
        Err(ErrorContactManager::AlreadyExist)
    } else {
        let mut vcard = Vcard::new(value_fn);
        let uuid = Uuid::new_v4();
        let field_uuid = format!("UID:{}\n", Uuid::new_v4());
        vcard.set_property(&Property::try_from(field_uuid.as_str())?)?;
        let data = vcard.to_string();
        fs::write(path_vcard_file_and_uid(&vcard, None, app_name)?.0, data)?;
        add_to_book(app_name, book_name, &uuid)?;
        Ok(uuid)
    }
}
/// delete a contact, removing it also from any book he was.
pub fn delete_contact(uuid: Uuid, app_name: &str) -> Result<(), ErrorContactManager> {
    fs::remove_file(path_vcard_file_from_uuid(&uuid, None, app_name)?)?;
    // remove link from all books
    for book_name in books_names(app_name)? {
        let file = path_vcard_file_from_uuid(&uuid, Some(&book_name), app_name)?;
        if file.exists() {
            remove_file(file)?
        }
    }
    Ok(())
}
/// remove a contact from a book
pub fn remove_from_book(
    app_name: &str,
    book_name: &str,
    uuid: Uuid,
) -> Result<(), ErrorContactManager> {
    let file = path_vcard_file_from_uuid(&uuid, Some(book_name), app_name)?;
    if file.exists() {
        fs::remove_file(file)?;
        return Ok(());
    }
    Err(ErrorContactManager::Inexistant)
}
/// add a contact to a book
pub fn add_to_book(
    app_name: &str,
    book_name: &str,
    uuid: &Uuid,
) -> Result<(), ErrorContactManager> {
    let file_path = path_vcard_file_from_uuid(uuid, None, app_name)?;
    if file_path.exists() {
        let file = format!("{}.vcf", uuid.to_string());
        let mut file_book = book_directory(book_name, app_name)?;
        file_book.push(file);
        symlink(file_path, file_book)?;
        return Ok(());
    } else {
        Err(ErrorContactManager::Inexistant)
    }
}

/// find properties of a type of first contact found by another property type and value, filterable by book.
pub fn find_properties(
    property_show_type: &PropertyType,
    property_show_parameters: Vec<Parameter>,
    uuid: &Uuid,
) -> Result<Vec<Property>, ErrorContactManager> {
    let vcard = vcard_by_uuid(&uuid)?;
    Ok(properties_by(
        &vcard,
        property_show_type,
        &property_show_parameters,
    ))
}
/// add or replace if matches a property to first contact equal with anoter property value, filterable by book.
/// you can precise the parameters
/// if the PID match, it will replace the property.
/// This function will return the set property including the pid number to allow replacing it.
pub fn add_or_replace_property(
    app_name: &str,
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
    fs::write(
        path_vcard_file_from_uuid(&uuid, None, app_name)?,
        vcard.to_string(),
    )?;
    Ok(property_with_pid)
}
/// delete the nb value of a property of the first contact found with a property value, filterable by book.
pub fn delete_property(
    app_name: &str,
    property_delete: Property,
    uuid: &Uuid,
) -> Result<(), ErrorContactManager> {
    let mut vcard = vcard_by_uuid(&uuid)?;
    vcard.remove_property(&property_delete)?;
    fs::write(
        path_vcard_file_from_uuid(&uuid, None, app_name)?,
        vcard.to_string(),
    )?;
    Ok(())
}
/// render an index with two chosen property.
pub fn generate_index(
    app_name: &str,
    book_name: Option<&str>,
    property1: &PropertyType,
    parameters1: &Vec<Parameter>,
    property2: &PropertyType,
    parameters2: &Vec<Parameter>,
) -> Result<Vec<(String, String)>, ErrorContactManager> {
    let all = read_contacts(book_name, app_name)?;
    let vcards = parse_vcards(&all)?;
    let mut vec = vec![];
    for vcard in vcards {
        let properties1 = properties_by(&vcard, property1, parameters1);
        let properties2 = properties_by(&vcard, property2, parameters2);
        if !properties1.is_empty() && !properties2.is_empty() {
            let value1 = properties1[0].get_value().to_string();
            let value2 = properties2[0].get_value().to_string();
            vec.push((value1, value2));
        }
    }
    Ok(vec)
}
