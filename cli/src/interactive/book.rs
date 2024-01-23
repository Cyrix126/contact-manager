use super::contact::VecContact;
use super::validator_new_bookname;
use crate::APP_SHORTNAME;
use anyhow::Result;
use contact_manager::{create_book, delete_book, rename_book};
use inquire::Text;
use promptable::derive_more::Display;
use promptable::promptable_derive::Promptable;
#[derive(Promptable, Clone, Display)]
#[display(fmt = "Book {}: {}", name, "contacts.len()")]
#[prompt(function_del = "book_del(element)?")]
pub struct Book {
    #[promptable(short_display = true)]
    #[promptable(function_new = "book_add()?")]
    #[promptable(function_mod = "book_mod(field)?")]
    pub name: String,
    #[promptable(function_new = "contacts_empty_vec()?")]
    #[promptable(function_mod = "VecContact::modify_by_prompt(field, self.name.as_str())?")]
    pub contacts: VecContact,
}

fn book_del(book: &Book) -> Result<()> {
    delete_book(&book.name, APP_SHORTNAME)?;
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

// impl Promptable for Book {
//     fn new_by_prompt(msg: &str) -> Result<Option<Self>>
//     where
//         Self: Sized + Display + PartialEq,
//     {
//         if let Some(name) = choose_new_bookname()? {
//             if create_book(&name, APP_SHORTNAME).is_err() {
//                 bail!("The creation of the new book has failed.");
//             }
//         }
//         Ok(Some(Book::default()))
//     }
//     // rename book
//     fn modify_by_prompt(&mut self, msg: &str) -> Result<()>
//     where
//         Self: Sized + Display + PartialEq,
//     {
//         // choose book
//         // choose new name
// let names = books_names(APP_SHORTNAME)?;
// if let Some(old_name) =
//     Select::new("choose the book to rename:", names).prompt_skippable()?
// {
//     if let Some(new_name) = choose_new_bookname()? {
//         return rename_book(&old_name, &new_name, APP_SHORTNAME);
//     }
// }
//         Ok(())
//     }
// }

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
