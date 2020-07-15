pub use ndarray::iter::*;
pub use ndarray::prelude::*;
pub use ndarray::Array;

#[derive(PartialOrd, PartialEq)]
pub enum ArrayTraces {
    OverColumns,
    OverRows,
}
