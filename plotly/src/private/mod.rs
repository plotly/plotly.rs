use crate::common::color::Color;
use serde::{Serialize, Serializer};

pub fn owned_string_vector<S: AsRef<str>>(s: Vec<S>) -> Vec<String> {
    s.iter()
        .map(|x| x.as_ref().to_string())
        .collect::<Vec<String>>()
}

pub fn to_color_array<C: Color>(v: Vec<C>) -> Vec<String> {
    let mut sv: Vec<String> = Vec::with_capacity(v.len());
    for e in v.iter() {
        sv.push(e.to_color_string());
    }
    sv
}

#[derive(Debug)]
pub struct TruthyEnum<E> {
    pub e: E,
}

impl<E> Serialize for TruthyEnum<E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = serde_json::to_string(&self.e)
            .unwrap()
            .chars()
            .filter(|c| *c != '"')
            .collect::<String>();
        if s == "true" {
            return serializer.serialize_bool(true);
        }
        if s == "false" {
            return serializer.serialize_bool(false);
        }
        serializer.serialize_str(&s)
    }
}
