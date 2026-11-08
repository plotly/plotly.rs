mod layout;
mod plotly_chart;

use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Configuration, Layout, Plot, Scatter, Scatter3D,
};
use proa_core::{DataLoader, WebContext, WebRenderSync, WriteError};
use proa_framework_core::metadata::{
    resolve_metadata_stack, CascadeValue, Metadata, ResolvedMetadata,
};
use proa_macros::html_sync;

use layout::{LayoutMetadata, PlotLayout};
use plotly_chart::PlotlyChart;

const TITLE: &str = "Plotly-rs Multiple Plots";
const HEADING: &str = "Multiple Plotly plots on the same HTML page";
const DESCRIPTION: &str = "Three Plotly figures composed into a Proa page.";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let charts = [
        Chart::new("scatter_1", "Sine wave", &first_plot())?,
        Chart::new("scatter_2", "Colored markers", &second_plot())?,
        Chart::new("scatter_3_3d", "3D scatter", &third_plot())?,
    ];
    let metadata = resolve_metadata_stack(&[&LayoutMetadata, &PageMetadata]).unwrap();
    let html = render_document(&charts, &metadata)?;

    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", html)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}

struct PageMetadata;

impl Metadata for PageMetadata {
    fn title(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set(TITLE)
    }

    fn description(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set(DESCRIPTION)
    }
}

struct Chart {
    title: &'static str,
    plot: PlotlyChart<'static>,
}

impl Chart {
    fn new(id: &'static str, title: &'static str, plot: &Plot) -> Result<Self, serde_json::Error> {
        Ok(Self {
            title,
            plot: PlotlyChart::new(id, plot)?,
        })
    }
}

struct ChartCard<'a> {
    chart: &'a Chart,
    class: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for ChartCard<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <section data-chart-card class={self.class}>
                <h2>{self.chart.title}</h2>
                {&self.chart.plot}
            </section>
        }
        .render(cx)
    }
}

struct MultiplePlotsPage<'a> {
    charts: &'a [Chart],
    class: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for MultiplePlotsPage<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <main data-multiple-plots-page class={self.class}>
                <h1>{HEADING}</h1>
                <div class="charts">
                    {for chart in self.charts {
                        ChartCard { chart, class: None }
                    }}
                </div>
            </main>
        }
        .render(cx)
    }
}

fn render_document(charts: &[Chart], metadata: &ResolvedMetadata) -> Result<String, WriteError> {
    let mut cx = WebContext::new();
    PlotLayout {
        metadata,
        class: None,
        children: Some(MultiplePlotsPage {
            charts,
            class: None,
        }),
    }
    .render(&mut cx)?;
    Ok(cx.into_output().into_string())
}

// These construct the same figures as customization/multiple_plots_example.
// They return Plot values, leaving HTML and library loading to the components.
fn first_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(t, y).mode(Mode::Markers));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

fn second_plot() -> Plot {
    let trace = Scatter::new(vec![1, 2, 3, 4], vec![10, 11, 12, 13])
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size_array(vec![40, 60, 80, 100])
                .color_array(vec![
                    NamedColor::Red,
                    NamedColor::Blue,
                    NamedColor::Cyan,
                    NamedColor::OrangeRed,
                ]),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

fn third_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let z = t.iter().map(|x| x.cos()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter3D::new(t, y, z).mode(Mode::Markers));
    plot.set_layout(Layout::new().title(Title::with_text("Scatter3d")));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

#[cfg(test)]
mod tests {
    use super::*;
    use proa_framework_core::metadata::TitleOnly;

    #[test]
    fn page_metadata_overrides_layout_defaults_and_omitted_fields_inherit() {
        static OTHER_PAGE: TitleOnly = TitleOnly::new("Other <page> & report");
        let metadata = resolve_metadata_stack(&[&LayoutMetadata, &OTHER_PAGE]).unwrap();
        let html = render_document(&[], &metadata).unwrap();
        let defaults = resolve_metadata_stack(&[&LayoutMetadata]).unwrap();
        assert_eq!(metadata.description, defaults.description);
        assert_eq!(html.matches("<title>").count(), 1);
        assert!(html.contains("Other &lt;page&gt; &amp; report"));
    }

    #[test]
    fn document_composes_each_chart_with_its_own_data_and_one_library() {
        let plots = [first_plot(), second_plot()];
        let charts = [
            Chart::new("first", "<First & second>", &plots[0]).unwrap(),
            Chart::new("other", "Other", &plots[1]).unwrap(),
        ];
        let metadata = resolve_metadata_stack(&[&LayoutMetadata, &PageMetadata]).unwrap();
        let html = render_document(&charts, &metadata).unwrap();
        assert_eq!(html.matches("src=\"https://cdn.plot.ly/").count(), 1);
        assert_eq!(html.matches("data-chart-card>").count(), charts.len());
        assert!(html.contains("&lt;First &amp; second&gt;"));
        let seeds: Vec<_> = html.split("data-plotly-seed>").skip(1).collect();
        assert_eq!(seeds.len(), charts.len());
        for (seed, plot) in seeds.iter().zip(&plots) {
            let json = seed.split("</template>").next().unwrap();
            let decoded = html_escape::decode_html_entities(json);
            let actual: serde_json::Value = serde_json::from_str(&decoded).unwrap();
            let expected: serde_json::Value = serde_json::from_str(&plot.to_json()).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
