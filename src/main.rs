use std::fs::{self, read_to_string, DirEntry};
use std::io;
use std::path::Path;
use std::process::exit;
use std::path::PathBuf;

use vcard_parser::parse_to_vcards_without_errors;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::Vcard;

use regex::Regex;
use std::process::{Command, Stdio};
use execute::Execute;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
}
#[derive(Clone, Eq, PartialEq, ValueEnum)]
enum PropertyTypeX {
	Motpoli,
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



fn findx(dir_contacts: &Path, property_show: PropertyTypeX, property_find: PropertyType, value_find: String) {
let all = read_contacts(dir_contacts);

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
                    PropertyTypeX::Motpoli => {
                        let uid = vcard.get_property_by_type(&PropertyType::Uid).unwrap();
                        let file = format!("{}.vcf", uid.get_value());
                        let path = dir_contacts.join(file);
                        let mut command = Command::new(RG_PATH);
                        command.arg("X-MOTPOLI");
                        command.arg(path);
                        command.stdout(Stdio::piped());
                        let property_found = command.execute_output().unwrap();
                        let property_string = String::from_utf8(property_found.stdout).unwrap();
                        let property_regex = Regex::new(r"(?i)X-MOTPOLI:").unwrap().replace_all(&property_string, "");
                            print!("{property_regex}");
                            
                    }

                    _ => {
                            println!("seul la propriété X supporté est X-MOTPOLI");
                            exit(0);
                        }
                    }
                }
                
            }
            
            
        }
        
        }



fn find(dir_contacts: &Path, property_show: PropertyType, property_find: PropertyType, value_find: String) {
	let all = read_contacts(dir_contacts);

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
                    PropertyType::Email | PropertyType::Adr | PropertyType::NickName => {
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

let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_SHORTNAME).unwrap();
let mut dir_contacts  = xdg_dirs.get_data_home();
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
        } => find(dir_contacts.as_path(), property_show, property_find, property_value),
	Commands::FindX {
	    property_show,
            property_find,
            property_value,
	    ..
	} => findx(dir_contacts.as_path(), property_show, property_find, property_value),	    
};
    }
