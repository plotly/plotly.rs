use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Scatter<X, Y> where X: num::Num + serde::Serialize, Y: num::Num + serde::Serialize{
    pub x: Vec<X>,
    pub y: Vec<Y>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<Vec<f64>>,
    r#type: String,
    pub visible: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showlegend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendgroup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    pub mode: String,
}

impl<X, Y> Scatter<X, Y>  where X: num::Num + serde::Serialize, Y: num::Num + serde::Serialize{
    pub fn new(name: &str, x: Vec<X>, y: Vec<Y>) -> Scatter<X, Y> {
        Scatter {
            x,
            y,
            z: None,
            r#type: String::from("scatter"),
            visible: true,
            name: name.to_owned(),
            showlegend: None,
            legendgroup: None,
            opacity: None,
            mode: String::from("lines"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Line {

}

#[derive(Serialize, Debug)]
pub struct Bar {

}

#[derive(Serialize, Debug)]
pub struct Pie {

}
