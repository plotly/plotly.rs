#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;

use csv::ReaderBuilder;
use plotly::layout::themes::BuiltinTheme;
use plotly::{
    common::{Marker, Mode, Title},
    layout::Layout,
    Plot, Scatter,
};
use plotly_utils::write_example_to_html;

// Distinct types for Gapminder data components
type GdpPerCapita = Vec<f64>;
type LifeExpectancy = Vec<f64>;
type Population = Vec<f64>;
type Continents = Vec<String>;
type Countries = Vec<String>;

#[derive(Debug)]
struct GapminderData {
    gdp_per_capita: GdpPerCapita,
    life_expectancy: LifeExpectancy,
    population: Population,
    continents: Continents,
    countries: Countries,
}

#[derive(Debug)]
struct ContinentData {
    gdp_per_capita: Vec<f64>,
    life_expectancy: Vec<f64>,
    population: Vec<f64>,
    countries: Vec<String>,
}

impl GapminderData {
    /// Get data for a specific continent
    fn get_continent_data(&self, continent: &str) -> Option<ContinentData> {
        let indices: Vec<usize> = self
            .continents
            .iter()
            .enumerate()
            .filter(|(_, c)| c.as_str() == continent)
            .map(|(idx, _)| idx)
            .collect();

        if indices.is_empty() {
            return None;
        }

        let gdp: Vec<f64> = indices
            .iter()
            .map(|&idx| self.gdp_per_capita[idx])
            .collect();
        let life_exp: Vec<f64> = indices
            .iter()
            .map(|&idx| self.life_expectancy[idx])
            .collect();
        let pop: Vec<f64> = indices.iter().map(|&idx| self.population[idx]).collect();
        let countries: Vec<String> = indices
            .iter()
            .map(|&idx| self.countries[idx].clone())
            .collect();

        Some(ContinentData {
            gdp_per_capita: gdp,
            life_expectancy: life_exp,
            population: pop,
            countries,
        })
    }
}

// Read Gapminder 2007 data from CSV
fn read_gapminder_data_from_csv() -> GapminderData {
    let file = File::open("assets/gapminder2007.csv").expect("Cannot open gapminder2007.csv");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));
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
    GapminderData {
        gdp_per_capita,
        life_expectancy,
        population,
        continents,
        countries,
    }
}

// ANCHOR: gapminder_plot_plotly
fn gapminder_plot_plotly(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::Default, show, file_name);
}
// ANCHOR_END: gapminder_plot_plotly

// ANCHOR: gapminder_plot_plotly_white
fn gapminder_plot_plotly_white(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::PlotlyWhite, show, file_name);
}
// ANCHOR_END: gapminder_plot_plotly_white

// ANCHOR: gapminder_plot_plotly_dark
fn gapminder_plot_plotly_dark(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::PlotlyDark, show, file_name);
}
// ANCHOR_END: gapminder_plot_plotly_dark

// ANCHOR: gapminder_plot_seaborn
fn gapminder_plot_seaborn(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::Seaborn, show, file_name);
}
// ANCHOR_END: gapminder_plot_seaborn

// ANCHOR: gapminder_plot_matplotlib
fn gapminder_plot_matplotlib(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::Matplotlib, show, file_name);
}
// ANCHOR_END: gapminder_plot_matplotlib

// ANCHOR: gapminder_plot_plotnine
fn gapminder_plot_plotnine(show: bool, file_name: &str) {
    gapminder_plot_with_theme(BuiltinTheme::Plotnine, show, file_name);
}
// ANCHOR_END: gapminder_plot_plotnine

fn gapminder_plot_with_theme(theme: BuiltinTheme, show: bool, file_name: &str) {
    let data = read_gapminder_data_from_csv();
    let mut plot = Plot::new();
    let continent_colors = vec![
        ("Asia", "#1f77b4"),
        ("Europe", "#ff7f0e"),
        ("Africa", "#2ca02c"),
        ("Americas", "#d62728"),
        ("Oceania", "#9467bd"),
    ];
    for (continent, color) in &continent_colors {
        if let Some(continent_data) = data.get_continent_data(continent) {
            let trace = Scatter::new(
                continent_data.gdp_per_capita,
                continent_data.life_expectancy,
            )
            .mode(Mode::Markers)
            .name(continent)
            .text_array(continent_data.countries)
            .marker(
                Marker::new()
                    .size_array(
                        continent_data
                            .population
                            .iter()
                            .map(|&p| ((p / 1_000_000.0).min(60.0)) as usize)
                            .collect(),
                    )
                    .color(*color)
                    .opacity(0.6)
                    .line(plotly::common::Line::new().width(1.0).color("#FFFFFF")),
            );
            plot.add_trace(trace);
        }
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
    let path = write_example_to_html(&plot, file_name);
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

fn main() {
    gapminder_plot_plotly(false, "gapminder_plotly");
    gapminder_plot_plotly_white(false, "gapminder_plotly_white");
    gapminder_plot_plotly_dark(false, "gapminder_plotly_dark");
    gapminder_plot_seaborn(false, "gapminder_seaborn");
    gapminder_plot_matplotlib(false, "gapminder_matplotlib");
    gapminder_plot_plotnine(false, "gapminder_plotnine");
}
