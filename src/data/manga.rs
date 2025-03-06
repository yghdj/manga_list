use std::str::FromStr;

use celes::Country;

// XXX: multiple editions by series:
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Series {
    pub title: String,
    pub authors: Vec<Author>,
    pub editors: Vec<Editor>,
    pub volume_count: i16,
}

impl Series {
    pub fn new(title: &str) -> Series {
        Series {
            title: String::from(title),
            authors: Vec::new(),
            editors: Vec::new(),
            volume_count: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Author {
    family_name: String,
    given_name: String,
    nationality: celes::Country,
}

impl Author {
    pub fn new(family_name: &str, given_name: &str, nationality: &str) -> Author {
        Author {
            family_name: String::from(family_name),
            given_name: String::from(given_name),
            nationality: Country::from_str(nationality).unwrap_or(Country::japan()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Editor {
    pub name: String,
    pub creation_year: i32,
    pub country: Country,
}

impl Editor {
    pub fn new(name: &str, creation_year: i32, country: &str) -> Editor {
        Editor {
            name: String::from(name),
            creation_year,
            country: Country::from_str(country).unwrap_or(Country::japan()),
        }
    }
}
