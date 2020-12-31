use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not find kaleido executable in path: {0}")]
    ExecutableNotFound(std::path::PathBuf),

    #[error("Failed to convert path to string: {0}")]
    PathSerializationError(std::path::PathBuf),

    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("Could not connect to kaleido stdin")]
    StdinError,

    #[error("Failed to get parent directory for process")]
    ParentDirectoryFailed,

    #[error(transparent)]
    Base64DecodeError {
        #[from]
        source: base64::DecodeError,
    },

    #[error(transparent)]
    SerdeError {
        #[from]
        source: serde_json::Error,
    },

    #[error(transparent)]
    VarError {
        #[from]
        source: std::env::VarError,
    },
}
