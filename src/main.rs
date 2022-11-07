use std::fs::{self, read_to_string, DirEntry};
use std::io;
use std::path::Path;
use std::process::exit;

use vcard_parser::parse_to_vcards_without_errors;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::Vcard;

use clap::{Args, Parser, Subcommand};

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
    Find {
        #[arg(value_name = "SHOW FIELD", required = true, value_enum)]
        property_show: PropertyType,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
}

fn find(property_show: PropertyType, property_find: PropertyType, value_find: String) {
    let dir_contacts = Path::new("/home/lm/.contacts/default");
    let files = fs::read_dir(dir_contacts).unwrap();
    let mut all = String::from("");
    for file in files {
        all.push_str(&format!(
            "{}",
            read_to_string(file.unwrap().path()).unwrap()
        ));
    }
    let vcards = parse_to_vcards_without_errors(&all);
    for vcard in vcards {
        let properties = vcard.get_properties_by_type(&property_find);
        if !properties.is_empty() {
            let mut property = String::from("");

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
                match property_show {
                    PropertyType::Tel => {
                        let properties_found = vcard.get_properties_by_type(&property_show);
                        if !properties_found.is_empty() {
                            let property_found: String = properties_found[0]
                                .get_value()
                                .to_string()
                                .chars()
                                .filter(|c| c.is_digit(10))
                                .collect();
                            println!("{}", property_found);
                        }
                    }
                    PropertyType::Email => {
                        let properties_found = vcard.get_properties_by_type(&property_show);
                        if !properties_found.is_empty() {
                            println!("{}", properties_found[0].get_value());
                        }
                    }

                    _ => {
                        if let Some(property_found) = vcard.get_property_by_type(&property_show) {
                            println!("{}", property_found.get_value());
                        }
                    }
                }
                exit(0);
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Find {
            property_show,
            property_find,
            property_value,
            ..
        } => find(property_show, property_find, property_value),
    };
}
