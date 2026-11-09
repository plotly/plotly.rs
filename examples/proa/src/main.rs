mod chart;
mod layout;
mod page;
mod plots;

use axum::{http::StatusCode, routing::get, Router};
use proa_core::HeadConfig;
use proa_framework_axum::{
    FrameworkBuilder, IslandManifest, RouteResponse, DEFAULT_GRACEFUL_SHUTDOWN_TIMEOUT,
};

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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    println!("Open http://127.0.0.1:3001 to view the plots. Press Ctrl-C to stop.");
    proa_framework_axum::serve(listener, app, DEFAULT_GRACEFUL_SHUTDOWN_TIMEOUT).await?;
    Ok(())
}

async fn index() -> Result<RouteResponse, StatusCode> {
    let page = MultiplePlotsPage::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(RouteResponse::ssr(PlotLayout {
        class: None,
        children: Some(page),
    }))
}
