mod config;
mod error;
mod keys;

use crate::error::Result;

use penrose::{
    PenroseError, WindowManager,
    __example_helpers::{KeyBindings, MouseBindings},
    logging_error_handler,
    xcb::{XcbConnection, XcbHooks},
};
use simplelog::{LevelFilter, SimpleLogger};

use std::collections::HashMap;

pub const WORKSPACES: [char; 5] = ['1', '2', '3', '4', '5'];
pub const FLOATING_CLASSES: [&str; 4] = ["dmenu", "rofi", "dunst", "polybar"];

pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "rofi";
pub const FILE_MANAGER: &str = "thunar";
pub const BROWSER: &str = "firefox";

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())
        .expect("Failed to initialize logger");
    let config = config::gen_config()?;
    let conn = XcbConnection::new()?;
    let hooks: XcbHooks = vec![];
    let error_handler: Box<dyn FnMut(PenroseError)> = logging_error_handler();
    let mut wm = WindowManager::new(config, conn, hooks, error_handler);

    let key_bindings: KeyBindings<XcbConnection> = keys::gen_key_map();
    let mouse_bindings: MouseBindings<XcbConnection> = HashMap::new();

    wm.init()?;
    wm.grab_keys_and_run(key_bindings, mouse_bindings)?;

    Ok(())
}
