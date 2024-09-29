mod api;
mod images;
mod utils;

use viuer::Config;
pub const MAX_LINES_FOR_QUOTE: usize = 5;
pub const HEIGHT: u8 = 10;
pub const WIDTH: u8 = 20;

fn main() {
    let viuer_conf = Config {
        width: Some(WIDTH as u32),
        height: Some(HEIGHT as u32),
        use_sixel: false, // Turning on sixel breaks wezterm for some reason
        absolute_offset: false,
        ..Default::default()
    };
    let manager = api::APIManager::new().unwrap();
    let quotes = manager.random().unwrap();
    let (single, width) = utils::get_quote_and_width(&quotes);
    utils::print_image_and_quote(single, width, &viuer_conf);
}
