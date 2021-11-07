use penrose::{
    Selector, XcbConnection,
    __example_helpers::KeyBindings,
    core::{
        data_types::Change::{Less, More},
        helpers::index_selectors,
        ring::Direction::{Backward, Forward},
    },
    gen_keybindings, run_external, run_internal,
};

use crate::{BROWSER, FILE_MANAGER, LAUNCHER, TERMINAL};

pub fn gen_key_map() -> KeyBindings<XcbConnection> {
    gen_keybindings! {
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
    }
}
