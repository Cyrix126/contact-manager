use std::{fs::read_dir, path::PathBuf};

use uuid::Uuid;
use vcard_parser::{traits::HasValue, vcard::Vcard};

use crate::ErrorContactManager;

/// return the default path of books directory, creating it if it does not exist.
pub fn books_directory(app_name: &str) -> Result<PathBuf, ErrorContactManager> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(app_name)?;
    Ok(xdg_dirs.create_data_directory("books")?)
}

/// will return the names of the available books.
pub fn books_names(app_name: &str) -> Result<Vec<String>, ErrorContactManager> {
    let mut paths = Vec::new();
    let dirs = read_dir(books_directory(app_name)?)?;
    for dir in dirs {
        paths.push(
            dir?.file_name()
                .into_string()
                .expect("non utf-8 caracter on name of book"),
        )
    }
    Ok(paths)
}

/// return the default path of contacts directory, creating it if it does not exist.
pub fn contacts_directory(app_name: &str) -> Result<PathBuf, ErrorContactManager> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(app_name).unwrap();
    Ok(xdg_dirs.create_data_directory("contacts")?)
}
pub(crate) fn path_vcard_file_and_uid<'a>(
    vcard: &Vcard,
    book_name: Option<&str>,
    app_name: &str,
) -> Result<(PathBuf, String), ErrorContactManager> {
    if let Some(uid) = vcard.get_property_by_name("UID") {
        let path = if let Some(book) = book_name {
            book_directory(book, app_name)?
        } else {
            contacts_directory(app_name)?
        };
        let uid = uid.get_value().to_string();
        let file = format!("{}.vcf", uid);
        Ok((path.join(file), uid))
    } else {
        Err(ErrorContactManager::Inexistant)
    }
}

pub(crate) fn path_vcard_file_from_uuid(
    uuid: &Uuid,
    book_name: Option<&str>,
    app_name: &str,
) -> Result<PathBuf, ErrorContactManager> {
    let path = if let Some(book) = book_name {
        book_directory(book, app_name)?
    } else {
        contacts_directory(app_name)?
    };
    let uid = uuid.to_string();
    let file: String = format!("{}.vcf", uid);
    // verify if file exist.
    Ok(path.join(file))
}

pub(crate) fn book_directory(
    book_name: &str,
    app_name: &str,
) -> Result<PathBuf, ErrorContactManager> {
    let mut path_book = books_directory(app_name)?;
    path_book.push(book_name);
    Ok(path_book)
}
