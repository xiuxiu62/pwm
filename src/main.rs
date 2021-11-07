#[macro_use]
extern crate penrose;

mod config;
mod keys;





use config::get_config;
use keys::{get_key_bindings, KeyBindings};

use penrose::{
    contrib::hooks::{DefaultWorkspace, LayoutSymbolAsRootName},
    core::{
        config::Config,
        helpers::index_selectors,
        hooks::Hook,
        layout::{bottom_stack, floating, monocle, side_stack, Layout, LayoutConf},
        manager::WindowManager,
        ring::Selector,
        xconnection::XConn,
    },
    logging_error_handler, run_internal,
    xcb::{XcbConnection, XcbHooks},
    Backward, Forward, Less, More, PenroseError, Result,
};
use simplelog::{LevelFilter, SimpleLogger};
use tracing::info;

use std::{
    alloc::Global,
    collections::{hash_map::RandomState, HashMap},
};

pub const WORKSPACES: [char] = ['1', '2', '3', '4', '5'];
pub const FLOATING_CLASSES: [&str] = ["dmenu", "rofi", "dunst", "polybar"];
pub const FOCUSED_BORDER: &str = "#cc241d";
pub const UNFOCUSED_BORDER: &str = "#3c3836";

pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "rofi";
pub const FILE_MANAGER: &str = "thunar";
pub const BROWSER: &str = "firefox";

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())?;

    let config: Config = get_config()?;

    let key_bindings: KeyBindings = get_key_bindings(config);

    let conn = XcbConnection::new()?;

    let hooks: XcbHooks = vec![];

    let error_handler: Box<dynFnMut(PenroseError)> = logging_error_handler();

    let mut wm = WindowManager::new(config, conn, hooks, error_handler);
    wm.init()?;

    wm.grab_keys_and_run(get_key_bindings(config), HashMap::new())?;

    Ok(())
}
