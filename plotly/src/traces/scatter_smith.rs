//! Smith chart scatter trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        Dim, Fill, Font, HoverOn, Label, LegendGroupTitle, Line, Marker, Mode, PlotType, Position,
        Selection, Visible,
    },
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// Construct a scatter trace on a Smith chart subplot, typically used to plot
/// normalized impedances given by their `real` and `imag` parts.
///
/// # Examples
///
/// ```
/// use plotly::{common::Mode, ScatterSmith};
///
/// let trace = ScatterSmith::new(vec![0.5, 1.0, 2.0], vec![0.5, -1.0, 0.0])
///     .mode(Mode::LinesMarkers);
///
/// let expected = serde_json::json!({
///     "type": "scattersmith",
///     "real": [0.5, 1.0, 2.0],
///     "imag": [0.5, -1.0, 0.0],
///     "mode": "lines+markers",
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ScatterSmith<Re, Im>
where
    Re: Serialize + Clone,
    Im: Serialize + Clone,
{
    #[field_setter(default = "PlotType::ScatterSmith")]
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
    /// Sets the real component of the data, in units of normalized impedance
    /// such that real=1, imag=0 is the center of the chart.
    real: Option<Vec<Re>>,
    /// Sets the imaginary component of the data, in units of normalized
    /// impedance such that real=1, imag=0 is the center of the chart.
    imag: Option<Vec<Im>>,
    /// Sets a reference between this trace's data coordinates and a Smith
    /// subplot, e.g. `"smith"` (the default) or `"smith2"`.
    subplot: Option<String>,
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
    /// Flaglist of `real`, `imag`, `text`, `name` joined with `+`, or one of
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
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
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

impl<Re, Im> ScatterSmith<Re, Im>
where
    Re: Serialize + Clone,
    Im: Serialize + Clone,
{
    pub fn new(real: Vec<Re>, imag: Vec<Im>) -> Box<Self> {
        Box::new(Self {
            real: Some(real),
            imag: Some(imag),
            ..Default::default()
        })
    }
}

impl<Re, Im> Trace for ScatterSmith<Re, Im>
where
    Re: Serialize + Clone,
    Im: Serialize + Clone,
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
    fn serialize_scatter_smith() {
        let trace = ScatterSmith::new(vec![1], vec![2])
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
            .subplot("smith2")
            .text("text")
            .text_position(Position::TopCenter)
            .text_template("%{a}")
            .text_template_fallback("-")
            .text_font(Font::new())
            .hover_text("h")
            .hover_info("real+imag")
            .hover_template("%{b}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .hover_on(HoverOn::Points)
            .marker(Marker::new())
            .line(Line::new())
            .connect_gaps(true)
            .clip_on_axis(false)
            .fill(Fill::ToSelf)
            .fill_color("#123456")
            .selected(Selection::new())
            .unselected(Selection::new())
            .selected_points(vec![0])
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "scattersmith",
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
            "real": [1], "imag": [2],
            "subplot": "smith2",
            "text": "text",
            "textposition": "top center",
            "texttemplate": "%{a}",
            "texttemplatefallback": "-",
            "textfont": {},
            "hovertext": "h",
            "hoverinfo": "real+imag",
            "hovertemplate": "%{b}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "hoveron": "points",
            "marker": {},
            "line": {},
            "connectgaps": true,
            "cliponaxis": false,
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
