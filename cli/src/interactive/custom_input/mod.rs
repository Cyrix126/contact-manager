pub mod adr;
use crate::interactive::custom_input::adr::Adress;
use crate::interactive::show_property;
use crate::interactive::Property;
use anyhow::bail;
use anyhow::Result;
use contact_manager::vcard_parser::constants::PropertyName;
use contact_manager::vcard_parser::traits::HasValue;
use contact_manager::vcard_parser::vcard::value::value_listcomponent::ValueListComponentData;
use contact_manager::vcard_parser::vcard::value::Value;
use inquire::validator::Validation;
use inquire::Text;
use promptable::basics::promptable::Promptable;
pub fn input_value(name: &str, property: Option<&mut Property>) -> Result<Option<Value>> {
    // a simple implementation would be to use TryFrom.
    // a more user friendly implementation would be to guide the user and to ask with multiple questions for value with listcomponent. or textlist. Like Adr.
    if let Some(ref p) = property {
        show_property(p);
    }
    let n = name.to_string();
    match name {
        PropertyName::ADR => {
            // TODO, ask if the user want a selection of the adress to be sure it exist or verify while he writes and ask if he wants to input a unkown adress if that's the case.
            // the proposed anwser could be filtered by features. For example, if feature adr-fr is enabled, the adresses of France will appear.
            // if no features about the adresses is enabled, the user will be able to input any adresses.
            if let Some(p) = property {
                let data_adr = match p {
                    Property::PropertyAdr(c) => c,
                    _ => bail!("the name was ADR but the property was not"),
                };
                let mut adr = Adress::from_vcard_property(&data_adr);
                adr.modify_by_prompt(())?;
                p.set_value(adr.to_property_value())?;
                Ok(None)
            } else {
                let adr = Adress::new_by_prompt(())?.unwrap();
                let value = Value::try_from(ValueListComponentData::try_from((
                    adr.to_string().as_str(),
                    ';',
                    ',',
                ))?)?;
                Ok(Some(value))
            }
        }

        _ => {
            let validator = move |input: &str| {
                let raw = format!("{n}:{input}\n");
                if Property::try_from(raw.as_str()).is_ok() {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(
                        "Value is not correct for this Property".into(),
                    ))
                }
            };
            if let Some(value) = Text::new("Insert value:")
                .with_validator(validator)
                .prompt_skippable()?
            {
                let raw = format!("{name}:{value}\n");
                let pr = Property::create_from_str(&raw)?;
                let value = pr.get_value();
                if let Some(p) = property {
                    p.set_value(value.to_owned())?;
                    return Ok(None);
                } else {
                    return Ok(Some(value.to_owned()));
                }
            }
            Ok(None)
        }
    }
}
