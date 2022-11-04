#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix2};
use serde::Serialize;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;

pub fn owned_string_vector<S: AsRef<str>>(s: Vec<S>) -> Vec<String> {
    s.iter()
        .map(|x| x.as_ref().to_string())
        .collect::<Vec<String>>()
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumOrString {
    S(String),
    F(f64),
    I(i64),
    U(u64),
}

impl From<String> for NumOrString {
    fn from(item: String) -> Self {
        NumOrString::S(item)
    }
}

impl From<&String> for NumOrString {
    fn from(item: &String) -> Self {
        NumOrString::S(item.clone())
    }
}

impl From<&str> for NumOrString {
    fn from(item: &str) -> Self {
        NumOrString::S(item.to_string())
    }
}

impl From<f64> for NumOrString {
    fn from(item: f64) -> Self {
        NumOrString::F(item)
    }
}

impl From<f32> for NumOrString {
    fn from(item: f32) -> Self {
        NumOrString::F(item as f64)
    }
}

impl From<usize> for NumOrString {
    fn from(item: usize) -> Self {
        NumOrString::U(item as u64)
    }
}

impl From<u64> for NumOrString {
    fn from(item: u64) -> Self {
        NumOrString::U(item)
    }
}

impl From<u32> for NumOrString {
    fn from(item: u32) -> Self {
        NumOrString::U(item as u64)
    }
}

impl From<isize> for NumOrString {
    fn from(item: isize) -> Self {
        NumOrString::I(item as i64)
    }
}

impl From<i64> for NumOrString {
    fn from(item: i64) -> Self {
        NumOrString::I(item)
    }
}

impl From<i32> for NumOrString {
    fn from(item: i32) -> Self {
        NumOrString::I(item as i64)
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct NumOrStringCollection(Vec<NumOrString>);

impl<T> From<Vec<T>> for NumOrStringCollection
where
    T: Into<NumOrString> + Clone,
{
    fn from(items: Vec<T>) -> Self {
        let mut collection: Vec<NumOrString> = Vec::with_capacity(items.len());
        for item in items.iter().cloned() {
            collection.push(item.into());
        }
        Self(collection)
    }
}

#[cfg(feature = "plotly_ndarray")]
pub fn trace_vectors_from<T>(traces_matrix: Array<T, Ix2>, array_traces: ArrayTraces) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut traces: Vec<Vec<T>> = Vec::new();
    let dim_index = usize::from(array_traces == ArrayTraces::OverColumns);
    let traces_count = traces_matrix.shape()[dim_index];
    let get_trace = |index| {
        if array_traces == ArrayTraces::OverColumns {
            traces_matrix.column(index)
        } else {
            traces_matrix.row(index)
        }
    };
    for col in 0..traces_count {
        let trace_data: Vec<T> = get_trace(col).to_vec();
        traces.push(trace_data);
    }

    traces
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_num_or_string() {
        let x: NumOrString = "String".to_string().into();
        assert_eq!(x, NumOrString::S("String".to_string()));

        let x: NumOrString = (&"String".to_string()).into();
        assert_eq!(x, NumOrString::S("String".to_string()));

        let x: NumOrString = "&str".into();
        assert_eq!(x, NumOrString::S("&str".to_string()));

        let x: NumOrString = 100.0_f64.into();
        assert_eq!(x, NumOrString::F(100.));

        let x: NumOrString = 100.0_f32.into();
        assert_eq!(x, NumOrString::F(100.));

        let x: NumOrString = (-100_isize).into();
        assert_eq!(x, NumOrString::I(-100));

        let x: NumOrString = (-100_i64).into();
        assert_eq!(x, NumOrString::I(-100));

        let x: NumOrString = (-100_i32).into();
        assert_eq!(x, NumOrString::I(-100));

        let x: NumOrString = 100_usize.into();
        assert_eq!(x, NumOrString::U(100));

        let x: NumOrString = 100_u64.into();
        assert_eq!(x, NumOrString::U(100));

        let x: NumOrString = 100_u32.into();
        assert_eq!(x, NumOrString::U(100));
    }

    #[test]
    fn test_num_or_string_collection() {
        let x: NumOrStringCollection = vec!["&str"].into();
        let expected = NumOrStringCollection(vec![NumOrString::S("&str".to_string())]);
        assert_eq!(x, expected);

        let x: NumOrStringCollection = vec![1.].into();
        let expected = NumOrStringCollection(vec![NumOrString::F(1.)]);
        assert_eq!(x, expected);

        let x: NumOrStringCollection = vec![1_i32].into();
        let expected = NumOrStringCollection(vec![NumOrString::I(1)]);
        assert_eq!(x, expected);

        let x: NumOrStringCollection = vec![1_u32].into();
        let expected = NumOrStringCollection(vec![NumOrString::U(1)]);
        assert_eq!(x, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_num_or_string() {
        assert_eq!(to_value(NumOrString::S("&str".to_string())).unwrap(), json!("&str"));
        assert_eq!(to_value(NumOrString::F(100.)).unwrap(), json!(100.0));
        assert_eq!(to_value(NumOrString::I(-50)).unwrap(), json!(-50));
        assert_eq!(to_value(NumOrString::U(50)).unwrap(), json!(50));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_num_or_string_collection() {
        assert_eq!(to_value(NumOrStringCollection(vec![NumOrString::S("&str".to_string())])).unwrap(), json!(["&str"]));
        assert_eq!(to_value(NumOrStringCollection(vec![NumOrString::F(100.)])).unwrap(), json!([100.0]));
        assert_eq!(to_value(NumOrStringCollection(vec![NumOrString::I(-50)])).unwrap(), json!([-50]));
        assert_eq!(to_value(NumOrStringCollection(vec![NumOrString::U(50)])).unwrap(), json!([50]));
    }
}
