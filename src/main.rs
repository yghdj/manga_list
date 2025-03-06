use crate::data::manga::Manga;

mod data;

fn main() {
    let new_series = Manga::new("Hello World");

    println!("{:?}", new_series);
}
