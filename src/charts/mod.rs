mod basic;
mod financial;
mod scientific;
mod statistical;

pub use basic::Scatter;
pub use basic::Line;
pub use basic::Bar;
pub use basic::Pie;

pub use statistical::ErrorBars;
pub use statistical::BoxPlot;
pub use statistical::Histogram;
pub use statistical::Density;

pub use scientific::Contour;
pub use scientific::HeatMap;
pub use scientific::ParallelCoordinates;

pub use financial::Candlestick;
pub use financial::Ohlc;

pub enum Trace<X, Y> where X: num::Num + serde::Serialize, Y: num::Num + serde::Serialize {
    Scatter(Scatter<X, Y>),
    Line(Line),
    Bar(Bar),
    Pie(Pie),
    ErrorBars(ErrorBars),
    BoxPlot(BoxPlot),
    Histogram(Histogram),
    Density(Density),
    Contour(Contour),
    HeatMap(HeatMap),
    ParallelCoordinates(ParallelCoordinates),
    Candlestick(Candlestick),
    Ohlc(Ohlc),
}
