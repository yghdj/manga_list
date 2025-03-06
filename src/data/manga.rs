// represents a series
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Manga {
    title: String,
    author: Vec<Author>,
    editors: Vec<Editor>,
    volume_count: i16,
}

impl Manga {
    pub fn new(title: &str) -> Manga {
        Manga {
            title: String::from(title),
            author: Vec::new(),
            editors: Vec::new(),
            volume_count: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Author {
    family_name: String,
    given_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Editor {
    name: String,
    creation_date: time::Date,
    country: celes::Country,
}
