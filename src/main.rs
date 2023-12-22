use std::fs::{self, read_to_string};
use std::path::Path;
use std::path::PathBuf;

use args::PropertyType;
use clap::Parser;
use interactive::properties_by_name;
use symlink::remove_symlink_file;
use symlink::symlink_file;
use uuid::Uuid;
use vcard_parser::parse_vcards;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::property_xname::PropertyXNameData;
use vcard_parser::vcard::value::value_text::ValueTextData;
use vcard_parser::vcard::value::Value;

use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;

mod args;
mod interactive;
use crate::args::{Cli, Commands};
use crate::interactive::find_interactive;

extern crate xdg;

const APP_SHORTNAME: &str = "cm";

fn new(dir_contacts: PathBuf, dir_book: PathBuf, value_fn: &str) {
    let all = read_contacts(&dir_contacts);
    let property_find = PropertyType::Fn;
    let value = value_fn.clone();

    if let Some(_) = find_one_vcard(all, &property_find, value_fn) {
        println!("Le contact {} existe déjà.", value)
    } else {
        let mut vcard = Vcard::new(value_fn);
        let uuid = format!("UID:{}\n", Uuid::new_v4());
        vcard
            .set_property(&Property::try_from(uuid.as_str()).unwrap())
            .unwrap();
        let data = vcard.to_string();
        let (file, _) = find_file_vcard(&dir_contacts, &vcard);
        fs::write(file, data).expect("Unable to write file.");
        addtobook(dir_contacts, dir_book, property_find, value_fn);
    }
}

fn newbook(path_books: PathBuf, book_name: String) {
    let mut path_book = path_books;
    path_book.push(book_name);
    fs::create_dir_all(&path_book).expect("Unable to create folder of book");
}

fn deletebook(path_books: PathBuf, book_name: String) {
    let mut path_book = path_books;
    path_book.push(book_name);
    fs::remove_dir_all(&path_book).expect("Unable to delete folder of book");
}

fn addtobook(
    dir_contacts: PathBuf,
    dir_book: PathBuf,
    property_find: PropertyType,
    value_find: &str,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        let (file_path, uid) = find_file_vcard(&dir_contacts, &vcard);
        let file = format!("{}.vcf", uid);
        let mut file_book = dir_book;
        file_book.push(file);
        symlink_file(file_path, file_book).unwrap();
    }
}

fn removefrombook(
    dir_books: PathBuf,
    book_name: String,
    property_find: PropertyType,
    value_find: String,
) {
    let mut dir_book = dir_books;
    dir_book.push(book_name);
    let dirb = dir_book.clone();
    let all = read_contacts(&dirb);
    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        let (_, uid) = find_file_vcard(&dir_book, &vcard);
        let file = format!("{}.vcf", uid);
        let mut file_book = dir_book;
        file_book.push(file);
        remove_symlink_file(file_book).unwrap();
    }
}

fn read_contacts(path: &PathBuf) -> String {
    let files = fs::read_dir(path).unwrap();
    let mut all = String::from("");
    for file in files {
        all.push_str(&format!(
            "{}",
            read_to_string(file.unwrap().path()).unwrap()
        ));
    }
    all
}

fn find_file_vcard<'a>(dir_contacts: &'a Path, vcard: &'a Vcard) -> (PathBuf, String) {
    let uid = vcard
        .get_property_by_name("UID")
        .unwrap()
        .get_value()
        .to_string();
    let file = format!("{}.vcf", uid);
    (dir_contacts.join(file), uid)
}

fn find_one_vcard(all: String, property_find: &PropertyType, value_find: &str) -> Option<Vcard> {
    let vcards = parse_vcards(&all).unwrap();
    for vcard in vcards {
        let properties = properties_by_name(&vcard, property_find);

        for property in properties {
            let mut value_present = property.get_value().to_string();
            if property_find == &PropertyType::Tel {
                value_present = value_present.chars().filter(|c| c.is_digit(10)).collect();
            }
            if &value_present == value_find {
                return Some(vcard);
            }
        }
    }
    None
}

fn property_is(vcard: &Vcard, property: &PropertyType) -> Option<Property> {
    let properties_found = properties_by_name(&vcard, property);
    if !properties_found.is_empty() {
        let property_owned = properties_found[0].clone();
        return Some(property_owned);
    }
    None
}

fn vcard_fn(vcard: &Vcard) -> String {
    vcard
        .get_property_by_name("FN")
        .unwrap()
        .get_value()
        .to_string()
}

fn edit(
    dir_contacts: PathBuf,
    property_edit: PropertyType,
    property_edit_value: &str,
    property_find: PropertyType,
    value_find: &str,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(mut vcard) = find_one_vcard(all, &property_find, &value_find) {
        let mut property = Property::default(property_edit.to_name());
        property
            .set_value(Value::from(ValueTextData::from(property_edit_value)))
            .expect("Unable to set value.");
        vcard
            .set_property(&property)
            .expect("Unable to update property.");
        // écrire le vcard sur le fichier
        let (file, _) = find_file_vcard(&dir_contacts, &vcard);
        fs::write(file, vcard.to_string()).expect("Unable to write file.");
    } else {
        panic!("ne peux pas éditer un contact inexistant !")
    }
}

fn edit_x(
    dir_contacts: PathBuf,
    property_edit: &str,
    property_edit_value: &str,
    property_find: PropertyType,
    value_find: &str,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(mut vcard) = find_one_vcard(all, &property_find, &value_find) {
        let mut property = Property::PropertyXName(PropertyXNameData::default(property_edit));
        property
            .set_value(Value::from(ValueTextData::from(property_edit_value)))
            .expect("Unable to set value.");
        vcard
            .set_property(&property)
            .expect("Unable to update property.");
        // écrire le vcard sur le fichier
        let (file, _) = find_file_vcard(&dir_contacts, &vcard);
        fs::write(file, vcard.to_string()).expect("Unable to write file.");
    } else {
        panic!("ne peux pas éditer un contact inexistant !")
    }
}
fn delete(
    dir_book: PathBuf,
    dir_contacts: PathBuf,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        let (file, _) = find_file_vcard(&dir_contacts, &vcard);
        fs::remove_file(file).expect("File delete failed");
        let (symlink, _) = find_file_vcard(&dir_book, &vcard);
        remove_symlink_file(symlink).expect("Symlink delete failed");
    }
}

fn findx(
    book_contacts: PathBuf,
    property_show: &str,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&book_contacts);
    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        let property = Property::PropertyXName(PropertyXNameData::default(property_show));
        let property_found = vcard.get_property_ref(&property);
        if let Some(property) = property_found {
            let value = property.get_value();
            print!("{value}");
        } else {
            print!("Aucune propriété de ce type.")
        }
    }
}

fn generate_index(dir_book: PathBuf, property1: &PropertyType, property2: &PropertyType) {
    let all = read_contacts(&dir_book);
    let vcards = parse_vcards(&all).unwrap();
    for vcard in vcards {
        let properties1 = properties_by_name(&vcard, &property1);
        let properties2 = properties_by_name(&vcard, &property2);
        if !properties1.is_empty() && !properties2.is_empty() {
            let property1 = properties1[0].get_value().to_string();
            let property2 = properties2[0].get_value().to_string();
            println!("{property1}\t{property2}");
        }
    }
}
fn list(dir_book: PathBuf, property: &PropertyType) {
    let all = read_contacts(&dir_book);
    let vcards = parse_vcards(&all).unwrap();
    for vcard in vcards {
        let properties = properties_by_name(&vcard, property);
        if !properties.is_empty() {
            let property = properties[0].get_value().to_string();
            println!("{property}");
        }
    }
}

fn find(
    dir_contacts: PathBuf,
    property_show: PropertyType,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&dir_contacts);

    // pas interactif, il faut un seul résultat.

    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        if let Some(property) = property_is(&vcard, &property_show) {
            println!("{}", property.get_value());
        }
    }
}

fn find_path_book(dir_books: &PathBuf, book_name: String) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(dir_books);
    path.push(book_name);
    path
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_SHORTNAME).unwrap();
    let mut dir_contacts = xdg_dirs.get_data_home().clone();
    dir_contacts.push("contacts");
    xdg_dirs.create_data_directory("contacts").unwrap();

    let mut dir_books = xdg_dirs.get_data_home().clone();
    dir_books.push("books");

    xdg_dirs.create_data_directory("books").unwrap();

    let args = Cli::parse();
    let book_default_name = String::from("default");
    let book_name: String;
    if let Some(book) = args.book.as_deref() {
        book_name = book.to_string();
    } else {
        book_name = book_default_name;
    };

    //let books = fs::read_dir(&dir_books);

    match args.command {
        Commands::Find {
            //  mut book_name,
            property_show,
            property_find,
            property_value,
            ..
        } => find(
            find_path_book(&dir_books, book_name),
            property_show,
            property_find,
            property_value,
        ),
        Commands::FindInteractive {
            property_show,
            property_find,
            property_value,
            ..
        } => find_interactive(
            find_path_book(&dir_books, book_name),
            property_show,
            property_find,
            &property_value,
        ),
        Commands::FindX {
            //  book_name,
            property_show,
            property_find,
            property_value,
            ..
        } => findx(
            find_path_book(&dir_books, book_name),
            &property_show,
            property_find,
            property_value,
        ),
        Commands::Edit {
            //   book_name,
            property_edit,
            property_edit_value,
            property_find,
            property_value,
            ..
        } => edit(
            find_path_book(&dir_books, book_name),
            property_edit,
            &property_edit_value,
            property_find,
            &property_value,
        ),
        Commands::EditX {
            //   book_name,
            property_edit,
            property_edit_value,
            property_find,
            property_value,
            ..
        } => edit_x(
            find_path_book(&dir_books, book_name),
            &property_edit,
            &property_edit_value,
            property_find,
            &property_value,
        ),
        Commands::New { value_fn, .. } => new(
            dir_contacts,
            find_path_book(&dir_books, book_name),
            &value_fn,
        ),
        Commands::Delete {
            property_find,
            property_value,
            ..
        } => delete(
            find_path_book(&dir_books, book_name),
            dir_contacts,
            property_find,
            property_value,
        ),
        Commands::NewBook { book_value, .. } => newbook(dir_books, book_value),
        Commands::DeleteBook { book_value, .. } => deletebook(dir_books, book_value),
        Commands::Addto {
            book_value,
            property_find,
            property_value,
            ..
        } => addtobook(
            dir_contacts,
            find_path_book(&dir_books, book_value),
            property_find,
            &property_value,
        ),
        Commands::Removefrom {
            book_value,
            property_find,
            property_value,
            ..
        } => removefrombook(dir_books, book_value, property_find, property_value),
        Commands::GenerateIndex {
            property1,
            property2,
            ..
        } => generate_index(
            find_path_book(&dir_books, book_name),
            &property1,
            &property2,
        ),
        Commands::List { property, .. } => list(find_path_book(&dir_books, book_name), &property),
    };
}
