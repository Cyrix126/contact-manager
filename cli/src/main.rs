use anyhow::{bail, Context, Result};
use args::Book;
use args::{Cli, ImmediateMode};
use clap::Parser;
use contact_manager_lib::{
    add_or_replace_property, add_to_book, create_book, create_contact, delete_book,
    delete_contacts, export, find_properties, find_uids, generate_index, import,
    paths::books_directory,
    remove_from_book, rename_book,
    vcard_parser::{traits::HasValue, vcard::property::Property},
};
#[cfg(feature = "interact")]
use interactive::book::ShortCutArgBook;
use promptable::basics::display::clear_screen;
use promptable::basics::promptable::Promptable;
use promptable::inspect::Inspectable;
mod args;
#[cfg(feature = "interact")]
mod interactive;
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
        immediate_mode(args)?;
    } else if !cfg!(feature = "interact") {
        bail!("no argument given and the binary has not been build whith the feature \"interact\"");
    } else {
        #[cfg(feature = "interact")]
        interact_mode()?;
    }
    Ok(())
}

#[cfg(feature = "interact")]
fn shortcut_book(s: ShortCutArgBook, book_name: String) -> Result<()> {
    use clap_shortcuts::ShortCuts;
    use contact_manager_lib::vcards_from_book;
    use interactive::book::Book as PromptBook;
    use interactive::contact::{Contact, VecContact, WrapperVcard};
    use promptable::termion::screen::{ToAlternateScreen, ToMainScreen};
    let mut book = PromptBook {
        contacts: VecContact(
            vcards_from_book(APP_SHORTNAME, Some(&book_name))?
                .into_iter()
                .map(|v| Contact {
                    vcard: WrapperVcard(v),
                })
                .collect(),
        ),
        name: book_name,
    };
    print!("{}", ToAlternateScreen);
    clear_screen();
    if let Some(s) = s.shortcut_mut {
        book.shortcut_mut(&s, ())?;
    } else if let Some(s) = s.shortcut_ref {
        book.shortcut_ref(&s, ())?;
    }
    print!("{}", ToMainScreen);
    Ok(())
}

#[cfg(feature = "interact")]
fn interact_mode() -> Result<()> {
    use promptable::termion::screen::{ToAlternateScreen, ToMainScreen};
    print!("{}", ToAlternateScreen);
    clear_screen();
    use crate::interactive::book::VecBook;
    use contact_manager_lib::paths::books_names;
    use contact_manager_lib::vcards_from_book;
    use inquire::Select;
    use interactive::book::Book as PromptBook;
    use interactive::contact::{Contact, VecContact, WrapperVcard};
    // get books structs

    let options = vec!["Manage", "Inspect", "Quit"];
    let mut books = VecBook(Vec::new());
    for book in books_names(APP_SHORTNAME)? {
        books.push(PromptBook {
            contacts: VecContact(
                vcards_from_book(APP_SHORTNAME, Some(&book))?
                    .into_iter()
                    .map(|v| Contact {
                        vcard: WrapperVcard(v),
                    })
                    .collect(),
            ),
            name: book,
        });
    }
    loop {
        // clear_screen();
        if let Some(choice) = Select::new("Contact-Manager\n", options.clone())
            .without_filtering()
            .prompt_skippable()?
        {
            match choice {
                "Manage" => {
                    books.modify_by_prompt(())?;
                }
                "Inspect" => VecBook::inspect_menu(&books)?,
                _ => break,
            }
        }
    }
    print!("{}", ToMainScreen);
    Ok(())
    // option to modify book and contacts. Adding a client suggest to add from another book or to create one.
    // deleting a contact from a book will delete it completly if it is not in any other books.
    // option to inspect book.
    // manage contacts and books
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
                &book.as_ref().unwrap_or(&book_default).name,
                &value_fn,
            )?;
            Ok(())
        }
        ImmediateMode::DeleteContact {
            book,
            find_filters,
            lo,
        } => {
            let book_name = book.as_ref().map(|b| b.name.as_str());
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
                    book_name(&book),
                    &find_filters.filter,
                    &lo.operator,
                    find_filters.forgive,
                )
                .context("Invalid vcard content in contacts stored.")?,
                find_filters.forgive,
            )?;
            // rendu
            if uid_properties.is_empty() {
                return Ok(());
            }
            let len = uid_properties.len() - 1;
            if !pretty {
                for (nb, up) in uid_properties.into_iter().enumerate() {
                    println!("{}", up.0);
                    for p in up.1 {
                        let p = p.to_string().replace('\n', "");
                        println!("{p}")
                    }
                    if nb < len {
                        println!();
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
                        println!();
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
                &properties.show.iter().map(|p| p).collect(),
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
        ImmediateMode::RemoveProperty {
            book,
            find_filters,
            lo,
            properties,
        } => {
            add_or_replace_property(
                APP_SHORTNAME,
                &properties.show.iter().map(|p| p).collect(),
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
            let index = generate_index(APP_SHORTNAME, book_name(&book), &properties.filter)?;
            println!("{}", index.join("\n"));
            Ok(())
        }
        ImmediateMode::Import {
            path_vcards_file,
            book,
        } => Ok(import(
            &path_vcards_file,
            &book.unwrap_or_default(),
            APP_SHORTNAME,
        )?),
        ImmediateMode::Export { book } => {
            Ok(println!("{}", export(book_name(&book), APP_SHORTNAME)?))
        }

        ImmediateMode::Shortcut { shortcut, book } => Ok(shortcut_book(shortcut, book.name)?),
    }
}

fn book_name(book: &Option<Book>) -> Option<&str> {
    if let Some(b) = &book {
        Some(b.name.as_str())
    } else {
        None
    }
}
