#[macro_use] extern crate unic_char_range;

use unic_emoji_char::is_emoji;
use unic_ucd_name::Name;
use unic_ucd_name_aliases::{name_aliases_of, NameAliasType};
use unic_ucd_category::GeneralCategory;

extern crate entities;
use entities::ENTITIES;
use std::collections::HashMap;

fn make_char_to_entity_mapping() -> HashMap<char, Vec<&'static str> > {
    let mut mapping = HashMap::new();
    for ent in ENTITIES.iter() {
        for c in ent.characters.chars() {
            if ! mapping.contains_key(&c) {
                mapping.insert(c, Vec::new());
            }
            mapping.get_mut(&c).unwrap().push(ent.entity);
        }
    }
    mapping
}

#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref CHAR_TO_ENTITY: HashMap<char, Vec<&'static str> > =
        make_char_to_entity_mapping();
}

fn all_names_that_make_sense(c: char) -> Option<String> {
    let relevant_alias_types = [
        NameAliasType::ControlCodeNames,
        NameAliasType::AlternateNames,
        NameAliasType::NameAbbreviations,
        NameAliasType::NameCorrections
    ];

    let mut names: Vec<String> = Vec::new();
    match Name::of(c) {
        Some(Name::NR1(x)) => names.push(x.to_string()),
        Some(Name::NR2(x,y)) => { names.push(vec!(x).join(" ") + &y.to_string()) },
        Some(Name::NR3(x)) => names.push(x.join(" ")),
        None => ()
    }

    for nat in relevant_alias_types.iter() {
        match name_aliases_of(c, *nat) {
            Some(x) => names.push(x.join(" ")),
            None => ()
        }
    }

    if is_emoji(c) {
        if let Some(name) = names.first_mut() {
            *name += " EMOJI";
        }
    }

    match CHAR_TO_ENTITY.get(&c) {
        Some(x) => {
            for y in x {
                names.push(String::from(*y))
            }
        }
        None => ()
    }

    let l = names.len();

    if l >= 2 {
        Some(format!("{}  {}", names[0], names[1..].join("  ")))
    } else if l == 1 {
        Some(names[0].clone())
    } else {
        None
    }
}

fn is_printable_without_messing_up_terminal(c: char) -> bool {
    let gc = GeneralCategory::of(c);
    if gc.is_mark() || gc.is_other() {
        false
    } else {
        true
    }
}

fn main() {
    for c in chars!(..) {
        match all_names_that_make_sense(c) {
            Some(x) => {
                let pc = if is_printable_without_messing_up_terminal(c) {c} else {' '};
                println!("{}  {}", pc, x);
            }
            None => ()
        }
    }
}
