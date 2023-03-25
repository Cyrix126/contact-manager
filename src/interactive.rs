use crate::args::PropertyType;
use crate::{read_contacts, vcard_fn};
use std::cmp::Ordering;
use std::path::PathBuf;
use vcard_parser::parse_vcards;
use vcard_parser::traits::HasValue;
use vcard_parser::vcard::property::Property;
use vcard_parser::vcard::Vcard;
pub fn find_interactive(
    dir_contacts: PathBuf,
    property_show: PropertyType,
    property_find: PropertyType,
    value_find: &str,
) {
    let all = read_contacts(&dir_contacts);
    let vcards = find_vcards(all, &property_find, &value_find, &property_show);
    match vcards.len().cmp(&1) {
        Ordering::Less => {
            println!("Aucun résultat pour cette recherche.");
        }
        Ordering::Greater | Ordering::Equal => {
            for vcard in vcards {
                let properties = properties_by_name(&vcard, &property_show);
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
    value_find: &str,
    property_show: &PropertyType,
) -> Vec<Vcard> {
    let vcards = parse_vcards(&all).unwrap();
    let mut vcards_found: Vec<Vcard> = Vec::new();
    
    for vcard in vcards {
        if properties_by_name(&vcard, property_show).is_empty() {
            continue
        }
        let properties = properties_by_name(&vcard, property_find);
 
        for property in properties {
            let value_present = property.get_value().to_string();
            // seulment le numéro pour les téléphones, pas le type. (home, work)
            //if property_find == &PropertyType::Tel {value_present = value_present.chars().filter(|c| c.is_digit(10)).collect();}
            if value_present == value_find {
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





pub fn properties_by_name(vcard: &Vcard, property_find: &PropertyType) -> Vec<Property> {
       let mut properties = Vec::new();
        match property_find {
             PropertyType::Fn|PropertyType::NickName => {if let Some(property) = vcard.get_property_by_name(property_find.to_name()) {
                 properties.push(property)
             }},
             PropertyType::Adr|PropertyType::Tel|PropertyType::Email => { properties.extend(vcard.get_properties_by_name(property_find.to_name()).into_iter())}
        }
        properties
}