use proa_core::{
    text::StaticRaw, ComponentRole, DataLoader, HeadConfig, HeadMetadata, SyncHeadOperation,
    WebContext, WebRenderSync, WriteError,
};
use proa_framework_core::{
    finalize_route_response_sync, FrameworkBody, FrameworkError, FrameworkRequestContext,
    IslandManifest, RouteFinalizeMode, RouteResponse,
};
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

pub struct PlotLayout<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}

impl<L: DataLoader, C: WebRenderSync<L>> WebRenderSync<L> for PlotLayout<C> {
    const ROLE: ComponentRole = ComponentRole::LAYOUT;

    fn render_head(&self, cx: &mut WebContext<L>) -> impl SyncHeadOperation {
        cx.head_mut().metadata(HeadMetadata {
            title: "Plotly.rs".into(),
            description: "Interactive Plotly figures.".into(),
            ..Default::default()
        })
    }

    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <!DOCTYPE html>
            <html lang="en" data-plot-layout>
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    {cx.head()}
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

// A static-file export uses the existing managed response pipeline so layout
// and Page head hooks run before the completed HTML is written to disk.
pub fn render_page(
    page: impl WebRenderSync + Send + 'static,
) -> Result<impl AsRef<[u8]>, FrameworkError> {
    let response = RouteResponse::ssr(PlotLayout {
        class: None,
        children: Some(page),
    })
    .with_head(HeadConfig::default());
    let manifest = IslandManifest {
        islands: Default::default(),
        assets: Default::default(),
        import_map: Default::default(),
        ssr_bundle: None,
    };
    let response = finalize_route_response_sync(
        response,
        &manifest,
        &FrameworkRequestContext::default(),
        RouteFinalizeMode::Document,
    )?;
    match response.body {
        FrameworkBody::Full(bytes) => Ok(bytes),
        _ => Err(FrameworkError::Invariant(
            "expected a buffered HTML document",
        )),
    }
}
