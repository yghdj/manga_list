use data::manga::{Author, Editor};

use crate::data::manga::Series;

mod data;

fn main() {
    let mut rave = Series::new("Rave Groove Adventure");

    rave.authors.push(Author::new("Mashima", "Hiro", "JP"));

    rave.editors.push(Editor::new("Kōdansha", 1909, "JP"));
    rave.editors.push(Editor::new("Glénat", 1969, "FR"));
    rave.editors.push(Editor::new("TOKYOPOP", 1997, "US"));

    rave.volume_count = 35;

    println!("{:?}", rave);
}
