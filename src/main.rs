use std::fs::{self, read_to_string};
use std::path::Path;
use std::path::PathBuf;

use execute::Execute;
use regex::Regex;
use std::process::{Command, Stdio};
use vcard_parser::parse_to_vcards_without_errors;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::property::Property;
//use vcard_parser::vcard::values::Value;
use vcard_parser::vcard::Vcard;

use clap::{Parser, Subcommand, ValueEnum};

extern crate xdg;

const RG_PATH: &str = "/home/lm/.cargo/bin/rg";
const APP_SHORTNAME: &str = "cm";

#[derive(Parser)]
#[command(name = "contact-manager")]
#[command(author = "Louis-Marie Baer <lm@baermail.fr>")]
#[command(version = "0.1")]
#[command(about = "contact manager", long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    New {
        #[arg(value_name = "FN VALUE", required = true)]
        value_fn: String,
    },
    Find {
        #[arg(value_name = "SHOW FIELD", required = true, value_enum)]
        property_show: PropertyType,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },

    FindX {
        #[arg(value_name = "SHOW FIELDX", required = true, value_enum)]
        property_show: PropertyTypeX,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
    Edit {
        #[arg(value_name = "EDIT FIELD", required = true, value_enum)]
        property_edit: PropertyType,
        #[arg(value_name = "NEW VALUE", required = true)]
        property_edit_value: String,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
    Delete {
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    
    },
}
#[derive(Clone, Eq, PartialEq, ValueEnum)]
enum PropertyTypeX {
    Motpoli,
    Test,
}

fn new(path: &Path, value_fn: String) {
    let all = read_contacts(&path);
    let property_find = PropertyType::Fn;
    let value = value_fn.clone();

    if let Some(_) = find_vcard(all, property_find, value_fn) {
        println!("Le contact {} existe déjà.", value)
    } else {
        let vcard = Vcard::from_fullname(&value).unwrap();
        let data = vcard.to_string();
        let file = find_file_vcard(&path, &vcard);
        fs::write(file, data).expect("Unable to write file.");
    }
}

fn read_contacts(path: &Path) -> String {
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

fn find_file_vcard<'a>(dir_contacts: &'a Path, vcard: &'a Vcard) -> PathBuf {
    let uid = vcard.get_property_by_type(&PropertyType::Uid).unwrap();
    let file = format!("{}.vcf", uid.get_value());
    dir_contacts.join(file)
}

fn find_vcard(all: String, property_find: PropertyType, value_find: String) -> Option<Vcard> {
    let vcards = parse_to_vcards_without_errors(&all);
    for vcard in vcards {
        let properties = vcard.get_properties_by_type(&property_find);
        if !properties.is_empty() {
            let property: String;

            if property_find != PropertyType::Tel {
                property = properties[0].get_value().to_string();
            } else {
                property = properties[0]
                    .get_value()
                    .to_string()
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect();
            }

            if property == value_find {
                return Some(vcard);
            }
        }
    }
    None
}

fn property_is(vcard: &Vcard, property: PropertyType) -> Option<Property> {
    match property {
        PropertyType::Tel => {
            let properties_found = &vcard.get_properties_by_type(&property);
            if !properties_found.is_empty() {
                let form_tel: String = format!(
                    "TEL:{}",
                    properties_found[0]
                        .get_value()
                        .to_string()
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                );
                let mut vcard2 = vcard.clone();
                vcard2
                    .update_property(properties_found[0].get_uuid(), &form_tel)
                    .expect("Unable to update property.");
                let property_owned = vcard2.get_properties_by_type(&property)[0].clone();
                return Some(property_owned);
            }
        }
        _ => {
            let properties_found = vcard.get_properties_by_type(&property);
            if !properties_found.is_empty() {
                let property_owned = properties_found[0].clone();
                return Some(property_owned);
            }
        }
    }
    None
}

fn edit(
    dir_contacts: &Path,
    property_edit: PropertyType,
    property_edit_value: String,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(mut vcard) = find_vcard(all, property_find, value_find) {
        if let Some(property) = property_is(&vcard, property_edit) {
            vcard
                .update_property(property.get_uuid(), &property_edit_value)
                .expect("Unable to update property.");
        } else {
            vcard
                .add_property(&property_edit_value)
                .expect("Unable to add property.");
        }

        let file = find_file_vcard(&dir_contacts, &vcard);
        fs::write(file, vcard.to_string()).expect("Unable to write file.");
    }
}

fn delete(
    dir_contacts: &Path,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(vcard) = find_vcard(all, property_find, value_find) {
    let file = find_file_vcard(&dir_contacts, &vcard);
    fs::remove_file(file).expect("File delete failed");
    }
}

fn findx(
    dir_contacts: &Path,
    property_show: PropertyTypeX,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(dir_contacts);
    if let Some(vcard) = find_vcard(all, property_find, value_find) {
        match property_show {
            PropertyTypeX::Motpoli => {
                let path = find_file_vcard(&dir_contacts, &vcard);
                let mut command = Command::new(RG_PATH);
                command.arg("X-MOTPOLI");
                command.arg(path);
                command.stdout(Stdio::piped());
                let property_found = command.execute_output().unwrap();
                let property_string = String::from_utf8(property_found.stdout).unwrap();
                let property_regex = Regex::new(r"(?i)X-MOTPOLI:")
                    .unwrap()
                    .replace_all(&property_string, "");
                print!("{property_regex}");
            }

            _ => {
                println!("seul la propriété X supporté est X-MOTPOLI");
            }
        }
    }
}

fn find(
    dir_contacts: &Path,
    property_show: PropertyType,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(dir_contacts);
    if let Some(vcard) = find_vcard(all, property_find, value_find) {
        if let Some(property) = property_is(&vcard, property_show) {
            println!("{}", property.get_value());
        }
    }
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_SHORTNAME).unwrap();
    let mut dir_contacts = xdg_dirs.get_data_home();
    dir_contacts.push("contacts");
    xdg_dirs.create_data_directory("contacts").unwrap();

    //println!("{dir_contacts}");

    let args = Cli::parse();

    match args.command {
        Commands::Find {
            property_show,
            property_find,
            property_value,
            ..
        } => find(
            &dir_contacts.as_path(),
            property_show,
            property_find,
            property_value,
        ),
        Commands::FindX {
            property_show,
            property_find,
            property_value,
            ..
        } => findx(
            &dir_contacts.as_path(),
            property_show,
            property_find,
            property_value,
        ),
        Commands::Edit {
            property_edit,
            property_edit_value,
            property_find,
            property_value,
            ..
        } => edit(
            dir_contacts.as_path(),
            property_edit,
            property_edit_value,
            property_find,
            property_value,
        ),
        Commands::New { value_fn, .. } => new(dir_contacts.as_path(), value_fn),
        Commands::Delete { property_find, property_value, .. } => delete(dir_contacts.as_path(), property_find, property_value, ),
    };
}
