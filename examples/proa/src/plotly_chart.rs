//! Prepare figure JSON once; compose the chart by borrowing this component.

use plotly::Plot;
use proa_core::{DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

pub struct PlotlyChart<'a> {
    id: &'a str,
    json: String,
    pub class: Option<&'static str>,
}

impl<'a> PlotlyChart<'a> {
    pub fn new(id: &'a str, plot: &Plot) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id,
            json: serde_json::to_string(plot)?,
            class: None,
        })
    }
}

impl<L: DataLoader> WebRenderSync<L> for &PlotlyChart<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div id={self.id} data-plotly-chart class={self.class}></div>
            <template data-plotly-seed>{self.json.as_str()}</template>
        }
        .render(cx)
    }
}

#[cfg(test)]
mod tests {
    use plotly::Layout;

    use super::*;

    #[test]
    fn figure_data_round_trips_through_html_text_escaping() {
        let title = "</template></script><script>injected()</script><!-- & ' \\\" café 🦀";
        let mut plot = Plot::new();
        plot.set_layout(Layout::new().title(title));
        let chart = PlotlyChart::new("example", &plot).unwrap();
        let mut cx = WebContext::new();
        chart.render(&mut cx).unwrap();
        let html = cx.into_output().into_string();
        assert_eq!(html.matches("</template>").count(), 1);
        assert!(!html.contains("<script>"));
        let seed = html.split("data-plotly-seed>").nth(1).unwrap();
        let json = html_escape::decode_html_entities(seed.split("</template>").next().unwrap());
        let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(actual["layout"]["title"]["text"], title);
    }

    #[test]
    fn chart_ids_are_attributes_and_never_interpolated_into_javascript() {
        let chart = PlotlyChart::new("chart\"<&\\name", &Plot::new()).unwrap();
        let mut cx = WebContext::new();
        chart.render(&mut cx).unwrap();
        let html = cx.into_output().into_string();
        assert!(html.contains("&quot;"));
        assert!(!html.contains("id=\"chart\"<"));
        let seed = html.split("data-plotly-seed>").nth(1).unwrap();
        let json = html_escape::decode_html_entities(seed.split("</template>").next().unwrap());
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&json).unwrap()["data"],
            serde_json::json!([])
        );
    }
}
