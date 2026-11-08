use proa_core::{
    text::{raw_text_ok, StaticRaw},
    DataLoader, WebContext, WebRenderSync, WriteError,
};
use proa_framework_core::metadata::{CascadeValue, Metadata, ResolvedMetadata};
use proa_macros::html_sync;

// Minimal vendor bridge: RSJS does not expose Plotly.newPlot or its DOM-node
// API. Only this developer-owned script bypasses HTML escaping. Figure data
// uses normal Proa text escaping; Plotly owns interaction and resizing.
const PLOTLY_INIT: &str = r#"
await Promise.all(Array.from(
    document.querySelectorAll("template[data-plotly-seed]"),
    seed => Plotly.newPlot(seed.previousElementSibling, JSON.parse(seed.content.textContent))
));
"#;

pub struct LayoutMetadata;

impl Metadata for LayoutMetadata {
    fn title(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set("Plotly.rs")
    }

    fn description(&self) -> CascadeValue<&'static str> {
        CascadeValue::Set("Interactive Plotly figures.")
    }
}

pub struct PlotLayout<'a, C = ()> {
    pub metadata: &'a ResolvedMetadata,
    pub class: Option<&'static str>,
    pub children: Option<C>,
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
