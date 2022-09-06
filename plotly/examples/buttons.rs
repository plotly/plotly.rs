use itertools::Itertools;
use plotly::{
    common::{Anchor, ColorScalePalette, Visible},
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
        BarMode,
    },
    Bar, HeatMap, Layout, Plot,
};

// Dropdown to alternate between two bar plots
fn bar_plot_visible_dropdown(show: bool) {
    type BarType = Bar<&'static str, i32>;
    let mut plot = Plot::new();
    plot.add_trace(
        BarType::new(vec!["giraffes", "orangutans", "monkeys"], vec![20, 14, 23]).name("Animals"),
    );
    plot.add_trace(
        BarType::new(
            vec!["parrot", "chicken", "bluebird", "own"],
            vec![8, 23, 17, 2],
        )
        .name("Birds")
        .visible(Visible::False),
    );
    let buttons = vec![
        ButtonBuilder::new()
            .label("Animals")
            .push_restyle(BarType::modify_visible(vec![Visible::True, Visible::False]))
            .build(),
        ButtonBuilder::new()
            .label("Birds")
            .push_restyle(BarType::modify_visible(vec![Visible::False, Visible::True]))
            .build(),
    ];
    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new().y(0.8).buttons(buttons)]));
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("bar_plot_visible_dropdown")));
}

// Heatmap with buttons to choose colorscale
fn basic_heat_map(show: bool) {
    type HeatMapType = HeatMap<f64, f64, Vec<f64>>;
    let gauss = |v: i32| (-v as f64 * v as f64 / 200.0).exp();
    let z = (-30..30)
        .map(|x| (-30..30).map(|y| gauss(x) * gauss(y)).collect_vec())
        .collect_vec();
    let trace = HeatMapType::new_z(z).color_scale(ColorScalePalette::Viridis.into());
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let buttons = IntoIterator::into_iter([
        ("Viridis", ColorScalePalette::Viridis),
        ("Portland", ColorScalePalette::Portland),
        ("Blackbody", ColorScalePalette::Blackbody),
    ])
    .map(|(label, palette)| {
        ButtonBuilder::new()
            .label(label)
            .push_restyle(HeatMapType::modify_all_color_scale(palette.into()))
            .build()
    })
    .collect_vec();

    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new()
            .ty(UpdateMenuType::Buttons)
            .y(0.8)
            .buttons(buttons)]));

    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("basic_heat_map")));
}

// Button to change barmode
fn bar_plot_relayout(show: bool) {
    type BarType = Bar<&'static str, i32>;
    let mut plot = Plot::new();
    plot.add_trace(
        BarType::new(vec!["giraffes", "orangutans", "monkeys"], vec![20, 14, 23]).name("Africa"),
    );
    plot.add_trace(
        BarType::new(vec!["giraffes", "orangutans", "monkeys"], vec![30, 8, 15]).name("Australia"),
    );
    let buttons = vec![("Group", BarMode::Group), ("Stack", BarMode::Stack)]
        .into_iter()
        .map(|(label, bar_mode)| {
            ButtonBuilder::new()
                .label(label)
                .push_relayout(Layout::modify_bar_mode(bar_mode))
                .build()
        })
        .collect_vec();

    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new()
            .x(0.1)
            .x_anchor(Anchor::Left)
            .y(1.2)
            .y_anchor(Anchor::Top)
            .ty(UpdateMenuType::Buttons)
            .direction(UpdateMenuDirection::Right)
            .buttons(buttons)]));
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("bar_plot_relayout")));
}

fn main() -> std::io::Result<()> {
    bar_plot_visible_dropdown(true);
    basic_heat_map(true);
    bar_plot_relayout(true);
    Ok(())
}
