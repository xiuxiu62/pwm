use penrose::{
    contrib::layouts::paper,
    core::{
        config::Config,
        layout::{bottom_stack, floating, monocle, side_stack, Layout, LayoutConf},
    },
    Result,
};

pub fn get_config() -> Result<Config> {
    let mut config_builder = Config::default().builder();
    config_builder
        .workspaces(crate::WORKSPACES)
        .floating_classes(crate::FLOATING_CLASSES)
        .focused_border(crate::FOCUSED_BORDER)?
        .unfocused_border(crate::UNFOCUSED_BORDER)?;

    let follow_focus_conf = LayoutConf {
        floating: false,
        gapless: false,
        follow_focus: true,
        allow_wrapping: false,
    };

    config_builder.layouts(get_layouts());

    let config = config_builder.build()?;
}

fn get_layouts() -> Vec<Layout> {
    // Default Number of clients | Default percentage of screen to fill with main area
    let n_main = 1;
    let ratio = 0.6;

    vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, n_main, ratio),
        Layout::new(
            "[bottom]",
            LayoutConf::default(),
            bottom_stack,
            n_main,
            ratio,
        ),
        Layout::new("mono", monocle, n_main, ratio),
        Layout::new("[paper]", follow_focus_conf, paper, n_main, ratio),
        Layout::floating("[----]"),
    ]
}
