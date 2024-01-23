use std::path::PathBuf;

use anyhow::Result;
use anyhow::bail;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::Args;
use clap::{Parser, Subcommand};
use contact_manager::paths::books_directory;
use contact_manager::paths::books_names;
use contact_manager::vcard::LogicalOperator;
use contact_manager::vcard_parser::vcard::property::Property;

use crate::APP_SHORTNAME;
#[derive(Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!())]
#[command(version = crate_version!())]
#[command(about = crate_description!(), long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub immediate_mode: Option<ImmediateMode>,
}

#[derive(Args)]
pub struct Book {
    #[arg(value_name = "BOOK NAME VALUE", name = "book-name", short, long, required=false, value_parser = book_name_parser)]
    pub name: String,
}

impl std::ops::Deref for Book {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl Default for Book {
    fn default() -> Self {
        Book {
            name: "default".to_string(),
        }
    }
}

#[derive(Args)]
pub struct Logic {
    #[arg(
        value_name = "LOGICAL OPERATOR",
        value_enum,
        default_value_t,
        short,
        long
    )]
    pub operator: LogicalOperator,
}
#[derive(Args)]
pub struct PropertyArg1 {
    // / example: TEL;VALUE=uri;TYPE=home
    // / the pid and value will be ignored to compare. Parameters will be used to match.
    // / for X-name, just use a name that will not be another standard name.
    #[arg(value_name = "PROPERTIES TO FILTER", 
        value_parser = convert_str_to_property, 
        required = true, short, long)]
    pub filter: Vec<Property>,
    #[arg(long)]
    pub forgive: bool,
}
#[derive(Args)]
pub struct PropertyArg2 {
    // / example: TEL;VALUE=uri;TYPE=home
    // / the pid and value will be ignored to compare. Parameters will be used to match.
    // / for X-name, just use a name that will not be another standard name.
    #[arg(value_name = "PROPERTIES TO SHOW", value_parser = convert_str_to_property, required = true, short, long)]
    pub show: Vec<Property>,
}

#[derive(Subcommand)]
pub enum ImmediateMode {
    #[command(arg_required_else_help = true)]
    NewBook {
        #[arg(value_name = "BOOK NAME VALUE",  required=true, value_parser = book_new_name_parser)]
        new_book: String
    },
    RenameBook {
        #[command(flatten)]
        book: Book,
        #[arg(value_name = "BOOK NAME VALUE", name = "new-name", short, long, required=true, value_parser = book_new_name_parser)]
        new_name: String
    },
    DeleteBook {
        #[command(flatten)]
        book: Book,
    },
    CreateContact {
        #[command(flatten)]
        book: Option<Book>,
        #[arg(value_name = "FULL NAME VALUE", required(true))]
        value_fn: Vec<String>,
    },
    DeleteContact {
        #[command(flatten)]
        book: Option<Book>,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
    },
    Addto {
        #[command(flatten)]
        book: Book,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
    },
    Removefrom {
        #[command(flatten)]
        book: Book,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
    },
    FindValue {
        #[arg(value_name = "pretty", long, short, default_value_t)]
        pretty: bool,
        #[command(flatten)]
        book: Option<Book>,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
        #[command(flatten)]
        show_filter: PropertyArg2,
    },
    AddProperty {
        #[command(flatten)]
        book: Option<Book>,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
        #[command(flatten)]
        properties: PropertyArg2,
    },
    RemoveProperty {
        #[command(flatten)]
        book: Option<Book>,
        #[command(flatten)]
        find_filters: PropertyArg1,
        #[command(flatten)]
        lo: Logic,
        #[command(flatten)]
        properties: PropertyArg2,
    },
    GenerateIndex {
        #[command(flatten)]
        book: Option<Book>,
        #[command(flatten)]
        properties: PropertyArg1,
    },
    Import {
    #[arg(value_name = "PATH OF FILE TO IMPORT")]
        path_vcards_file: PathBuf,
        #[command(flatten)]
        book: Option<Book>,
    },
    Export {
        #[command(flatten)]
        book: Option<Book>,
    }
}

fn convert_str_to_property(str: &str) -> Result<Property> {
    let mut str_eol = str.to_owned();
    str_eol.push('\n');
    Ok(Property::create_from_str(&str_eol)?)
}

fn book_name_parser(str: &str) -> Result<String> {
    let names = books_names(APP_SHORTNAME)?;
    if str == Book::default().name {
        bail!("You can't use the name for the default book (to prevent accidental deletion or renaming of the default book). If you want to search in all books, omit the --book-name argument.")
    }
    let string = str.to_string();
        if names.contains(&string) {
        return Ok(string)
    } 
    bail!("the book name \"{str}\" doesn't exist in the directory {}, you can create it with create-book.\nPresent book names:\n{}", books_directory(APP_SHORTNAME)?.display(), names.join("\n"))
}

pub fn book_new_name_parser(str: &str) -> Result<String> {
    let names = books_names(APP_SHORTNAME)?;
    let string = str.to_string();
        if !names.contains(&string) {
        return Ok(string)
    } 
    bail!("the book name \"{str}\" already exist in the directory {}, you must precise a non existent name.\nPresent book names:\n{}", books_directory(APP_SHORTNAME)?.display(), names.join("\n"))
}
