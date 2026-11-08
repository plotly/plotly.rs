mod plotly_chart;

use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Configuration, Layout, Plot, Scatter, Scatter3D,
};
use proa_core::{
    text::{raw_text_ok, StaticRaw},
    DataLoader, WebContext, WebRenderSync, WriteError,
};
use proa_framework_core::metadata::{
    resolve_metadata_stack, CascadeValue, Metadata, ResolvedMetadata,
};
use proa_macros::html_sync;

use plotly_chart::PlotlyChart;

const TITLE: &str = "Plotly-rs Multiple Plots";
const HEADING: &str = "Multiple Plotly plots on the same HTML page";
const DESCRIPTION: &str = "Three Plotly figures composed into a Proa page.";

// Minimal vendor bridge: RSJS does not expose Plotly.newPlot or its DOM-node
// API. Only this developer-owned script bypasses HTML escaping; figure data
// goes through HtmlJson. Plotly owns interaction and resizing after this pass.
const PLOTLY_INIT: &str = r#"
await Promise.all(Array.from(
    document.querySelectorAll("template[data-plotly-seed]"),
    seed => Plotly.newPlot(seed.previousElementSibling, JSON.parse(seed.content.textContent))
));
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let charts = [
        Chart {
            id: "scatter_1",
            title: "Sine wave",
            plot: first_plot(),
        },
        Chart {
            id: "scatter_2",
            title: "Colored markers",
            plot: second_plot(),
        },
        Chart {
            id: "scatter_3_3d",
            title: "3D scatter",
            plot: third_plot(),
        },
    ];
    let metadata = resolve_metadata_stack(&[&LayoutMetadata, &PageMetadata]).unwrap();
    let html = render_document(&charts, &metadata)?;

    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", html)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}

struct LayoutMetadata;

impl Metadata for LayoutMetadata {
    fn title(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set("Plotly.rs")
    }

    fn description(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set("Interactive Plotly figures.")
    }
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

struct PlotLayout<'a, C = ()> {
    metadata: &'a ResolvedMetadata,
    class: Option<&'static str>,
    children: Option<C>,
}

impl<L: DataLoader, C: WebRenderSync<L>> WebRenderSync<L> for PlotLayout<'_, C> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let title = self.metadata.title.as_deref();
        html_sync! {
            <!DOCTYPE html>
            <html lang="en" data-plot-layout>
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>{raw_text_ok(title)}</title>
                    {if let Some(description) = self.metadata.description {
                        html_sync! { <meta name="description" content={description} /> }
                    }}
                    <style>
                        r#"
                        body { margin: 0; color: #172033; background: #f5f7fb; font-family: system-ui, sans-serif; }
                        main { max-width: 1200px; margin: 0 auto; padding: 24px; }
                        h1 { font-size: 1.75rem; }
                        .charts { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 24px; }
                        [data-chart-card] { min-width: 0; padding: 16px; background: white; border: 1px solid #dce2eb; border-radius: 12px; }
                        [data-chart-card] h2 { margin: 0 0 12px; font-size: 1.125rem; }
                        [data-plotly-chart] { height: 420px; }
                        [data-chart-card]:last-child { grid-column: 1 / -1; }
                        [data-chart-card]:last-child [data-plotly-chart] { height: clamp(360px, 65vw, 800px); }
                        @media (max-width: 700px) { .charts { grid-template-columns: 1fr; } main { padding: 12px; } }
                        "#
                    </style>
                    <script src="https://cdn.plot.ly/plotly-3.7.0.min.js"></script>
                </head>
                <body class={self.class}>
                    {self.children}
                    <noscript>"Enable JavaScript to display the interactive charts."</noscript>
                    <script type="module">{StaticRaw(PLOTLY_INIT)}</script>
                </body>
            </html>
        }
        .render(cx)
    }
}

struct Chart {
    id: &'static str,
    title: &'static str,
    plot: Plot,
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
                {PlotlyChart { id: self.chart.id, plot: &self.chart.plot, class: None }}
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
        let charts = [
            Chart {
                id: "first",
                title: "<First & second>",
                plot: first_plot(),
            },
            Chart {
                id: "other",
                title: "Other",
                plot: second_plot(),
            },
        ];
        let metadata = resolve_metadata_stack(&[&LayoutMetadata, &PageMetadata]).unwrap();
        let html = render_document(&charts, &metadata).unwrap();
        assert_eq!(html.matches("src=\"https://cdn.plot.ly/").count(), 1);
        assert_eq!(html.matches("data-chart-card>").count(), charts.len());
        assert!(html.contains("&lt;First &amp; second&gt;"));
        let seeds: Vec<_> = html.split("data-plotly-seed>").skip(1).collect();
        assert_eq!(seeds.len(), charts.len());
        for (seed, chart) in seeds.iter().zip(&charts) {
            let json = seed.split("</template>").next().unwrap();
            let actual: serde_json::Value = serde_json::from_str(json).unwrap();
            let expected: serde_json::Value = serde_json::from_str(&chart.plot.to_json()).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
