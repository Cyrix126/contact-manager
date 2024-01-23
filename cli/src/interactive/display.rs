use std::fmt::Display;

use contact_manager::vcard_parser::traits::{HasName, HasParameters, HasValue};
use promptable::basics::display::PromptableDisplay;

use super::{
    contact::{Contact, VecContact, WrapperVcard},
    menu::{ParameterWrapper, PropertyWrapper},
};

impl PromptableDisplay for PropertyWrapper {
    fn display_short(&self) -> String {
        format!("{} = {}", self.name(), self.get_value())
    }
    fn display_human(&self) -> String {
        let mut render = String::new();
        let params = self.get_parameters();
        let params_render = params
            .iter()
            .map(|p| ParameterWrapper(p.clone()).display_human())
            .collect::<Vec<String>>()
            .join("|");
        render.push_str(self.name());
        render.push('\n');
        render.push_str(&params_render);
        render.push('\n');
        render.push_str(&self.get_value().to_string());
        // or just self.get_value().to_string() ?
        render
    }
}

impl PromptableDisplay for ParameterWrapper {
    fn display_short(&self) -> String {
        self.name().to_string()
    }
    fn display_human(&self) -> String {
        format!("{} = {}", self.name(), self.get_value())
    }
}

impl PromptableDisplay for VecContact {
    fn display_short(&self) -> String {
        self.0.display_short()
    }
    fn display_human(&self) -> String {
        self.0.display_human()
    }
}

impl PromptableDisplay for Contact {
    fn display_short(&self) -> String {
        self.get_property_by_name("FN")
            .unwrap()
            .get_value()
            .to_string()
    }
    fn display_human(&self) -> String {
        self.get_properties()
            .iter()
            .map(|p| PropertyWrapper(p.clone()).display_short())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
impl PromptableDisplay for WrapperVcard {
    fn display_short(&self) -> String {
        self.get_property_by_name("FN")
            .unwrap()
            .get_value()
            .to_string()
    }
    fn display_human(&self) -> String {
        self.get_properties()
            .iter()
            .map(|p| PropertyWrapper(p.clone()).display_short())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.vcard.get_property_by_name("FN").unwrap().get_value()
        )
    }
}
