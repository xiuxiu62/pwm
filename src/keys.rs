use penrose::{
    core::{config::Config, helpers::index_selectors},
    Backward, Forward, Less, More, Selector,
};

pub type KeyBindings = HashMap<
    KeyCode,
    Box<dyn FnMut(&mut WindowManager<{ unknown }>) -> Result<(), PenroseError>, Global>,
    RandomState,
>;

pub fn get_key_bindings(config: Config) -> KeyMap {
    let terminal = crate::TERMINAL;
    let launcher = crate::LAUNCHER;
    let file_manager = crate::FILE_MANAGER;
    let browser = crate::BROWSER;

    let config_length = config.workspaces().len();

    gen_keybindings! {
        // Programs
        "M-Return" => run_external!(terminal);
        "M-p" => run_external!(launcher);
        "M-f" => run_external!(file_manager);
        "M-w" => run_external!(browser);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-q" => run_internal!(kill_client);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-slash" => sp.toggle();

        // workspace management
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

        // Each keybinding here will be templated in with the workspace index of each workspace,
        // allowing for common workspace actions to be bound at once.

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config_length) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config_length) ];
        };
    }
}
