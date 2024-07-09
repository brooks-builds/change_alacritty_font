use std::env::args;

use change_alacritty_font::{change_alacritty_font, validate_font};
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let mut args = args();

    args.next();

    let new_font = args
        .next()
        .expect("missing theme, please make sure to call this command with a theme");
    let config_file_path = "/Users/brookzerker/.config/alacritty/font.toml";

    if validate_font(&new_font) {
        println!("validated font");
        change_alacritty_font(&config_file_path, &new_font, "brooks", false).unwrap();
    }
}
