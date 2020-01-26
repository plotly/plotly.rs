use crate::charts::{ErrorData, Marker, Mode, PlotType};
use crate::TraceSerialize;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub x: Vec<X>,
    pub y: Vec<Y>,
    r#type: PlotType,
    pub visible: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showlegend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    pub mode: Mode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textposition: Option<Vec<String>>,
    pub marker: Marker,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_y: Option<ErrorData>,
}

impl<X, Y> Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub fn new(name: &str, x: Vec<X>, y: Vec<Y>) -> Box<Scatter<X, Y>> {
        Box::new(Scatter {
            x,
            y,
            r#type: PlotType::Scatter,
            visible: true,
            name: name.to_owned(),
            showlegend: None,
            legendgroup: None,
            opacity: None,
            mode: Mode::Lines,
            text: None,
            textposition: None,
            marker: Marker::new(),
            error_x: None,
            error_y: None,
        })
    }
}

impl<X, Y> TraceSerialize for Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub x: Vec<X>,
    pub y: Vec<Y>,
    r#type: PlotType,
    pub visible: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showlegend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textposition: Option<Vec<String>>,
    pub marker: Marker,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_y: Option<ErrorData>,
}

impl<X, Y> Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub fn new(name: &str, x: Vec<X>, y: Vec<Y>) -> Box<Bar<X, Y>> {
        Box::new(Bar {
            x,
            y,
            r#type: PlotType::Bar,
            visible: true,
            name: name.to_owned(),
            showlegend: None,
            legendgroup: None,
            opacity: None,
            text: None,
            textposition: None,
            marker: Marker::new(),
            error_x: None,
            error_y: None,
        })
    }
}

impl<X, Y> TraceSerialize for Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
