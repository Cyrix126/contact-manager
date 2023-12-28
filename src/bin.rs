use anyhow::{Context, Result};
use args::Book;
use args::{Cli, ImmediateMode};
use clap::Parser;
use cm_lib::delete_book;
use cm_lib::delete_contacts;
use cm_lib::find_properties;
use cm_lib::find_uids;
use cm_lib::paths::books_directory;
use cm_lib::remove_from_book;
use cm_lib::{add_or_replace_property, generate_index};
use cm_lib::{add_to_book, rename_book};
use cm_lib::{create_book, import};
use cm_lib::{create_contact, export};
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::Property;
mod args;
pub const APP_SHORTNAME: &str = "cm";
fn main() -> Result<()> {
    // directory with all contacts files is contacts
    // directory for books is books
    // default book directory is default.
    // create contacts and books and default book directory if does not exist.
    let dir_books = books_directory(APP_SHORTNAME).context("ici")?;
    let mut default_book = dir_books;
    default_book.push("default");
    std::fs::create_dir_all(default_book)?;

    // parse command line arguments.
    let args = Cli::parse();

    // execute actions from arguments.
    actions(args)?;
    Ok(())
}

fn actions(cli: Cli) -> Result<()> {
    if let Some(args) = cli.immediate_mode {
        immediate_mode(args)?
    }
    Ok(())
}

fn immediate_mode(args: ImmediateMode) -> Result<()> {
    match args {
        ImmediateMode::NewBook { new_book } => Ok(create_book(&new_book, APP_SHORTNAME)?),
        ImmediateMode::RenameBook { book, new_name } => {
            Ok(rename_book(&book.name, &new_name, APP_SHORTNAME)?)
        }
        ImmediateMode::DeleteBook { book } => Ok(delete_book(&book.name, APP_SHORTNAME)?),
        ImmediateMode::CreateContact { book, value_fn } => {
            let book_default = Book::default();
            create_contact(
                APP_SHORTNAME,
                &book.as_ref().unwrap_or_else(|| &book_default).name,
                &value_fn,
            )?;
            Ok(())
        }
        ImmediateMode::DeleteContact {
            book,
            find_filters,
            lo,
        } => {
            let book_name = if let Some(b) = &book {
                Some(b.name.as_str())
            } else {
                None
            };
            Ok(delete_contacts(
                &find_uids(
                    APP_SHORTNAME,
                    book_name,
                    &find_filters.filter,
                    &lo.operator,
                    find_filters.forgive,
                )?,
                APP_SHORTNAME,
            )?)
        }
        ImmediateMode::Addto {
            book,
            find_filters,
            lo,
        } => Ok(add_to_book(
            APP_SHORTNAME,
            &book.name,
            &find_uids(
                APP_SHORTNAME,
                Some(book.name.as_str()),
                &find_filters.filter,
                &lo.operator,
                find_filters.forgive,
            )?,
        )?),
        ImmediateMode::Removefrom {
            book,
            find_filters,
            lo,
        } => Ok(remove_from_book(
            APP_SHORTNAME,
            &book.name,
            &find_uids(
                APP_SHORTNAME,
                Some(book.name.as_str()),
                &find_filters.filter,
                &lo.operator,
                find_filters.forgive,
            )?,
        )?),
        ImmediateMode::FindValue {
            book,
            find_filters,
            lo,
            show_filter,
            pretty,
        } => {
            let uid_properties = find_properties(
                APP_SHORTNAME,
                &show_filter.show,
                &find_uids(
                    APP_SHORTNAME,
                    book_name(&book).as_deref(),
                    &find_filters.filter,
                    &lo.operator,
                    find_filters.forgive,
                )
                .context("Invalid vcard content in contacts stored.")?,
                find_filters.forgive,
            )?;
            // rendu
            let len = uid_properties.len() - 1;
            if !pretty {
                for (nb, up) in uid_properties.into_iter().enumerate() {
                    println!("{}", up.0.to_string());
                    for p in up.1 {
                        let p = p.to_string().replace("\n", "");
                        println!("{p}")
                    }
                    if nb < len {
                        println!("");
                    }
                }
            } else if len == 0 {
                for p in uid_properties[0].1.iter() {
                    println!("{}", p.get_value());
                }
            } else if len > 0 {
                // afficher le full name si plusieurs contacts
                for (nb, (u, ps)) in uid_properties.into_iter().enumerate() {
                    // aller chercher full name
                    let fullname = find_properties(
                        APP_SHORTNAME,
                        &vec![Property::default("FN")],
                        &vec![u],
                        find_filters.forgive,
                    )?;
                    let fullname = fullname[0].1[0].get_value();
                    println!("{}:", fullname);
                    // TODO if some properties have the same name, show also the parameters for those.
                    for p in ps {
                        println!("{}", p.get_value());
                    }
                    if nb < len {
                        println!("");
                    }
                }
            }

            Ok(())
        }
        ImmediateMode::AddProperty {
            book,
            find_filters,
            lo,
            properties,
        } => {
            add_or_replace_property(
                APP_SHORTNAME,
                &properties.show,
                &find_uids(
                    APP_SHORTNAME,
                    book_name(&book).as_deref(),
                    &find_filters.filter,
                    &lo.operator,
                    find_filters.forgive,
                )?,
            )?;
            Ok(())
        }
        ImmediateMode::RemoveProperty {
            book,
            find_filters,
            lo,
            properties,
        } => {
            add_or_replace_property(
                APP_SHORTNAME,
                &properties.show,
                &find_uids(
                    APP_SHORTNAME,
                    book_name(&book),
                    &find_filters.filter,
                    &lo.operator,
                    find_filters.forgive,
                )?,
            )?;
            Ok(())
        }
        ImmediateMode::GenerateIndex { book, properties } => {
            let index = generate_index(
                APP_SHORTNAME,
                book_name(&book).as_deref(),
                &properties.filter,
            )?;
            println!("{}", index.join("\n"));
            Ok(())
        }
        ImmediateMode::Import {
            path_vcards_file,
            book,
        } => {
            import(&path_vcards_file, &book.unwrap_or_default(), APP_SHORTNAME)?;
            Ok(())
        }
        ImmediateMode::Export { book } => {
            println!("{}", export(book_name(&book).as_deref(), APP_SHORTNAME)?);
            Ok(())
        }
    }
}

fn book_name(book: &Option<Book>) -> Option<&str> {
    if let Some(b) = &book {
        Some(b.name.as_str())
    } else {
        None
    }
}
