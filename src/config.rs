use penrose::{
    Config,
    __example_helpers::LayoutConf,
    contrib::layouts::paper,
    core::{
        layout::{bottom_stack, monocle, side_stack},
        Layout,
    },
};

use crate::WORKSPACES;

pub fn gen_config() -> Result<Config, String> {
    let mut config_builder = Config::default().builder();
    config_builder
        .workspaces(WORKSPACES)
        .bar_height(24)
        .border_px(0)
        .gap_px(5);

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
        Layout::new(
            "[paper]",
            LayoutConf {
                floating: false,
                gapless: false,
                follow_focus: true,
                allow_wrapping: false,
            },
            paper,
            n_main,
            ratio,
        ),
        Layout::floating("[----]"),
    ]);

    config_builder.build()
}
