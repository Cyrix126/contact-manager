use crate::{read_contacts, vcard_fn};
use std::cmp::Ordering;
use std::path::PathBuf;
use vcard_parser::parse_to_vcards_without_errors;
use vcard_parser::vcard::property::types::PropertyType;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;
pub fn find_interactive(
    dir_contacts: PathBuf,
    property_show: PropertyType,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&dir_contacts);
    let vcards = find_vcards(all, &property_find, &value_find, &property_show);
    match vcards.len().cmp(&1) {
        Ordering::Less => {
            println!("Aucun résultat pour cette recherche.");
        }
        Ordering::Greater | Ordering::Equal => {
            for vcard in vcards {
                let properties = properties_are(&vcard, &property_show);
                println!("contact: {}", vcard_fn(&vcard));
                for property in properties {
                    println!("{}", property.get_value());
                }
            }
        }
    }
}

pub fn find_vcards(
    all: String,
    property_find: &PropertyType,
    value_find: &String,
    property_show: &PropertyType,
) -> Vec<Vcard> {
    let vcards = parse_to_vcards_without_errors(&all);
    let mut vcards_found: Vec<Vcard> = Vec::new();
    for vcard in vcards {
        if !is_property_present(&vcard, property_show) {
            continue;
        }
        let properties = vcard.get_properties_by_type(property_find);

        for property in properties {
            let value_present = property.get_value().to_string();
            // seulment le numéro pour les téléphones, pas le type. (home, work)
            //if property_find == &PropertyType::Tel {value_present = value_present.chars().filter(|c| c.is_digit(10)).collect();}

            if &value_present == value_find {
                vcards_found.push(vcard);
                // ne pas chercher une deuxième valeur pour le meme vcard
                break;
            } else {
                let patterns = value_find.split(" ");
                for x in patterns {
                    if value_present.contains(x) {
                        vcards_found.push(vcard);
                    }
                    break; // on ne cherche pas plus de pattern dans la valeur trouvé
                }
                break; // ne pas chercher une deuxième valeur pour le meme vcard
            }
        }
    }
    vcards_found
}

fn properties_are(vcard: &Vcard, property_show: &PropertyType) -> Vec<Property> {
    vcard.get_properties_by_type(property_show)
}

fn is_property_present(vcard: &Vcard, property_show: &PropertyType) -> bool {
    if vcard.get_properties_by_type(property_show).is_empty() {
        false
    } else {
        true
    }
}
