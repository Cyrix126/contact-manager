use vcard_parser::{
    traits::{HasParameters, HasValue},
    vcard::{
        parameter::{parameter_pid::ParameterPidData, Parameter},
        property::Property,
    },
};

/// get the value of the property
pub fn value(property: &Property) -> String {
    property.get_value().to_string()
}
/// get the pid parameter of the property
pub fn pid(property: &Property) -> Option<ParameterPidData> {
    property.get_parameters();
    for param in property.get_parameters() {
        match param {
            Parameter::ParameterPid(d) => return Some(d),
            _ => continue,
        }
    }
    None
}
