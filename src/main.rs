mod images;
mod api;

use viuer::{print, Config};

fn main() {
    let conf = Config {
        width: Some(20),
        height: Some(10),
        restore_cursor: true,
        absolute_offset: false,
        ..Default::default()
    };
    let images = images::Images::new();
    let manager = api::APIManager::new().unwrap();
    let quotes = manager.random().unwrap();
    let single = quotes.quotes.first().unwrap();
    let image = images.fetch(&single.nation);

    println!("");
    print(&image, &conf).expect("Image printing failed.");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("                         {}", single.quote);
    println!("                         {}", single.character);
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
}
