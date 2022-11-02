//! Mapbox scatter plot

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::common::{
    color::Color, Dim, Font, HoverInfo, Label, LegendGroupTitle, Line, Marker, Mode, PlotType,
    Position, Visible,
};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::Trace;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Fill {
    None,
    ToSelf,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct SelectionMarker {
    color: Option<Box<dyn Color>>,
    opacity: Option<f64>,
    size: Option<Dim<usize>>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Selection {
    marker: SelectionMarker,
}

impl Selection {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the marker color of un/selected points.
    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.marker.color = Some(Box::new(color));
        self
    }

    /// Sets the marker opacity of un/selected points.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.marker.opacity = Some(opacity);
        self
    }

    /// Sets the marker size of un/selected points.
    pub fn size(mut self, size: usize) -> Self {
        self.marker.size = Some(Dim::Scalar(size));
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone,
    Lon: Serialize + Clone,
{
    #[field_setter(default = "PlotType::ScatterMapbox")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appear as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,

    /// Determines whether or not an item corresponding to this trace is shown
    /// in the legend.
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater
    /// than 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the legend group for this trace. Traces part of the same legend
    /// group show/hide at the same time when toggling legend items.
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    /// Set and style the title to appear for the legend group.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,

    /// Sets the opacity of the trace.
    opacity: Option<f64>,
    /// Determines the drawing mode for this scatter trace. If the provided
    /// `Mode` includes "Text" then the `text` elements appear at the
    /// coordinates. Otherwise, the `text` elements appear on hover. If
    /// there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    mode: Option<Mode>,
    /// Assigns id labels to each datum. These ids for object constancy of data
    /// points during animation. Should be an array of strings, not numbers
    /// or any other type.
    ids: Option<Vec<String>>,

    lat: Option<Vec<Lat>>,
    lon: Option<Vec<Lon>>,

    /// Sets text elements associated with each (x,y) pair. If a single string,
    /// the same string appears over all the data points. If an array of
    /// strings, the items are mapped in order to the this trace's (x,y)
    /// coordinates. If the trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover
    /// labels.
    text: Option<Dim<String>>,
    /// Sets the positions of the `text` elements with respects to the (x,y)
    /// coordinates.
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    /// Template string used for rendering the information text that appear on
    /// points. Note that this will override `textinfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using
    /// d3-time-format's syntax %{variable|d3-time-format}, for example
    /// "Day: %{2019-01-01|%A}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified
    /// per-point (the ones that are `arrayOk: true`) are available.
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    /// Sets hover text elements associated with each (x,y) pair. If a single
    /// string, the same string appears over all the data points. If an
    /// array of string, the items are mapped in order to the this trace's
    /// (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appear on hover. If `HoverInfo::None`
    /// or `HoverInfo::Skip` are set, no information is displayed upon
    /// hovering. But, if `HoverInfo::None` is set, click and hover events
    /// are still fired.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Template string used for rendering the information that appear on hover
    /// box. Note that this will override `HoverInfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's
    /// syntax %{variable|d3-time-format}, for example "Day:
    /// %{2019-01-01|%A}". https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in
    /// `hovertemplate` are the ones emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available. Anything contained in tag
    /// `<extra>` is displayed in the secondary box, for example
    /// "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes. Attributes such as trace `name`,
    /// graph, axis and colorbar `title.text`, annotation `text`
    /// `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same
    /// trace, simply use `%{meta[i]}` where `i` is the index or key of the
    /// `meta` item in question. To access trace `meta` in layout
    /// attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of
    /// the `meta` and `n` is the trace index.
    meta: Option<NumOrString>,
    /// Assigns extra data each datum. This may be useful when listening to
    /// hover, click and selection events. Note that, "scatter" traces also
    /// appends customdata items in the markers DOM elements.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,

    /// Sets a reference between this trace's data coordinates and a mapbox
    /// subplot. If "mapbox" (the default value), the data refer to
    /// `layout.mapbox`. If "mapbox2", the data refer to `layout.mapbox2`, and
    /// so on.
    subplot: Option<String>,
    /// Determines how points are displayed and joined.
    marker: Option<Marker>,

    /// Line display properties.
    line: Option<Line>,

    /// Sets the text font.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,

    /// Vector containing integer indices of selected points. Has an effect only
    /// for traces that support selections. Note that an empty vector means
    /// an empty selection where the `unselected` are turned on for all
    /// points.
    #[serde(rename = "selectedpoints")]
    selected_points: Option<Vec<usize>>,

    /// Sets the style of selected points.
    selected: Option<Selection>,
    /// Sets the style of unselected points.
    unselected: Option<Selection>,

    /// Determines if this scattermapbox trace's layers are to be inserted
    /// before the layer with the specified ID. By default, scattermapbox
    /// layers are inserted above all the base layers. To place the
    /// scattermapbox layers above every other layer, set `below` to "''".
    below: Option<String>,
    /// Determines whether or not gaps (i.e. {nan} or missing values) in the
    /// provided data arrays are connected.
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,

    /// Sets the area to fill with a solid color. Defaults to "none" unless this
    /// trace is stacked, then it gets "tonexty" ("tonextx") if
    /// `orientation` is "v" ("h") Use with `fillcolor` if not
    /// "none". "tozerox" and "tozeroy" fill to x=0 and y=0 respectively.
    /// "tonextx" and "tonexty" fill between the endpoints of this trace and
    /// the endpoints of the trace before it, connecting those endpoints
    /// with straight lines (to make a stacked area graph); if there is
    /// no trace before it, they behave like "tozerox" and "tozeroy". "toself"
    /// connects the endpoints of the trace (or each segment of the trace if
    /// it has gaps) into a closed shape. "tonext" fills the space between
    /// two traces if one completely encloses the other (eg consecutive
    /// contour lines), and behaves like "toself" if there is no trace before
    /// it. "tonext" should not be used if one trace does not enclose the
    /// other. Traces in a `stackgroup` will only fill to (or be filled to)
    /// other traces in the same group. With multiple `stackgroup`s or some
    /// traces stacked and some not, if fill-linked traces are not
    /// already consecutive, the later ones will be pushed down in the drawing
    /// order.
    fill: Option<Fill>,
    /// Sets the fill color. Defaults to a half-transparent variant of the line
    /// color, marker color, or marker line color, whichever is available.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Properties of label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,

    /// Controls persistence of some user-driven changes to the trace:
    /// `constraintrange` in `parcoords` traces, as well as some `editable:
    /// True` modifications such as `name` and `colorbar.title`. Defaults to
    /// `layout.uirevision`. Note that other user-driven trace attribute changes
    /// are controlled by `layout` attributes: `trace.visible` is controlled
    /// by `layout.legend.uirevision`, `selectedpoints` is controlled
    /// by `layout.selectionrevision`, and `colorbar.(x|y)` (accessible with
    /// `config: {editable: True}`) is controlled by `layout.editrevision`.
    /// Trace changes are tracked by `uid`, which only falls back on trace
    /// index if no `uid` is provided. So if your app can add/remove traces
    /// before the end of the `data` array, such that the same trace has a
    /// different index, you can still preserve user-driven changes if you give
    /// each trace a `uid` that stays with it as it moves.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Lat, Lon> ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone + std::default::Default, // TODO why is "+ Default" necessary?
    Lon: Serialize + Clone + std::default::Default,
{
    pub fn new<Lat1, Lon1>(lat: Vec<Lat>, lon: Vec<Lon>) -> Box<Self> {
        Box::new(Self {
            lat: Some(lat),
            lon: Some(lon),
            ..Default::default()
        })
    }
}

impl<Lat, Lon> Trace for ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone,
    Lon: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_fill() {
        assert_eq!(to_value(Fill::None).unwrap(), json!("none"));
        assert_eq!(to_value(Fill::ToSelf).unwrap(), json!("toself"));
    }

    #[test]
    fn test_serialize_selection() {
        let selection = Selection::new().color("#123456").opacity(0.5).size(6);
        let expected = json!({"marker": {"color": "#123456", "opacity": 0.5, "size": 6}});

        assert_eq!(to_value(selection).unwrap(), expected);
    }

    #[test]
    fn test_serialize_scatter_mapbox() {
        let scatter_mapbox = ScatterMapbox::new(vec![45.5017], vec![-73.5673])
            .name("name")
            .visible(Visible::True)
            .show_legend(true)
            .legend_rank(1000)
            .legend_group("legend group")
            .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
            .opacity(0.5)
            .mode(Mode::LinesText)
            .ids(vec!["one"])
            .text("text")
            .text_array(vec!["text"])
            .text_position(Position::BottomLeft)
            .text_position_array(vec![Position::TopCenter])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .hover_info(HoverInfo::XAndYAndZ)
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template"])
            .meta("meta")
            .custom_data(vec!["custom_data"])
            .subplot("mapbox2")
            .marker(Marker::new())
            .line(Line::new())
            .text_font(Font::new())
            .selected_points(vec![0])
            .selected(Selection::new().color("#111111"))
            .unselected(Selection::new().color("#777777"))
            .below("")
            .connect_gaps(false)
            .fill(Fill::None)
            .fill_color("#ff0000aa")
            .hover_label(Label::new())
            .ui_revision(6);
        let expected = json!({
            "type": "scattermapbox",
            "lat": [45.5017],
            "lon": [-73.5673],
            "name": "name",
            "visible": true,
            "showlegend": true,
            "legendrank": 1000,
            "legendgroup": "legend group",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "opacity": 0.5,
            "mode": "lines+text",
            "ids": ["one"],
            "text": ["text"],
            "textposition": ["top center"],
            "texttemplate": ["text_template"],
            "hovertext": ["hover_text"],
            "hoverinfo": "x+y+z",
            "hovertemplate": ["hover_template"],
            "meta": "meta",
            "customdata": ["custom_data"],
            "subplot": "mapbox2",
            "marker": {},
            "line": {},
            "textfont": {},
            "selectedpoints": [0],
            "selected": {"marker": {"color": "#111111"}},
            "unselected": {"marker": {"color": "#777777"}},
            "below": "",
            "connectgaps": false,
            "fill": "none",
            "fillcolor": "#ff0000aa",
            "hoverlabel": {},
            "uirevision": 6,
        });

        assert_eq!(to_value(scatter_mapbox).unwrap(), expected);
    }
}
