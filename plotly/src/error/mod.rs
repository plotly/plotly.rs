use thiserror::Error;

const DEFAULT_HTML_APP_NOT_FOUND: &str = r#"Could not find default application for HTML files.
Consider using the `to_html` method to save the plot instead. If using the `kaleido` feature the
`save` method can be used to produce a static image in one of the following formats:
- ImageFormat::PNG
- ImageFormat::JPEG
- ImageFormat::WEBP
- ImageFormat::SVG
- ImageFormat::PDF
- ImageFormat::EPS

used as follows:
let plot = Plot::new();
...
let width = 1024;
let height = 680;
let scale = 1.0;
plot.save("filename", ImageFormat::PNG, width, height, scale);

See https://igiagkiozis.github.io/plotly/content/getting_started.html for further details.
"#;

#[derive(Error, Debug)]
pub enum Error {
    #[error("...")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to convert path to string: {0}")]
    PathSerializationError(std::path::PathBuf),

    #[cfg(feature = "kaleido")]
    #[error(transparent)]
    PlotlyKaleidoError {
        #[from]
        source: plotly_kaleido::error::Error,
    },

    #[error(transparent)]
    VarError {
        #[from]
        source: std::env::VarError,
    },

    #[error("{msg}")]
    NoDefaultAppError { msg: String, source: std::io::Error },

    #[error(transparent)]
    AskamaError {
        #[from]
        source: askama::Error,
    },

    #[error(transparent)]
    SerdeError {
        #[from]
        source: serde_json::Error,
    },

    #[error(transparent)]
    TraceError {
        #[from]
        source: TraceError,
    },
}

impl Error {
    pub fn new_default_app_error(source: std::io::Error) -> Self {
        Error::NoDefaultAppError {
            msg: DEFAULT_HTML_APP_NOT_FOUND.into(),
            source,
        }
    }
}

#[derive(Error, Debug)]
pub enum TraceError {
    #[error("{0}")]
    CustomError(String),

    #[error(transparent)]
    SerdeError {
        #[from]
        source: serde_json::Error,
    },
}
