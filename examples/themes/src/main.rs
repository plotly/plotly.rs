#![allow(dead_code)]

use plotly::{
    common::{Marker, Mode, Title},
    layout::Layout,
    Plot, Scatter,
};

use plotly::layout::themes::BuiltinTheme;

use std::fs::File;
use std::io::BufReader;

use csv::ReaderBuilder;

// Read Gapminder 2007 data from CSV
fn read_gapminder_data_from_csv() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<String>, Vec<String>) {
    let file = File::open("assets/gapminder2007.csv").expect("Cannot open gapminder2007.csv");
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
    let mut gdp_per_capita = Vec::new();
    let mut life_expectancy = Vec::new();
    let mut population = Vec::new();
    let mut continents = Vec::new();
    let mut countries = Vec::new();
    for result in rdr.records() {
        let record = result.expect("CSV record error");
        countries.push(record[0].to_string());
        population.push(record[1].parse::<f64>().unwrap_or(0.0));
        continents.push(record[2].to_string());
        life_expectancy.push(record[3].parse::<f64>().unwrap_or(0.0));
        gdp_per_capita.push(record[4].parse::<f64>().unwrap_or(0.0));
    }
    (gdp_per_capita, life_expectancy, population, continents, countries)
}

fn create_gapminder_scatter_plot(theme: BuiltinTheme, show: bool) {
    let (gdp, life_exp, pop, continents, countries) = read_gapminder_data_from_csv();

    let mut plot = Plot::new();
    let continent_colors = vec![
        ("Asia", "#1f77b4"),
        ("Europe", "#ff7f0e"),
        ("Africa", "#2ca02c"),
        ("Americas", "#d62728"),
        ("Oceania", "#9467bd"),
    ];

    for (continent, color) in &continent_colors {
        let indices: Vec<usize> = continents
            .iter()
            .enumerate()
            .filter(|(_, c)| c == continent)
            .map(|(idx, _)| idx)
            .collect();
        if indices.is_empty() {
            continue;
        }
        let continent_gdp: Vec<f64> = indices.iter().map(|&idx| gdp[idx]).collect();
        let continent_life: Vec<f64> = indices.iter().map(|&idx| life_exp[idx]).collect();
        let continent_pop: Vec<f64> = indices.iter().map(|&idx| pop[idx]).collect();
        let continent_countries: Vec<String> = indices.iter().map(|&idx| countries[idx].clone()).collect();
        let trace = Scatter::new(continent_gdp, continent_life)
            .mode(Mode::Markers)
            .name(continent.to_string())
            .text_array(continent_countries)
            .marker(
                Marker::new()
                    .size_array(
                        continent_pop
                            .iter()
                            .map(|&p| ((p / 1_000_000.0).min(60.0)) as usize)
                            .collect(),
                    )
                    .color(*color)
                    .opacity(0.6)
                    .line(plotly::common::Line::new().width(1.0).color("white")),
            );
        plot.add_trace(trace);
    }

    let theme_template = theme.build();
    plot.set_layout(
        Layout::new()
            .template(theme_template)
            .title(Title::from(format!(
                "Gapminder 2007: '{}' theme",
                theme_name(theme)
            )))
            .x_axis(
                plotly::layout::Axis::new()
                    .title("GDP per capita (log scale)")
                    .type_(plotly::layout::AxisType::Log),
            )
            .y_axis(plotly::layout::Axis::new().title("Life Expectancy"))
            .width(800)
            .height(600)
            .show_legend(true),
    );

    let path = write_example_to_html(
        &plot,
        &format!("gapminder_{}", theme_name(theme).to_lowercase()),
    );
    if show {
        plot.show_html(path);
    }
}

fn theme_name(theme: BuiltinTheme) -> &'static str {
    match theme {
        BuiltinTheme::Default => "plotly",
        BuiltinTheme::PlotlyWhite => "plotly_white",
        BuiltinTheme::PlotlyDark => "plotly_dark",
        BuiltinTheme::Seaborn => "seaborn",
        BuiltinTheme::SeabornWhitegrid => "seaborn_whitegrid",
        BuiltinTheme::SeabornDark => "seaborn_dark",
        BuiltinTheme::Matplotlib => "matplotlib",
        BuiltinTheme::Plotnine => "plotnine",
    }
}

fn write_example_to_html(plot: &Plot, name: &str) -> String {
    std::fs::create_dir_all("./output").unwrap();
    // Write inline HTML
    let html = plot.to_inline_html(Some(name));
    let path = format!("./output/inline_{}.html", name);
    std::fs::write(path, html).unwrap();
    // Write standalone HTML
    let path = format!("./output/{}.html", name);
    plot.write_html(&path);
    path
}

fn main() {
    // Create Gapminder-style plots with different themes, matching the Plotly documentation
    // Based on: https://plotly.com/python/templates/

    // Create plots for each theme
    let themes = vec![
        BuiltinTheme::Default,
        BuiltinTheme::PlotlyWhite,
        BuiltinTheme::PlotlyDark,
        BuiltinTheme::Seaborn,
        BuiltinTheme::Matplotlib,
        BuiltinTheme::Plotnine,
    ];

    for theme in themes {
        create_gapminder_scatter_plot(theme, false);
    }

    // Show one example (the default theme)
    create_gapminder_scatter_plot(BuiltinTheme::Default, true);
}
