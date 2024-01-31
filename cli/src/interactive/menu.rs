use crate::{interactive::custom_input::input_value, APP_SHORTNAME};
use anyhow::Result;
use contact_manager_lib::{
    add_or_replace_property,
    api_tools::remove_parameters,
    delete_properties,
    uuid::Uuid,
    vcard::PROPERTY_NO_MODIFICATION_BY_USER,
    vcard_parser::{
        traits::{HasName, HasParameters, HasValue},
        vcard::{parameter::Parameter, property::Property},
    },
};
use inquire::{MultiSelect, Select, Text};
use promptable::derive_more::{Deref, DerefMut};
use promptable::{
    basics::display::PromptableDisplay,
    basics::menu::{menu_cancel, menu_confirm, MenuClassic},
};

use super::{contact::Contact, properties_to_add, validator_param};

#[derive(Deref, DerefMut)]
pub struct PropertyWrapper(pub Property);
#[derive(Deref, DerefMut)]
pub struct ParameterWrapper(pub Parameter);

pub fn menu_properties(contact: &mut Contact) -> Result<()> {
    let options = MenuClassic::consts().to_vec();
    let restore_contact = contact.clone();
    loop {
        println!("{}", contact.display_human());
        if let Some(choix) = Select::new("Action on Property:\n", options.clone())
            .without_filtering()
            .prompt_skippable()?
        {
            match choix {
                MenuClassic::ADD => menu_properties_add(contact)?,
                MenuClassic::MODIFY => menu_properties_modify(contact)?,
                MenuClassic::DELETE => menu_properties_delete(contact)?,
                MenuClassic::CANCEL => {
                    if menu_cancel(&restore_contact, contact)? {
                        return Ok(());
                    }
                }
                _ => {
                    if menu_confirm(&restore_contact, contact)? {
                        return Ok(());
                    }
                }
            }
        } else {
            if menu_cancel(&restore_contact, contact)? {
                return Ok(());
            }
        }
    }
}
fn menu_properties_add(contact: &mut Contact) -> Result<()> {
    // choose a possible property to add

    let properties = properties_to_add(&contact.vcard);
    let named_properties: Vec<&str> = properties.iter().map(|p| p.name()).collect();
    let choix = Select::new("Property to add", named_properties).prompt_skippable()?;
    if let Some(c) = choix {
        if let Some(value) = input_value(c, None)? {
            let raw = format!("{c}:{value}\n");
            let property = &Property::create_from_str(&raw)?;
            contact.set_property(&property)?;
            add_or_replace_property(
                APP_SHORTNAME,
                &vec![property],
                &vec![Uuid::parse_str(
                    &contact
                        .get_property_by_name("UID")
                        .unwrap()
                        .get_value()
                        .to_string(),
                )
                .unwrap()],
            )?;
        }
    }
    Ok(())
}
fn menu_properties_modify(contact: &mut Contact) -> Result<()> {
    let contact_restore = contact.clone();
    let properties = contact.get_properties();
    if let Some(mut property) = Select::new("Property to modify", properties).prompt_skippable()? {
        let options = [
            "Parameters",
            "Value",
            MenuClassic::CANCEL,
            MenuClassic::CONFIRM,
        ]
        .to_vec();
        if let Some(c) = Select::new("Choice", options).prompt_skippable()? {
            match c {
                "Parameters" => menu_params(&mut property)?,
                "Value" => {
                    let name = property.name().to_string();
                    input_value(&name, Some(&mut property))?;
                }
                MenuClassic::CANCEL => {
                    if menu_cancel(&contact_restore, contact)? {
                        return Ok(());
                    }
                }
                _ => {
                    if menu_confirm(&contact_restore, contact)? {
                        add_or_replace_property(
                            APP_SHORTNAME,
                            &vec![&property],
                            &vec![Uuid::parse_str(
                                &contact
                                    .get_property_by_name("UID")
                                    .unwrap()
                                    .get_value()
                                    .to_string(),
                            )
                            .unwrap()],
                        )?;

                        return Ok(());
                    }
                }
            }
        }
    }
    Ok(())
}
fn menu_properties_delete(contact: &mut Contact) -> Result<()> {
    let mut properties = contact.get_properties();
    properties.retain(|p| PROPERTY_NO_MODIFICATION_BY_USER.contains(&p.name()) && p.name() != "FN");

    let choix = MultiSelect::new("Properties to remove", properties).prompt_skippable()?;
    if let Some(vp) = choix {
        for p in vp.iter() {
            contact.remove_property(&p)?;
        }
        delete_properties(
            APP_SHORTNAME,
            &vp.iter().map(|p| p).collect(),
            &vec![Uuid::parse_str(
                &contact
                    .get_property_by_name("UID")
                    .unwrap()
                    .get_value()
                    .to_string(),
            )?],
        )?
    }
    Ok(())
}

fn menu_params_add(property: &mut Property) -> Result<()> {
    let params = property.allowed_parameters();
    if let Some(name_param) = Select::new("Param to add", params).prompt_skippable()? {
        let validator = move |input: &str| {
            let param_raw = [";", &name_param, "=", input].concat();
            Ok(validator_param(param_raw)?)
        };

        if let Ok(value) = Text::new("value of parameter:")
            .with_validator(validator)
            .prompt()
        {
            let param_raw = [";", &name_param, "=", &value].concat();
            let param = Parameter::try_from(param_raw.as_str())?;
            property.add_parameter(param)?;
        };
    }
    Ok(())
}
fn menu_params_modify(property: &mut Property) -> Result<()> {
    if let Some(param) =
        Select::new("Param to modify", property.get_parameters()).prompt_skippable()?
    {
        let param_name = param.name().to_string();
        let validator = move |input: &str| {
            let param_raw = [";", &param_name, "=", input].concat();
            Ok(validator_param(param_raw)?)
        };

        if let Ok(value) = Text::new("value of parameter:")
            .with_placeholder(&param.get_value().to_string())
            .with_default(&param.get_value().to_string())
            .with_validator(validator)
            .prompt()
        {
            let param_raw = [";", &param.name(), "=", &value].concat();
            let new_param = Parameter::try_from(param_raw.as_str())?;
            remove_parameters(property, &vec![param]);
            property.add_parameter(new_param)?;
        }
    }
    Ok(())
}

fn menu_params_delete(property: &mut Property) -> Result<()> {
    if let Some(params) =
        MultiSelect::new("Params to remove", property.get_parameters()).prompt_skippable()?
    {
        remove_parameters(property, &params);
    }
    Ok(())
}

fn menu_params(property: &mut Property) -> Result<()> {
    let property_restore = property.clone();
    let options = MenuClassic::consts().to_vec();
    loop {
        if let Some(c) = Select::new("Choice for Parameters:\n", options.clone())
            .without_filtering()
            .prompt_skippable()?
        {
            match c {
                MenuClassic::ADD => menu_params_add(property)?,
                MenuClassic::MODIFY => menu_params_modify(property)?,
                MenuClassic::DELETE => menu_params_delete(property)?,
                MenuClassic::CANCEL => {
                    if menu_cancel(&property_restore, property)? {
                        return Ok(());
                    }
                }
                _ => {
                    if menu_confirm(
                        &PropertyWrapper(property_restore.clone()),
                        &PropertyWrapper(property.clone()),
                    )? {
                        return Ok(());
                    }
                }
            }
        } else {
            if menu_cancel(&property_restore, property)? {
                return Ok(());
            }
        }
    }
}

// fullname can't be deleted
// begin,end,version,uid can't be changed.
// if added, no unique property can be choosen if already present.
// if modified, parameters can be added/modified/deleted. value of parameter can be modified.
// escape come back cancel on previous menu.

// last options are cancel(or escape) and confirm.
