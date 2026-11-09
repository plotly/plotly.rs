use proa_core::{
    ComponentRole, DataLoader, HeadMetadata, SyncHeadOperation, WebContext, WebRenderSync,
    WriteError,
};
use proa_macros::html_sync;

use crate::{
    chart::{Chart, ChartCard},
    plots::{first_plot, second_plot, third_plot},
};

const TITLE: &str = "Plotly-rs Multiple Plots";
const HEADING: &str = "Multiple Plotly plots on the same HTML page";
const DESCRIPTION: &str = "Three Plotly figures composed into a Proa page.";

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
