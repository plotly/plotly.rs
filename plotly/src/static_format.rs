#![cfg(not(feature = "kaleido"))]
use serde::Serialize;

/// Image format for static image export.
/// Redifined from `plotly_static` package to be used when `kaleido` feature is
/// not enabled
#[derive(Debug, Clone, Serialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    SVG,
    PDF,
    EPS,
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PNG => "png",
                Self::JPEG => "jpeg",
                Self::WEBP => "webp",
                Self::SVG => "svg",
                Self::PDF => "pdf",
                Self::EPS => "eps",
            }
        )
    }
}
