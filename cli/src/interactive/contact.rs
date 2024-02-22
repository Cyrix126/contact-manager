use core::panic;

use crate::interactive::menu::menu_properties;
use crate::APP_SHORTNAME;

use anyhow::Result;
use contact_manager_lib::{
    add_to_book,
    api_tools::generate_uid_property,
    create_contact,
    paths::books_names,
    remove_from_book,
    vcard::uuids_from_vcards,
    vcard_parser::{
        constants::PropertyName,
        traits::HasValue,
        vcard::{property::Property, Vcard},
    },
    vcards_from_book,
};

use inquire::{Select, Text};

use promptable::derive_more::{Deref, DerefMut};
use promptable::promptable_derive::Promptable;
#[derive(Promptable, Clone, Deref, DerefMut)]
#[prompt(custom_prompt_display)]
#[prompt(params = "book: &str")]
#[prompt(
    trigger_del = "remove_from_book(APP_SHORTNAME, params, &uuids_from_vcards(&deleted.iter().map(|d|&d.0).collect())?)?"
)]
pub struct Contact {
    #[promptable(function_add = "contact_add_from_or_create(params)?")]
    #[promptable(function_new = "contact_new_by_prompt(params)?")]
    #[promptable(function_mod = "contact_modify_by_prompt(field)?")]
    #[deref]
    #[deref_mut]
    #[promptable(inspect = false)]
    pub vcard: WrapperVcard,
}

#[derive(Deref, DerefMut, Clone)]
pub struct WrapperVcard(pub Vcard);

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        get_fn(&self).get_value().to_string() == get_fn(&other).get_value().to_string()
    }
}

fn get_fn(contact: &Contact) -> Property {
    contact
        .vcard
        .get_property_by_name(PropertyName::FN)
        .unwrap()
}

// impl Display for Contact {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let fullname = self
//             .0
//             .get_property_ref(&Property::PropertyFn(PropertyFnData::default()))
//             .expect("this vcard does not have a full name");
//         let uid = self
//             .0
//             .get_property_ref(&Property::PropertyUid(PropertyUidData::default()))
//             .expect("this vcard does not have an uid");
//         writeln!(f, "Full Name: {}", fullname.get_value());
//         writeln!(f, "Uuid: {}", uid.get_value())
//     }
// }

fn contact_add_from_or_create(book: &str) -> Result<Option<WrapperVcard>> {
    let options = vec![
        "Create new contact",
        "Add from another book",
        "Move In from another book",
    ];
    if let Some(choice) = Select::new("Action", options).prompt_skippable()? {
        match choice {
            "Create new contact" => return contact_new_by_prompt(book),
            "Add from another book" => return copy_contacts_from_book(book),
            "Move In from another book" => return moves_in_from_book(book),
            _ => panic!(),
        }
    }
    Ok(None)
}

fn vcards2contacts(vcards: Vec<Vcard>) -> VecContact {
    VecContact(
        vcards
            .into_iter()
            .map(|v| Contact {
                vcard: WrapperVcard(v),
            })
            .collect(),
    )
}

fn moves_in_from_book(book: &str) -> Result<Option<WrapperVcard>> {
    // choose book
    let mut books = books_names(APP_SHORTNAME)?;
    books.retain(|c| c != book);
    if let Some(b) =
        Select::new("Select the book to move in the contact from:", books).prompt_skippable()?
    {
        let mut contacts = vcards2contacts(vcards_from_book(APP_SHORTNAME, Some(&b))?);
        let contacts_already_present: VecContact =
            vcards2contacts(vcards_from_book(APP_SHORTNAME, Some(book))?);
        contacts.retain(|c: &Contact| !contacts_already_present.contains(c));
        // if we could return a vec for add in Promptable
        // if let Some(contacts) = MultiSelect::new("Select the contacts to move in and delete from this book", contacts.0).prompt_skippable()? {
        //     let uuids = uuids_from_vcard(&contacts.iter().map(|c|c.vcard.0).collect());
        //     add_to_book(APP_SHORTNAME, book, &uuids)?;
        //     remove_from_book(APP_SHORTNAME, &b, &uuids)?;
        // }
        if let Some(contact) = Select::new(
            "Select the contacts to move in and delete from this book",
            contacts.0,
        )
        .prompt_skippable()?
        {
            let uuids = uuids_from_vcards(&vec![&contact.vcard.0])?;
            add_to_book(APP_SHORTNAME, book, &uuids)?;
            remove_from_book(APP_SHORTNAME, &b, &uuids)?;
            return Ok(Some(contact.vcard));
        }
    }
    Ok(None)
}

fn copy_contacts_from_book(book: &str) -> Result<Option<WrapperVcard>> {
    // contacts
    let mut contacts = vcards2contacts(vcards_from_book(APP_SHORTNAME, None)?);

    let contacts_already_present = vcards2contacts(vcards_from_book(APP_SHORTNAME, Some(book))?);
    contacts.retain(|c| !contacts_already_present.contains(c));
    // if we could return a vec for add in Promptable
    //     if let Some(vcards) = MultiSelect::new("Select the contacts to add", contacts.0).prompt_skippable()? {
    //         let uuids = uuids_from_vcard(&vcards.iter().map(|c|c.vcard.0).collect());
    //         add_to_book(APP_SHORTNAME, book, &uuids)?;
    // }

    if let Some(contact) =
        Select::new("Select the contacts to add", contacts.0).prompt_skippable()?
    {
        let uuids = uuids_from_vcards(&vec![&contact.vcard.0])?;
        add_to_book(APP_SHORTNAME, book, &uuids)?;
        return Ok(Some(contact.vcard));
    }
    Ok(None)
}

fn contact_new_by_prompt(book: &str) -> Result<Option<WrapperVcard>> {
    if let Some(fullname) = Text::new("Insert Full Name:").prompt_skippable()? {
        let mut vcard = Vcard::new(&fullname);
        vcard.set_property(&generate_uid_property()?.0)?;
        create_contact(APP_SHORTNAME, &book, &vec![fullname])?;
        Ok(Some(WrapperVcard(vcard)))
    } else {
        Ok(None)
    }
}
fn contact_modify_by_prompt(field: &mut WrapperVcard) -> Result<()> {
    let mut contact = Contact {
        vcard: field.clone(),
    };
    menu_properties(&mut contact)?;
    *field = contact.vcard;

    Ok(())
}
