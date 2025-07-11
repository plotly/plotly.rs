#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;
use plotly::{
    common::{Anchor, ColorScalePalette, Font, Mode, Pad, Title, Visible},
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
        AnimationOptions, Axis, BarMode, Layout, Slider, SliderCurrentValue,
        SliderCurrentValueXAnchor, SliderStep, SliderStepBuilder,
    },
    Bar, HeatMap, Plot, Scatter,
};
use plotly_utils::write_example_to_html;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GapminderData {
    country: String,
    year: i32,
    pop: f64,
    continent: String,
    #[serde(rename = "lifeExp")]
    life_exp: f64,
    #[serde(rename = "gdpPercap")]
    gdp_per_cap: f64,
}

fn load_gapminder_data() -> Vec<GapminderData> {
    let mut rdr = csv::Reader::from_path("assets/gapminder.csv").unwrap();
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: GapminderData = result.unwrap();
        data.push(record);
    }
    data
}

// ANCHOR: bar_plot_with_dropdown_for_different_data
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
            .build()
            .unwrap(),
        ButtonBuilder::new()
            .label("Birds")
            .push_restyle(BarType::modify_visible(vec![Visible::False, Visible::True]))
            .build()
            .unwrap(),
    ];
    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new().y(0.8).buttons(buttons)]));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: bar_plot_with_dropdown_for_different_data

// ANCHOR: heat_map_with_modifiable_colorscale
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
            .unwrap()
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
// ANCHOR_END: heat_map_with_modifiable_colorscale

// ANCHOR: bar_chart_with_modifiable_bar_mode
/// Display a bar chart, with buttons to toggle between stacked or grouped
/// display modes.
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
                .unwrap()
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
// ANCHOR_END: bar_chart_with_modifiable_bar_mode

// ANCHOR: bar_chart_with_slider_customization
/// Display a bar chart with a data slider to animate through different years,
/// showing how to customize the slider.
fn bar_chart_with_slider_customization(show: bool, file_name: &str) {
    type BarType = Bar<&'static str, i32>;
    let mut plot = Plot::new();
    plot.add_trace(
        BarType::new(vec!["Giraffes", "Orangutans", "Monkeys"], vec![20, 14, 23])
            .name("2019")
            .visible(Visible::True),
    );
    plot.add_trace(
        BarType::new(vec!["Giraffes", "Orangutans", "Monkeys"], vec![25, 18, 28])
            .name("2020")
            .visible(Visible::False),
    );
    plot.add_trace(
        BarType::new(vec!["Giraffes", "Orangutans", "Monkeys"], vec![22, 16, 25])
            .name("2021")
            .visible(Visible::False),
    );
    plot.add_trace(
        BarType::new(vec!["Giraffes", "Orangutans", "Monkeys"], vec![28, 20, 30])
            .name("2022")
            .visible(Visible::False),
    );
    let slider_steps = vec![
        SliderStepBuilder::new()
            .label("2019")
            .value("2019")
            .push_restyle(BarType::modify_visible(vec![
                Visible::True,
                Visible::False,
                Visible::False,
                Visible::False,
            ]))
            .build()
            .unwrap(),
        SliderStepBuilder::new()
            .label("2020")
            .value("2020")
            .push_restyle(BarType::modify_visible(vec![
                Visible::False,
                Visible::True,
                Visible::False,
                Visible::False,
            ]))
            .build()
            .unwrap(),
        SliderStepBuilder::new()
            .label("2021")
            .value("2021")
            .push_restyle(BarType::modify_visible(vec![
                Visible::False,
                Visible::False,
                Visible::True,
                Visible::False,
            ]))
            .build()
            .unwrap(),
        SliderStepBuilder::new()
            .label("2022")
            .value("2022")
            .push_restyle(BarType::modify_visible(vec![
                Visible::False,
                Visible::False,
                Visible::False,
                Visible::True,
            ]))
            .build()
            .unwrap(),
    ];

    plot.set_layout(
        Layout::new()
            .title("Animal Population by Year (Custom Slider Fields)")
            .sliders(vec![Slider::new()
                .active(0)
                .steps(slider_steps)
                .x(0.2)
                .x_anchor(Anchor::Left)
                .y(-0.6)
                .y_anchor(Anchor::Bottom)
                .length(0.8)
                .background_color("#f8fafc")
                .border_color("#bec8d9")
                .border_width(2)
                .tick_color("#e74c3c")
                .tick_length(15)
                .tick_width(4)
                .current_value(
                    SliderCurrentValue::new()
                        .prefix("Year: ")
                        .suffix(" (selected)")
                        .visible(true)
                        .x_anchor(SliderCurrentValueXAnchor::Center)
                        .font(Font::new().size(16).color("#2c3e50"))
                        .offset(5),
                )]),
    );
    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: bar_chart_with_slider_customization

// ANCHOR: sinusoidal_slider_example
// Sinusoidal Wave with Slider Control
fn sinusoidal_slider_example(show: bool, file_name: &str) {
    use ndarray::Array;

    let mut plot = Plot::new();
    let num_steps = 51; // 0..=50 for frequencies 0.0 to 5.0 in 0.1 steps

    // Add traces, one for each slider step (frequency parameter)
    for step in 0..=num_steps {
        let frequency = step as f64 / 10.0; // 0.0, 0.1, ..., 5.0
        let x: Vec<f64> = Array::linspace(0.0, 10.0, 1001).into_raw_vec().to_vec();
        let y: Vec<f64> = x.iter().map(|&x_val| (frequency * x_val).sin()).collect();
        let trace = Scatter::new(x, y)
            .visible(if step == 10 {
                Visible::True
            } else {
                Visible::False
            }) // Make 10th trace visible
            .line(plotly::common::Line::new().color("#00CED1").width(6.0))
            .name(format!("ν = {frequency:.1}"));
        plot.add_trace(trace);
    }

    // Create slider steps
    let mut steps = Vec::new();
    for i in 0..num_steps {
        let frequency = i as f64 / 10.0;
        let mut visible = vec![Visible::False; num_steps];
        visible[i] = Visible::True;
        let step = SliderStepBuilder::new()
            .label(format!("step-{i}"))
            .value(format!("{frequency:.1}"))
            .push_restyle(Scatter::<f64, f64>::modify_visible(visible))
            .push_relayout(Layout::modify_title(format!(
                "Slider switched to step: {i}"
            )))
            .build()
            .unwrap();
        steps.push(step);
    }
    let layout = Layout::new()
        .title(Title::with_text("Simple Slider Control"))
        .sliders(vec![Slider::new()
            .active(10)
            .pad(Pad::new(50, 0, 0))
            .steps(steps)]);
    plot.set_layout(layout);
    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: sinusoidal_slider_example

// ANCHOR: gdp_life_expectancy_slider_example
// GDP per Capita/Life Expectancy Slider (matches second plot in https://plotly.com/python/sliders/)
fn gdp_life_expectancy_slider_example(show: bool, file_name: &str) {
    let data = load_gapminder_data();

    // Get unique years and sort them
    let years: Vec<i32> = data
        .iter()
        .map(|d| d.year)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .sorted()
        .collect();

    // Create color mapping for continents to match the Python plotly example
    let continent_colors = HashMap::from([
        ("Asia".to_string(), "rgb(99, 110, 250)"),
        ("Europe".to_string(), "rgb(239, 85, 59)"),
        ("Africa".to_string(), "rgb(0, 204, 150)"),
        ("Americas".to_string(), "rgb(171, 99, 250)"),
        ("Oceania".to_string(), "rgb(255, 161, 90)"),
    ]);
    let continents: Vec<String> = continent_colors.keys().cloned().sorted().collect();

    let mut plot = Plot::new();

    // Create a trace for each continent for each year
    for &year in &years {
        for continent in &continents {
            let records: Vec<&GapminderData> = data
                .iter()
                .filter(|d| d.continent == *continent && d.year == year)
                .collect();

            if !records.is_empty() {
                let x: Vec<f64> = records.iter().map(|r| r.gdp_per_cap).collect();
                let y: Vec<f64> = records.iter().map(|r| r.life_exp).collect();
                let size: Vec<f64> = records.iter().map(|r| r.pop).collect();
                let hover: Vec<String> = records.iter().map(|r| r.country.clone()).collect();

                let trace = Scatter::new(x, y)
                    .name(continent)
                    .mode(Mode::Markers)
                    .hover_text_array(hover)
                    .visible(if year == years[0] {
                        Visible::True
                    } else {
                        Visible::False
                    })
                    .marker(
                        plotly::common::Marker::new()
                            .color(*continent_colors.get(continent).unwrap())
                            .size_array(size.into_iter().map(|s| s as usize).collect())
                            .size_mode(plotly::common::SizeMode::Area)
                            .size_ref(200000)
                            .size_min(4),
                    );
                plot.add_trace(trace);
            }
        }
    }

    // Create slider steps for each year
    let steps: Vec<SliderStep> = years
        .iter()
        .enumerate()
        .map(|(i, &year)| {
            let mut visible = vec![Visible::False; years.len() * continents.len()];
            let start = i * continents.len();
            let end = start + continents.len();
            visible[start..end].fill(Visible::True);

            SliderStepBuilder::new()
                .label(year.to_string())
                .value(year)
                .push_restyle(Scatter::<f64, f64>::modify_visible(visible))
                .push_relayout(Layout::modify_title(format!(
                    "GDP vs. Life Expectancy ({year})"
                )))
                .build()
                .unwrap()
        })
        .collect();

    let layout = Layout::new()
        .title(Title::with_text(format!(
            "GDP vs. Life Expectancy ({})",
            years[0]
        )))
        .x_axis(
            Axis::new()
                .title(Title::with_text("gdpPercap"))
                .type_(plotly::layout::AxisType::Log),
        )
        .y_axis(
            Axis::new()
                .title(Title::with_text("lifeExp"))
                .range(vec![30.0, 85.0]), // Fixed range for Life Expectancy
        )
        .sliders(vec![Slider::new().active(0).steps(steps).current_value(
            SliderCurrentValue::new()
                .visible(true)
                .prefix("Year: ")
                .x_anchor(SliderCurrentValueXAnchor::Right)
                .font(Font::new().size(20).color("rgb(102, 102, 102)")),
        )]);
    plot.set_layout(layout);
    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: gdp_life_expectancy_slider_example

// ANCHOR: gdp_life_expectancy_animation_example
// GDP per Capita/Life Expectancy Animation (animated version of the slider
// example)
fn gdp_life_expectancy_animation_example(show: bool, file_name: &str) {
    use plotly::{
        common::Font,
        common::Pad,
        common::Title,
        layout::Axis,
        layout::{
            update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
            Animation, AnimationMode, Frame, FrameSettings, Slider, SliderCurrentValue,
            SliderCurrentValueXAnchor, SliderStepBuilder, TransitionSettings,
        },
        Layout, Plot, Scatter,
    };

    let data = load_gapminder_data();

    // Get unique years and sort them
    let years: Vec<i32> = data
        .iter()
        .map(|d| d.year)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .sorted()
        .collect();

    // Create color mapping for continents to match the Python plotly example
    let continent_colors = HashMap::from([
        ("Asia".to_string(), "rgb(99, 110, 250)"),
        ("Europe".to_string(), "rgb(239, 85, 59)"),
        ("Africa".to_string(), "rgb(0, 204, 150)"),
        ("Americas".to_string(), "rgb(171, 99, 250)"),
        ("Oceania".to_string(), "rgb(255, 161, 90)"),
    ]);
    let continents: Vec<String> = continent_colors.keys().cloned().sorted().collect();

    let mut plot = Plot::new();
    let mut initial_traces = Vec::new();

    for (frame_index, &year) in years.iter().enumerate() {
        let mut frame_traces = plotly::Traces::new();

        for continent in &continents {
            let records: Vec<&GapminderData> = data
                .iter()
                .filter(|d| d.continent == *continent && d.year == year)
                .collect();

            if !records.is_empty() {
                let x: Vec<f64> = records.iter().map(|r| r.gdp_per_cap).collect();
                let y: Vec<f64> = records.iter().map(|r| r.life_exp).collect();
                let size: Vec<f64> = records.iter().map(|r| r.pop).collect();
                let hover: Vec<String> = records.iter().map(|r| r.country.clone()).collect();

                let trace = Scatter::new(x, y)
                    .name(continent)
                    .mode(Mode::Markers)
                    .hover_text_array(hover)
                    .marker(
                        plotly::common::Marker::new()
                            .color(*continent_colors.get(continent).unwrap())
                            .size_array(size.into_iter().map(|s| s as usize).collect())
                            .size_mode(plotly::common::SizeMode::Area)
                            .size_ref(200000)
                            .size_min(4),
                    );

                frame_traces.push(trace.clone());

                // Store traces from first year for initial plot
                if frame_index == 0 {
                    initial_traces.push(trace);
                }
            }
        }

        // Create layout for this frame
        let frame_layout = Layout::new()
            .title(Title::with_text(format!(
                "GDP vs. Life Expectancy ({year})"
            )))
            .x_axis(
                Axis::new()
                    .title(Title::with_text("gdpPercap"))
                    .type_(plotly::layout::AxisType::Log),
            )
            .y_axis(
                Axis::new()
                    .title(Title::with_text("lifeExp"))
                    .range(vec![30.0, 85.0]), // Fixed range for Life Expectancy
            );

        // Add frame with all traces for this year
        plot.add_frame(
            Frame::new()
                .name(format!("frame{frame_index}"))
                .data(frame_traces)
                .layout(frame_layout),
        );
    }

    // Add initial traces to the plot (all traces from first year)
    for trace in initial_traces {
        plot.add_trace(trace);
    }

    // Create animation configuration for playing all frames
    let play_animation = Animation::all_frames().options(
        AnimationOptions::new()
            .mode(AnimationMode::Immediate)
            .frame(FrameSettings::new().duration(500).redraw(false))
            .transition(TransitionSettings::new().duration(300))
            .fromcurrent(true),
    );

    let play_button = ButtonBuilder::new()
        .label("Play")
        .animation(play_animation)
        .build()
        .unwrap();

    let pause_animation = Animation::pause();

    let pause_button = ButtonBuilder::new()
        .label("Pause")
        .animation(pause_animation)
        .build()
        .unwrap();

    let updatemenu = UpdateMenu::new()
        .ty(UpdateMenuType::Buttons)
        .direction(UpdateMenuDirection::Right)
        .buttons(vec![play_button, pause_button])
        .x(0.1)
        .y(1.15)
        .show_active(true)
        .visible(true);

    // Create slider steps for each year
    let mut slider_steps = Vec::new();
    for (i, &year) in years.iter().enumerate() {
        let frame_animation = Animation::frames(vec![format!("frame{}", i)]).options(
            AnimationOptions::new()
                .mode(AnimationMode::Immediate)
                .frame(FrameSettings::new().duration(300).redraw(false))
                .transition(TransitionSettings::new().duration(300)),
        );
        let step = SliderStepBuilder::new()
            .label(year.to_string())
            .value(year)
            .animation(frame_animation)
            .build()
            .unwrap();
        slider_steps.push(step);
    }

    let slider = Slider::new()
        .pad(Pad::new(55, 0, 130))
        .current_value(
            SliderCurrentValue::new()
                .visible(true)
                .prefix("Year: ")
                .x_anchor(SliderCurrentValueXAnchor::Right)
                .font(Font::new().size(20).color("rgb(102, 102, 102)")),
        )
        .steps(slider_steps);

    // Set the layout with initial title, buttons, and slider
    let layout = Layout::new()
        .title(Title::with_text(format!(
            "GDP vs. Life Expectancy ({}) - Click 'Play' to animate",
            years[0]
        )))
        .x_axis(
            Axis::new()
                .title(Title::with_text("gdpPercap"))
                .type_(plotly::layout::AxisType::Log),
        )
        .y_axis(
            Axis::new()
                .title(Title::with_text("lifeExp"))
                .range(vec![30.0, 85.0]), // Fixed range for Life Expectancy
        )
        .update_menus(vec![updatemenu])
        .sliders(vec![slider]);

    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: gdp_life_expectancy_animation_example

// ANCHOR: animation_randomize_example
/// Animation example based on the Plotly.js "Randomize" animation.
/// This demonstrates the new builder API for animation configuration.
fn animation_randomize_example(show: bool, file_name: &str) {
    use plotly::{
        layout::update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuDirection, UpdateMenuType},
        layout::{
            Animation, AnimationEasing, AnimationMode, Frame, FrameSettings, TransitionSettings,
        },
        Plot, Scatter,
    };

    // Initial data
    let x = vec![1, 2, 3];
    let y0 = vec![0.0, 0.5, 1.0];
    let y1 = vec![0.2, 0.8, 0.3];
    let y2 = vec![0.9, 0.1, 0.7];

    let mut plot = Plot::new();
    let base =
        Scatter::new(x.clone(), y0.clone()).line(plotly::common::Line::new().simplify(false));
    plot.add_trace(base.clone());

    // Add frames with different y-values and auto-adjusting layouts
    let mut trace0 = plotly::Traces::new();
    trace0.push(base);

    let mut trace1 = plotly::Traces::new();
    trace1.push(Scatter::new(x.clone(), y1.clone()));

    let mut trace2 = plotly::Traces::new();
    trace2.push(Scatter::new(x.clone(), y2.clone()));

    let animation = Animation::new().options(
        AnimationOptions::new()
            .mode(AnimationMode::Immediate)
            .frame(FrameSettings::new().duration(500))
            .transition(
                TransitionSettings::new()
                    .duration(500)
                    .easing(AnimationEasing::CubicInOut),
            ),
    );

    let layout0 = plotly::Layout::new()
        .title(Title::with_text("First frame"))
        .y_axis(plotly::layout::Axis::new().range(vec![0.0, 1.0]));
    let layout1 = plotly::Layout::new()
        .title(Title::with_text("Second frame"))
        .y_axis(plotly::layout::Axis::new().range(vec![0.0, 1.0]));
    let layout2 = plotly::Layout::new()
        .title(Title::with_text("Third frame"))
        .y_axis(plotly::layout::Axis::new().range(vec![0.0, 1.0]));

    // Add frames using the new API
    plot.add_frame(Frame::new().name("frame0").data(trace0).layout(layout0))
        .add_frame(Frame::new().name("frame1").data(trace1).layout(layout1))
        .add_frame(Frame::new().name("frame2").data(trace2).layout(layout2));

    let randomize_button = ButtonBuilder::new()
        .label("Animate")
        .animation(animation)
        .build()
        .unwrap();

    let updatemenu = UpdateMenu::new()
        .ty(UpdateMenuType::Buttons)
        .direction(UpdateMenuDirection::Right)
        .buttons(vec![randomize_button])
        .x(0.1)
        .y(1.15)
        .show_active(true)
        .visible(true);

    plot.set_layout(
        Layout::new()
            .title("Animation Example - Click 'Animate'")
            .y_axis(Axis::new().title("Y Axis").range(vec![0.0, 1.0]))
            .update_menus(vec![updatemenu]),
    );

    let path = plotly_utils::write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: animation_randomize_example

fn main() {
    // Change false to true on any of these lines to display the example.
    bar_plot_with_dropdown_for_different_data(false, "bar_plot");
    heat_map_with_modifiable_colorscale(false, "heat_map");
    bar_chart_with_modifiable_bar_mode(false, "bar_chart");

    // Silder examples
    bar_chart_with_slider_customization(false, "bar_chart_with_slider_customization");
    sinusoidal_slider_example(false, "sinusoidal_slider_example");
    gdp_life_expectancy_slider_example(false, "gdp_life_expectancy_slider_example");
    // Animation examples
    animation_randomize_example(false, "animation_randomize_example");
    gdp_life_expectancy_animation_example(false, "gdp_life_expectancy_animation_example");
}
