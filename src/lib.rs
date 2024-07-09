mod allowed_font;

use allowed_font::ALLOWED_FONTS;
use eyre::{Context, OptionExt, Result};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use toml_edit::{DocumentMut, Item, Value};

pub fn get_current_font(config_path: impl AsRef<Path>) -> Result<String> {
    let config_file = load_config(config_path.as_ref()).context("loading the config")?;
    let config = config_file
        .parse::<DocumentMut>()
        .context("parsing config file into editable format")?;
    let previous_font_family = config["font"]["normal"]["family"].clone();

    Ok(previous_font_family
        .as_str()
        .ok_or_eyre("converting font to string")?
        .to_owned())
}

pub fn change_alacritty_font(
    config_path: impl AsRef<Path>,
    new_font_family: &str,
    username: &str,
    reversion: bool,
) -> Result<String> {
    let config_file = load_config(config_path.as_ref()).context("loading the config")?;
    let mut config = config_file
        .parse::<DocumentMut>()
        .context("parsing config file into editable format")?;
    let previous_font_family = config["font"]["normal"]["family"].clone();
    let font_family_value = Value::from(new_font_family);
    let font_family = Item::Value(font_family_value);

    config["font"]["normal"]["family"] = font_family;

    save_config(config_path.as_ref(), config.to_string().as_str()).context("saving config")?;

    if reversion {
        announce_config_reverted().context("announcing config reverted")?;
    } else {
        announce_config_changed(new_font_family, username)
            .context("announce font family changed")?;
    }

    Ok(previous_font_family
        .as_str()
        .ok_or_eyre("converting previous font to string")?
        .to_owned())
}

pub fn validate_font(font_name: &str) -> bool {
    ALLOWED_FONTS.binary_search(&font_name).is_ok()
}

fn load_config(config_path: &Path) -> Result<String> {
    let mut file = File::open(config_path).context("opening config file")?;
    let mut config = String::new();

    file.read_to_string(&mut config)
        .context("reading config file to string")?;

    Ok(config)
}

fn save_config(path: &Path, config: &str) -> Result<()> {
    let mut file = std::fs::File::create(path).context("opening config file in write mode")?;

    file.write(config.as_bytes())
        .context("writing config to file")?;
    Ok(())
}

fn announce_config_changed(new_font: &str, username: &str) -> Result<()> {
    std::process::Command::new("say")
        .arg(username)
        .arg("changed font to ")
        .arg(new_font)
        .arg("for a few minutes")
        .output()
        .context("speaking")?;

    Ok(())
}

fn announce_config_reverted() -> Result<()> {
    std::process::Command::new("say")
        .arg("ok, that was nice, but back to our previous font")
        .output()
        .context("speaking")?;

    Ok(())
}
