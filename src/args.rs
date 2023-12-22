use clap::{Parser, Subcommand, ValueEnum};
#[derive(Parser)]
#[command(name = "contact-manager")]
#[command(author = "Louis-Marie Baer <lm@baermail.fr>")]
#[command(version = "0.1")]
#[command(about = "contact manager", long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(short, long)]
    pub book: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    New {
        #[arg(value_name = "FULL NAME VALUE", required = true)]
        value_fn: String,
    },
    NewBook {
        #[arg(value_name = "BOOK NAME VALUE", required = true)]
        book_value: String,
    },
    Addto {
        #[arg(value_name = "BOOK NAME VALUE", required = true)]
        book_value: String,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
    Removefrom {
        #[arg(value_name = "BOOK NAME VALUE", required = true)]
        book_value: String,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },

    Find {
        //  #[arg(value_name = "BOOK NAME")]
        //  book_name: Option<String>,
        #[arg(value_name = "SHOW FIELD", required = true, value_enum)]
        property_show: PropertyType,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },

    FindInteractive {
        #[arg(value_name = "SHOW FIELD", required = true, value_enum)]
        property_show: PropertyType,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },

    FindX {
        //   #[arg(value_name = "BOOK NAME VALUE")]
        //   book_name: String,
        #[arg(value_name = "SHOW FIELDX", required = true, value_enum)]
        property_show: String,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
    Edit {
        //   #[arg(value_name = "BOOK NAME VALUE")]
        //   book_name: String,
        #[arg(value_name = "EDIT FIELD", required = true, value_enum)]
        property_edit: PropertyType,
        #[arg(value_name = "NEW VALUE", required = true)]
        property_edit_value: String,
        #[arg(value_name = "FIND FIELD", required = true, value_enum)]
        property_find: PropertyType,
        #[arg(value_name = "VALUE", required = true)]
        property_value: String,
    },
    EditX {
        //   #[arg(value_name = "BOOK NAME VALUE")]
        //   book_name: String,
        #[arg(value_name = "EDIT FIELD", required = true, value_enum)]
        property_edit: String,
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
    DeleteBook {
        #[arg(value_name = "BOOK NAME VALUE", required = true)]
        book_value: String,
    },
    GenerateIndex {
        #[arg(value_name = "FIELD 1", required = true, value_enum)]
        property1: PropertyType,
        #[arg(value_name = "FIELD 2", required = true, value_enum)]
        property2: PropertyType,
    },
    List {
        #[arg(value_name = "FIELD 1", required = true, value_enum)]
        property: PropertyType,
    },
}

#[derive(Clone, Eq, PartialEq, ValueEnum)]
pub enum PropertyType {
    Fn,
    Tel,
    Adr,
    Email,
    NickName,
    Name,
    Uid,
    Url,
    Org,
}

impl PropertyType {
    pub fn to_name(&self) -> &str {
        match self {
            PropertyType::Fn => "FN",
            PropertyType::Tel => "TEL",
            PropertyType::Adr => "ADR",
            PropertyType::Email => "EMAIL",
            PropertyType::NickName => "NICKNAME",
            PropertyType::Name => "N",
            PropertyType::Uid => "UID",
            PropertyType::Url => "URL",
            PropertyType::Org => "ORG",
        }
    }
}
