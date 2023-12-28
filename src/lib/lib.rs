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
    path::Path,
};

use error::{error_vcard_descriptive, ErrorContactManager};
use paths::{book_directory, books_names, path_vcard_file_and_uid, path_vcard_file_from_uuid};
use uuid::Uuid;
use vcard::{
    filter_vcards_by_properties, properties_show_from_vcards, read_contacts, vcard_uuid,
    vcards_by_uuid, LogicalOperator,
};
use vcard_parser::{
    parse_vcards,
    traits::HasValue,
    vcard::{
        property::{property_fn::PropertyFnData, property_uid::PropertyUidData, Property},
        value::{value_text::ValueTextData, Value},
        Vcard,
    },
};

use crate::paths::books_directory;
/// get the vcards from filters properties with operator logic and from book or all.
pub fn find_uids(
    app_name: &str,
    book_name: Option<&str>,
    filter_properties: &Vec<Property>,
    lo: &LogicalOperator,
    forgive: bool,
) -> Result<Vec<Uuid>, ErrorContactManager> {
    let mut uuids = vec![];
    let vcards_all = read_contacts(book_name, app_name)?;
    let vcards = filter_vcards_by_properties(&vcards_all, filter_properties, forgive, lo)?;
    let property_uuid = Property::PropertyUid(PropertyUidData::default());
    for vcard in vcards {
        uuids.push(
            Uuid::parse_str(
                &vcard
                    .get_property_ref(&property_uuid)
                    .expect(&error_vcard_descriptive(&vcard))
                    .get_value()
                    .to_string(),
            )?, // .expect(&error_vcard_descriptive(&vcard)),
        )
    }
    Ok(uuids)
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
/// rename a book. All contacts in the new book will be preserved.
pub fn rename_book(
    book_name: &str,
    book_new_name: &str,
    app_name: &str,
) -> Result<(), ErrorContactManager> {
    let path_book = book_directory(book_name, app_name)?;
    let mut path_new = books_directory(app_name)?;
    path_new.push(book_new_name);
    fs::rename(path_book, path_new)?;
    Ok(())
}
/// create a new contact with a fullname, will fail if contact could not be created.
/// You can't have two contacts with the same fullname.
/// Will give the uuid if the contact was successfully created.
pub fn create_contact(
    app_name: &str,
    book_name: &str,
    values_fn: &Vec<String>,
) -> Result<Vec<Uuid>, ErrorContactManager> {
    // load every contacts from book
    let vcards = read_contacts(None, app_name)?;
    // find the vcard by comparing FullName value.
    let mut uuids = Vec::new();
    for value_fn in values_fn {
        let mut fn_property = Property::PropertyFn(PropertyFnData::default());
        fn_property.set_value(Value::ValueText(ValueTextData {
            value: value_fn.to_owned(),
        }))?;

        if filter_vcards_by_properties(&vcards, &vec![fn_property], false, &LogicalOperator::Or)?
            .is_empty()
        {
            let uuid = Uuid::new_v4();
            let mut property_uuid = Property::PropertyUid(PropertyUidData::default());
            property_uuid.set_value(Value::ValueUri(
                vcard_parser::vcard::value::value_uri::ValueUriData {
                    value: uuid.to_string(),
                },
            ))?;
            let mut vcard = Vcard::new(&value_fn);
            vcard.set_property(&property_uuid)?;
            let data = vcard.to_string();
            fs::write(path_vcard_file_and_uid(&vcard, None, app_name)?.0, data)?;
            uuids.push(uuid);
            add_to_book(app_name, book_name, &vec![uuid])?;
        } else {
            return Err(ErrorContactManager::AlreadyExist);
        }
    }
    Ok(uuids)
}
/// delete a contact, removing it also from any book he was.
pub fn delete_contacts(uuids: &Vec<Uuid>, app_name: &str) -> Result<(), ErrorContactManager> {
    for uuid in uuids {
        fs::remove_file(path_vcard_file_from_uuid(&uuid, None, app_name)?)?;
        for book_name in books_names(app_name)? {
            let file = path_vcard_file_from_uuid(&uuid, Some(&book_name), app_name)?;
            if file.exists() {
                remove_file(file)?
            }
        }
    }
    // remove link from all books
    Ok(())
}
/// remove a contact from a book
pub fn remove_from_book(
    app_name: &str,
    book_name: &str,
    uuids: &Vec<Uuid>,
) -> Result<(), ErrorContactManager> {
    for uuid in uuids {
        let file = path_vcard_file_from_uuid(&uuid, Some(book_name), app_name)?;
        if file.exists() {
            fs::remove_file(file)?;
        }
    }
    Ok(())
}
/// add a contact to a book
pub fn add_to_book(
    app_name: &str,
    book_name: &str,
    uuids: &Vec<Uuid>,
) -> Result<(), ErrorContactManager> {
    for uuid in uuids {
        let file_path = path_vcard_file_from_uuid(uuid, None, app_name)?;
        if file_path.exists() {
            let file = format!("{}.vcf", uuid.to_string());
            let mut file_book = book_directory(book_name, app_name)?;
            file_book.push(file);
            symlink(file_path, file_book)?;
        } else {
            return Err(ErrorContactManager::Inexistant);
        }
    }
    return Ok(());
}

/// find some properties of vcards, filterable by book.
pub fn find_properties(
    app_name: &str,
    properties_show: &Vec<Property>,
    uuids: &Vec<Uuid>,
    forgive: bool,
) -> Result<Vec<(Uuid, Vec<Property>)>, ErrorContactManager> {
    let mut vcards = vcards_by_uuid(uuids, app_name)?;
    Ok(properties_show_from_vcards(
        &mut vcards,
        properties_show,
        forgive,
    )?)
}

/// add or replace if matches a property to first contact equal with anoter property value, filterable by book.
/// you can precise the parameters
/// if the PID match, it will replace the property.
/// This function will return the set property including the pid number to allow replacing it.
pub fn add_or_replace_property(
    app_name: &str,
    properties_add: &Vec<Property>,
    uuids: &Vec<Uuid>,
) -> Result<Vec<(Uuid, Vec<Property>)>, ErrorContactManager> {
    let mut vcards = vcards_by_uuid(uuids, app_name)?;
    let mut properties_id = vec![];
    for vcard in &mut vcards {
        let mut properties = vec![];
        for p in properties_add {
            properties.push(vcard.set_property(p)?)
        }
        let uuid = vcard_uuid(&vcard)?;
        properties_id.push((uuid, properties));
        fs::write(
            path_vcard_file_from_uuid(&uuid, None, app_name)?,
            vcard.to_string(),
        )?;
    }
    Ok(properties_id)
}
/// delete properties for every contacts matched with uuids.
pub fn delete_properties(
    app_name: &str,
    property_delete: &Vec<Property>,
    uuids: &Vec<Uuid>,
) -> Result<(), ErrorContactManager> {
    let mut vcards = vcards_by_uuid(&uuids, app_name)?;
    for vcard in &mut vcards {
        for p in property_delete {
            vcard.remove_property(p)?;
        }
        let uuid = vcard_uuid(&vcard)?;
        fs::write(
            path_vcard_file_from_uuid(&uuid, None, app_name)?,
            vcard.to_string(),
        )?;
    }
    Ok(())
}
/// render an index with the chosen properties. Will only render a contact line if every property exist.
pub fn generate_index(
    app_name: &str,
    book_name: Option<&str>,
    properties: &Vec<Property>,
) -> Result<Vec<String>, ErrorContactManager> {
    let vcards = read_contacts(book_name, app_name)?;
    let uuids = properties_show_from_vcards(&vcards, &properties, false)?;
    let mut index = vec![];

    for uuid in uuids {
        let mut line = vec![];
        if uuid.1.len() == properties.len() {
            for p in uuid.1 {
                line.push(p.get_value().to_string());
            }
        }
        index.push(line.join("\t").to_string());
    }
    Ok(index)
}

/// export to a string all contacts of a book or of all books if book name not given
pub fn export(book_name: Option<&str>, app_name: &str) -> Result<String, ErrorContactManager> {
    let contacts = read_contacts(book_name, app_name)?;
    let mut all = String::new();
    for c in contacts {
        all.push_str(&c.to_string());
    }
    Ok(all)
}

/// import all vcards from a file into a book name.
/// if a contact is invalid, the import will be canceled.
/// If no valid uid is discovered for each contact, it will be created.
pub fn import(path: &Path, book_name: &str, app_name: &str) -> Result<(), ErrorContactManager> {
    if path.is_dir() {
        return Err(ErrorContactManager::ImportError);
    }
    let path = if path.is_relative() {
        let mut current_dir = std::env::current_dir()?;
        current_dir.push(&path);
        current_dir
    } else {
        path.to_owned()
    };
    // parse vcards
    let mut contacts = parse_vcards(&fs::read_to_string(&path)?)?;
    let mut uuids = Vec::new();
    for mut c in &mut contacts {
        // verify that uuid is present and valid
        let uuid = match c.get_property_ref(&Property::PropertyUid(PropertyUidData::default())) {
            Some(p) => match Uuid::try_parse(&p.get_value().to_string()) {
                Ok(uuid) => uuid,
                Err(_) => set_new_uuid(&mut c)?,
            },
            None => set_new_uuid(&mut c)?,
        };
        fs::write(
            path_vcard_file_from_uuid(&uuid, None, app_name)?,
            c.to_string(),
        )?;
        uuids.push(uuid);
    }
    add_to_book(app_name, book_name, &uuids)
}

fn set_new_uuid(vcard: &mut Vcard) -> Result<Uuid, ErrorContactManager> {
    let mut property_uuid = Property::PropertyUid(PropertyUidData::default());
    let uuid = Uuid::new_v4();
    property_uuid.set_value(Value::ValueText(ValueTextData {
        value: uuid.to_string(),
    }))?;
    vcard.set_property(&property_uuid)?;
    Ok(uuid)
}
