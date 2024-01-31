use std::fmt::Display;

use contact_manager_lib::vcard_parser::traits::HasValue;
use contact_manager_lib::vcard_parser::vcard::property::property_adr::PropertyAdrData;
use contact_manager_lib::vcard_parser::vcard::value::value_listcomponent::ValueListComponentData;
use contact_manager_lib::vcard_parser::vcard::value::Value;
use promptable::promptable_derive::Promptable;
// use promptable::Promptable;

#[derive(Promptable, Clone, Default)]
pub struct Adress {
    #[promptable(default)]
    post_office_box: Option<String>,
    #[promptable(default)]
    suite: Option<u32>,
    #[promptable(name = "Number and Adress Street")]
    #[promptable(short_display)]
    street_adress: String,
    #[promptable(name = "City")]
    locality: String,
    #[promptable(name = "State")]
    #[promptable(default)]
    region: Option<String>,
    postalcode: u32,
    #[promptable(default)]
    country: Option<String>,
}
impl Display for Adress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if suite is None, display an empty string instead of 0.
        let suite = self.suite.unwrap_or_default();
        let suite_display = if suite == 0 {
            String::new()
        } else {
            format!("{suite}")
        };
        write!(
            f,
            "{};{};{};{};{};{};{}",
            self.post_office_box.as_ref().unwrap_or(&String::new()),
            suite_display,
            self.street_adress,
            self.locality,
            self.region.as_ref().unwrap_or(&String::new()),
            self.postalcode,
            self.country.as_ref().unwrap_or(&String::new())
        )
    }
}

impl Adress {
    pub fn from_vcard_property(p: &PropertyAdrData) -> Adress {
        let data = match p.get_value() {
            Value::ValueListComponent(c) => c,
            _=> panic!("should not panic because PropertyAdrData Value is ValueListComponent and nothing else")
        };

        let delim: String = data.delimiter_child.into();
        let values = data
            .value
            .iter()
            .map(|v| v.join(&delim))
            .collect::<Vec<String>>();
        Adress {
            post_office_box: to_some_string(&values[0]),
            suite: to_some_u32(&values[1]),
            street_adress: values[2].to_string(),
            locality: values[3].to_string(),
            region: to_some_string(&values[4]),
            postalcode: values[5].parse().unwrap_or_default(),
            country: to_some_string(&values[6]),
        }
    }
    pub fn to_property_value(&self) -> Value {
        Value::ValueListComponent(ValueListComponentData::try_from((
            self.to_string().as_str(),
            ';',
            ',',
        )).expect("should not panic here because Adress to string should be a correct string t obe converted to PropertyAdrData"))
    }
}

fn to_some_u32(str: &str) -> Option<u32> {
    (!str.is_empty()).then(|| str.parse().unwrap_or_default())
}
fn to_some_string(str: &str) -> Option<String> {
    (!str.is_empty()).then(|| str.to_string())
}
