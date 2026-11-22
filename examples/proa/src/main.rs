mod chart;
mod layout;
mod page;
mod plots;

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use proa_core::HeadConfig;
use proa_framework_axum::{FrameworkBuilder, IslandManifest, RouteResponse};
use tower::ServiceExt;

use crate::{layout::PlotLayout, page::MultiplePlotsPage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // No React islands are used by this example.
    let manifest = IslandManifest {
        islands: Default::default(),
        assets: Default::default(),
        import_map: Default::default(),
        ssr_bundle: None,
    };
    let app = FrameworkBuilder::new(manifest)
        .with_head(HeadConfig::default())
        .with_dynamic_routes(Router::new().route("/", get(index)))
        .build();

    // Render one request in-process; no listening server is needed.
    let response = app.oneshot(Request::get("/").body(Body::empty())?).await?;
    if !response.status().is_success() {
        return Err(format!("Page rendering failed: {}", response.status()).into());
    }
    let html = to_bytes(response.into_body(), usize::MAX).await?;
    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", html)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}

async fn index() -> Result<RouteResponse, StatusCode> {
    let page = MultiplePlotsPage::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(RouteResponse::ssr(PlotLayout {
        class: None,
        children: Some(page),
    }))
}
