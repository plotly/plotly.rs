//! Carpet scatter trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        Dim, Fill, Font, HoverOn, Label, LegendGroupTitle, Line, Marker, Mode, PlotType, Position,
        Selection, Visible, XAxisId, YAxisId,
    },
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// Construct a scatter trace on a [`Carpet`](crate::Carpet): each point is
/// placed by its `a` and `b` carpet coordinates. The carpet is selected by its
/// [`carpet`](ScatterCarpet::carpet) id.
///
/// # Examples
///
/// ```
/// use plotly::{common::Mode, ScatterCarpet};
///
/// let trace = ScatterCarpet::new(vec![4.0, 4.5], vec![1.5, 2.5])
///     .carpet("c1")
///     .mode(Mode::Markers);
///
/// let expected = serde_json::json!({
///     "type": "scattercarpet",
///     "a": [4.0, 4.5],
///     "b": [1.5, 2.5],
///     "carpet": "c1",
///     "mode": "markers",
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ScatterCarpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    #[field_setter(default = "PlotType::ScatterCarpet")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    opacity: Option<f64>,
    mode: Option<Mode>,
    ids: Option<Vec<String>>,
    /// Sets the `a`-axis coordinates.
    a: Option<Vec<A>>,
    /// Sets the `b`-axis coordinates.
    b: Option<Vec<B>>,
    /// The [`carpet`](crate::Carpet::carpet) id of the carpet this trace lies
    /// on. If omitted, the first carpet is used.
    carpet: Option<String>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    /// Sets the layer on which this trace is displayed, relative to other
    /// SVG traces on the same subplot.
    #[serde(rename = "zorder")]
    z_order: Option<i32>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "texttemplatefallback")]
    text_template_fallback: Option<Dim<String>>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Flaglist of `a`, `b`, `text`, `name` joined with `+`, or one of
    /// `all`, `none`, `skip`.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<String>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoveron")]
    hover_on: Option<HoverOn>,
    marker: Option<Marker>,
    line: Option<Line>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    fill: Option<Fill>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    selected: Option<Selection>,
    unselected: Option<Selection>,
    #[serde(rename = "selectedpoints")]
    selected_points: Option<Vec<u32>>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<A, B> ScatterCarpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    pub fn new(a: Vec<A>, b: Vec<B>) -> Box<Self> {
        Box::new(Self {
            a: Some(a),
            b: Some(b),
            ..Default::default()
        })
    }
}

impl<A, B> Trace for ScatterCarpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_scatter_carpet() {
        let trace = ScatterCarpet::new(vec![1], vec![2])
            .name("t")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.5)
            .mode(Mode::LinesMarkersText)
            .ids(vec!["i"])
            .carpet("c1")
            .x_axis("x2")
            .y_axis("y2")
            .z_order(1)
            .text("text")
            .text_position(Position::TopCenter)
            .text_template("%{a}")
            .text_template_fallback("-")
            .text_font(Font::new())
            .hover_text("h")
            .hover_info("a+b")
            .hover_template("%{b}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .hover_on(HoverOn::Points)
            .marker(Marker::new())
            .line(Line::new())
            .connect_gaps(true)
            .fill(Fill::ToSelf)
            .fill_color("#123456")
            .selected(Selection::new())
            .unselected(Selection::new())
            .selected_points(vec![0])
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "scattercarpet",
            "name": "t",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.5,
            "mode": "lines+markers+text",
            "ids": ["i"],
            "a": [1], "b": [2],
            "carpet": "c1",
            "xaxis": "x2",
            "yaxis": "y2",
            "zorder": 1,
            "text": "text",
            "textposition": "top center",
            "texttemplate": "%{a}",
            "texttemplatefallback": "-",
            "textfont": {},
            "hovertext": "h",
            "hoverinfo": "a+b",
            "hovertemplate": "%{b}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "hoveron": "points",
            "marker": {},
            "line": {},
            "connectgaps": true,
            "fill": "toself",
            "fillcolor": "#123456",
            "selected": {"marker": {}},
            "unselected": {"marker": {}},
            "selectedpoints": [0],
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
