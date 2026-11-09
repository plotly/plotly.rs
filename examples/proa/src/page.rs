use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Configuration, Layout, Plot, Scatter, Scatter3D,
};
use proa_core::{
    ComponentRole, DataLoader, HeadMetadata, SyncHeadOperation, WebContext, WebRenderSync,
    WriteError,
};
use proa_macros::html_sync;

use crate::plotly_chart::PlotlyChart;

const TITLE: &str = "Plotly-rs Multiple Plots";
const HEADING: &str = "Multiple Plotly plots on the same HTML page";
const DESCRIPTION: &str = "Three Plotly figures composed into a Proa page.";

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

pub struct MultiplePlotsPage {
    charts: Vec<Chart>,
    metadata: HeadMetadata,
    class: Option<&'static str>,
}

impl MultiplePlotsPage {
    pub fn new() -> Result<Self, serde_json::Error> {
        let charts = vec![
            Chart::new("scatter_1", "Sine wave", &first_plot())?,
            Chart::new("scatter_2", "Colored markers", &second_plot())?,
            Chart::new("scatter_3_3d", "3D scatter", &third_plot())?,
        ];
        Ok(Self {
            charts,
            metadata: HeadMetadata {
                title: TITLE.into(),
                description: DESCRIPTION.into(),
                ..Default::default()
            },
            class: None,
        })
    }
}

impl<L: DataLoader> WebRenderSync<L> for MultiplePlotsPage {
    const ROLE: ComponentRole = ComponentRole::PAGE;

    fn render_head(&self, cx: &mut WebContext<L>) -> impl SyncHeadOperation {
        cx.head_mut().metadata(self.metadata.clone())
    }

    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <main data-multiple-plots-page class={self.class}>
                <h1>{HEADING}</h1>
                <div class="charts">
                    {for chart in &self.charts {
                        ChartCard { chart, class: None }
                    }}
                </div>
            </main>
        }
        .render(cx)
    }
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
    use proa_framework_core::WebContextDocumentExt;

    use super::*;
    use crate::layout::PlotLayout;

    #[test]
    fn document_composes_each_chart_with_its_own_data_and_one_library() {
        let plots = [first_plot(), second_plot()];
        let charts = vec![
            Chart::new("first", "<First & second>", &plots[0]).unwrap(),
            Chart::new("other", "Other", &plots[1]).unwrap(),
        ];
        let count = charts.len();
        let page = MultiplePlotsPage {
            charts,
            metadata: HeadMetadata {
                title: "Other <page> & report".into(),
                ..Default::default()
            },
            class: None,
        };
        let mut cx = WebContext::new();
        cx.render(PlotLayout {
            class: None,
            children: Some(page),
        })
        .unwrap();
        let bytes = cx.finish().unwrap();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(html.matches("<title>").count(), 1);
        assert!(html.contains("<title>Other &lt;page&gt; &amp; report</title>"));
        assert_eq!(html.matches("name=\"description\"").count(), 1);
        assert_eq!(html.matches("src=\"https://cdn.plot.ly/").count(), 1);
        assert_eq!(html.matches("data-chart-card>").count(), count);
        assert!(html.contains("&lt;First &amp; second&gt;"));
        let seeds: Vec<_> = html.split("data-plotly-seed>").skip(1).collect();
        assert_eq!(seeds.len(), count);
        for (seed, plot) in seeds.iter().zip(&plots) {
            let json = seed.split("</template>").next().unwrap();
            let decoded = html_escape::decode_html_entities(json);
            let actual: serde_json::Value = serde_json::from_str(&decoded).unwrap();
            let expected: serde_json::Value = serde_json::from_str(&plot.to_json()).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
