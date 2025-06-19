#![allow(dead_code)]

use itertools::Itertools;
use plotly::{
    common::{Anchor, ColorScalePalette, Visible},
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
        BarMode,
    },
    Bar, HeatMap, Layout, Plot,
};
use plotly_utils::write_example_to_html;

/// Display a bar chart with an associated dropdown selector to show different
/// data.
fn bar_plot_with_dropdown_for_different_data(show: bool, file_name: &str) {
    type BarType = Bar<&'static str, i32>;
    let mut plot = Plot::new();
    plot.add_trace(
        BarType::new(vec!["Giraffes", "Orangutans", "Monkeys"], vec![20, 14, 23]).name("Animals"),
    );
    plot.add_trace(
        BarType::new(
            vec!["Little Grebes", "Nuthatches", "Firecrests", "Goldfinches"],
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

/// Display a heat map, with buttons to allow for toggling of different
/// colorscales.
fn heat_map_with_modifiable_colorscale(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

/// Display a bar chart, with buttons to toggle between stacked or grouped
/// display maodes.
fn bar_chart_with_modifiable_bar_mode(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: colorscale_plot

fn main() {
    // Change false to true on any of these lines to display the example.

    bar_plot_with_dropdown_for_different_data(false, "bar_plot");
    heat_map_with_modifiable_colorscale(false, "heat_map");
    bar_chart_with_modifiable_bar_mode(false, "bar_chart");
}
