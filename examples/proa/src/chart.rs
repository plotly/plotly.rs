//! Prepare figure JSON once and compose charts through a shared card component.

use plotly::Plot;
use proa_core::{DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

pub struct Chart {
    id: &'static str,
    title: &'static str,
    json: String,
    pub class: Option<&'static str>,
}

impl Chart {
    pub fn new(
        id: &'static str,
        title: &'static str,
        plot: &Plot,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id,
            title,
            json: serde_json::to_string(plot)?,
            class: None,
        })
    }
}

impl<L: DataLoader> WebRenderSync<L> for &Chart {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div id={self.id} data-plotly-chart class={self.class}></div>
            <template data-plotly-seed>{self.json.as_str()}</template>
        }
        .render(cx)
    }
}

pub struct ChartCard<'a> {
    pub chart: &'a Chart,
    pub class: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for ChartCard<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <section data-chart-card class={self.class}>
                <h2>{self.chart.title}</h2>
                {self.chart}
            </section>
        }
        .render(cx)
    }
}
