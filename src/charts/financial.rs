use crate::charts::{Color, LineSegment, PlotType};
use crate::TraceSerialize;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Direction {
    Increasing { line: LineSegment },
    Decreasing { line: LineSegment },
}

#[derive(Serialize, Debug)]
pub struct Candlestick<X, O>
    where
        X: Serialize,
        O: num::Num + Serialize,
{
    pub x: Vec<X>,
    pub open: Vec<O>,
    pub high: Vec<O>,
    pub low: Vec<O>,
    pub close: Vec<O>,
    pub r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineSegment>,
    pub increasing: Direction,
    pub decreasing: Direction,
}

impl<X, O> Candlestick<X, O>
    where
        X: Serialize,
        O: num::Num + Serialize,
{
    pub fn new(
        x: Vec<X>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Candlestick<X, O>> {
        let mut iline = LineSegment::new();
        iline.width = Some(1);
        iline.color = Some(Color::Green);
        let mut dline = LineSegment::new();
        dline.width = Some(1);
        dline.color = Some(Color::Red);
        Box::new(Candlestick {
            x,
            open,
            high,
            low,
            close,
            r#type: PlotType::Candlestick,
            line: None,
            increasing: Direction::Increasing { line: iline },
            decreasing: Direction::Decreasing { line: dline },
        })
    }
}

impl<X, Y> TraceSerialize for Candlestick<X, Y>
    where
        X: Serialize,
        Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Ohlc<X, O>
    where
        X: Serialize,
        O: num::Num + Serialize,
{
    pub x: Vec<X>,
    pub open: Vec<O>,
    pub high: Vec<O>,
    pub low: Vec<O>,
    pub close: Vec<O>,
    pub r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineSegment>,
    pub increasing: Direction,
    pub decreasing: Direction,
}

impl<X, O> Ohlc<X, O>
    where
        X: Serialize,
        O: num::Num + Serialize,
{
    pub fn new(
        x: Vec<X>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Ohlc<X, O>> {
        let mut iline = LineSegment::new();
        iline.width = Some(2);
        iline.color = Some(Color::Green);
        let mut dline = LineSegment::new();
        dline.width = Some(2);
        dline.color = Some(Color::Red);
        Box::new(Ohlc {
            x,
            open,
            high,
            low,
            close,
            r#type: PlotType::Ohlc,
            line: None,
            increasing: Direction::Increasing { line: iline },
            decreasing: Direction::Decreasing { line: dline },
        })
    }
}

impl<X, Y> TraceSerialize for Ohlc<X, Y>
    where
        X: Serialize,
        Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
