use std::{fmt, str::FromStr};

use celes::Country;
use time::macros::format_description;

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

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let all_authors: Vec<String> = self
            .authors
            .iter()
            .map(|author| format!("{}", author))
            .collect();
        write!(f, "{} ({})", self.title, all_authors.join(", "))
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

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.nationality.alpha3 {
            "JPN" | "CHN" => write!(f, "{} {}", self.family_name, self.given_name),
            _ => write!(f, "{} {}", self.given_name, self.family_name),
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Manga<'a> {
    parent: &'a Series,
    title: String,
    volume_number: i16,
    release_date: time::Date,
    isbn_code: String,
}

impl<'a> Manga<'a> {
    pub fn new(
        parent: &'a Series,
        title: &str,
        volume_number: i16,
        release_date: &str,
        isbn_code: &str,
    ) -> Manga<'a> {
        Manga {
            parent,
            title: String::from(title),
            volume_number,
            release_date: time::Date::parse(
                release_date,
                format_description!("[year]-[month]-[day]"),
            )
            .expect("Wrong date format, expected YYYY-MM-DD"),
            isbn_code: String::from(isbn_code),
        }
    }
}
