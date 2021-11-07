use penrose::{
    contrib::layouts::paper,
    core::{
        config::Config,
        helpers::index_selectors,
        layout::{bottom_stack, monocle, side_stack, Layout, LayoutConf},
    },
    gen_keybindings, logging_error_handler, run_external, run_internal,
    xcb::{XcbConnection, XcbHooks},
    Backward, Forward, Less, More, PenroseError, Selector, WindowManager,
};
use simplelog::{LevelFilter, SimpleLogger};

use std::collections::HashMap;

pub const WORKSPACES: [char; 5] = ['1', '2', '3', '4', '5'];
pub const FLOATING_CLASSES: [&str; 4] = ["dmenu", "rofi", "dunst", "polybar"];
pub const FOCUSED_BORDER: u32 = 0xcc241d;
pub const UNFOCUSED_BORDER: u32 = 0x3c3836;

pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "rofi";
pub const FILE_MANAGER: &str = "thunar";
pub const BROWSER: &str = "firefox";

fn main() -> penrose::Result<()> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())
        .expect("Failed to initialize logger");

    let mut config_builder = Config::default().builder();
    config_builder
        .workspaces(crate::WORKSPACES)
        .floating_classes(crate::FLOATING_CLASSES)
        .focused_border(crate::FOCUSED_BORDER)
        .unfocused_border(crate::UNFOCUSED_BORDER);

    let follow_focus_conf = LayoutConf {
        floating: false,
        gapless: false,
        follow_focus: true,
        allow_wrapping: false,
    };

    let n_main = 1;
    let ratio = 0.6;

    config_builder.layouts(vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, n_main, ratio),
        Layout::new(
            "[bottom]",
            LayoutConf::default(),
            bottom_stack,
            n_main,
            ratio,
        ),
        Layout::new("mono", LayoutConf::default(), monocle, n_main, ratio),
        Layout::new("[paper]", follow_focus_conf, paper, n_main, ratio),
        Layout::floating("[----]"),
    ]);

    let config = config_builder.build().unwrap();
    let conn = XcbConnection::new()?;
    let hooks: XcbHooks = vec![];
    let error_handler: Box<dyn FnMut(PenroseError)> = logging_error_handler();
    let mut wm = WindowManager::new(config, conn, hooks, error_handler);

    let key_bindings = gen_keybindings! {
        // Programs
        "M-Return" => run_external!(TERMINAL);
        "M-p" => run_external!(LAUNCHER);
        "M-f" => run_external!(FILE_MANAGER);
        "M-w" => run_external!(BROWSER);

        // Client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-q" => run_internal!(kill_client);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);

        // Workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward);

        // Layout management
        "M-space" => run_internal!(cycle_layout, Forward);
        "M-S-space" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        "M-A-s" => run_internal!(detect_screens);
        "M-A-Escape" => run_internal!(exit);

        // Workspaces
        refmap [ 1..5 ] in {
            "M-{}" => focus_workspace [ index_selectors(5) ];
            "M-S-{}" => client_to_workspace [ index_selectors(5) ];
        };
    };

    wm.init()?;
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
