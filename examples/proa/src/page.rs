use plotly::Plot;
use proa_core::{
    ComponentRole, DataLoader, HeadMetadata, SyncHeadOperation, WebContext, WebRenderSync,
    WriteError,
};
use proa_macros::html_sync;

use crate::plotly_chart::PlotlyChart;

const TITLE: &str = "Plotly-rs Multiple Plots";
const HEADING: &str = "Multiple Plotly plots on the same HTML page";
const DESCRIPTION: &str = "Three Plotly figures composed into a Proa page.";

pub struct Chart {
    title: &'static str,
    plot: PlotlyChart<'static>,
}

impl Chart {
    pub fn new(
        id: &'static str,
        title: &'static str,
        plot: &Plot,
    ) -> Result<Self, serde_json::Error> {
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
    pub fn new(charts: Vec<Chart>) -> Self {
        Self {
            charts,
            metadata: HeadMetadata {
                title: TITLE.into(),
                description: DESCRIPTION.into(),
                ..Default::default()
            },
            class: None,
        }
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

#[cfg(test)]
mod tests {
    use proa_core::HeadConfig;
    use proa_framework_core::render_sync_html_document;

    use super::*;
    use crate::{first_plot, layout::PlotLayout, second_plot};

    #[test]
    fn document_composes_each_chart_with_its_own_data_and_one_library() {
        let plots = [first_plot(), second_plot()];
        let charts = vec![
            Chart::new("first", "<First & second>", &plots[0]).unwrap(),
            Chart::new("other", "Other", &plots[1]).unwrap(),
        ];
        let count = charts.len();
        let mut page = MultiplePlotsPage::new(charts);
        page.metadata = HeadMetadata {
            title: "Other <page> & report".into(),
            ..Default::default()
        };
        let bytes = render_sync_html_document(
            PlotLayout {
                class: None,
                children: Some(page),
            },
            HeadConfig::default(),
        )
        .unwrap();
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
