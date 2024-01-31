use super::contact::VecContact;
use super::validator_new_bookname;
use crate::APP_SHORTNAME;
use anyhow::{bail, Result};
use clap_shortcuts::clap_shortcuts_derive::ShortCuts;
use contact_manager_lib::vcard::uuids_from_vcard;
use contact_manager_lib::{create_book, delete_book, rename_book};
use inquire::{Select, Text};
use promptable::derive_more::Display;
use promptable::promptable_derive::Promptable;
use promptable::termion::screen::ToMainScreen;
#[derive(Promptable, Clone, Display, ShortCuts)]
#[display(fmt = "Book {}: {}", name, "contacts.len()")]
#[prompt(trigger_del = "book_del(deleted)?")]
#[shortcut(values(
    name = "Add a Client",
    func = "add_contact(&mut self.contacts, self.name.as_str())?"
))]
#[shortcut(values(name = "Select a Client", func = "select_contact(&self.contacts)?"))]
pub struct Book {
    #[promptable(short_display = true)]
    #[promptable(function_new = "book_add()?")]
    #[promptable(function_mod = "book_mod(field)?")]
    pub name: String,
    #[promptable(function_new = "contacts_empty_vec()?")]
    #[promptable(function_mod = "VecContact::modify_by_prompt(field, self.name.as_str())?")]
    pub contacts: VecContact,
}

fn book_del(deleted_books: Vec<Book>) -> Result<()> {
    for book in deleted_books {
        delete_book(&book.name, APP_SHORTNAME)?;
    }
    Ok(())
}

fn contacts_empty_vec() -> Result<Option<VecContact>> {
    Ok(Some(VecContact(Vec::new())))
}

fn book_add() -> Result<Option<String>> {
    if let Some(name) = choose_new_bookname()? {
        create_book(&name, APP_SHORTNAME)?;
        return Ok(Some(name));
    }
    Ok(None)
}
fn book_mod(field: &mut String) -> Result<()> {
    if let Some(new_name) = choose_new_bookname()? {
        rename_book(&field, &new_name, APP_SHORTNAME)?;
        *field = new_name;
    }

    return Ok(());
}

fn choose_new_bookname() -> Result<Option<String>> {
    let validator = move |input: &str| Ok(validator_new_bookname(&input)?);
    if let Some(name) = Text::new("Book name: ")
        .with_validator(validator)
        .prompt_skippable()?
    {
        return Ok(Some(name));
    }
    Ok(None)
}

fn select_contact(contacts: &VecContact) -> Result<()> {
    if contacts.is_empty() {
        bail!("no contacts in this book")
    }
    if let Some(c) =
        Select::new("Select a contact to get the uuid", contacts.to_vec()).prompt_skippable()?
    {
        print!("{}", ToMainScreen);
        println!("{}", uuids_from_vcard(vec![&c])[0]);
    }
    Ok(())
}
fn add_contact(contacts: &mut VecContact, book_name: &str) -> Result<()> {
    if contacts.add_by_prompt_vec(book_name)? {
        print!("{}", ToMainScreen);
        println!(
            "{}",
            uuids_from_vcard(vec![&contacts.last().expect(
                "should have at least one contact because add_by_prompt_vec returned true"
            )])[0]
        );
    }
    Ok(())
}
