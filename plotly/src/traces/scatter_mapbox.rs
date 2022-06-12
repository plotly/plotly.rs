//! Mapbox scatter plot

use serde::Serialize;

use crate::common::{
    color::Color,
    Dim, Font, HoverInfo, Label, LegendGroupTitle, Line, Marker, Mode,
    PlotType, Position, Visible,
};
use crate::private;
use crate::Trace;
use crate::private::{
    copy_iterable_to_vec, NumOrString, NumOrStringCollection
};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Fill {
    None,
    ToSelf
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SelectionMarker {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Box<dyn Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        assert!(0.0 <= opacity && opacity <= 1.0);
        self.marker.opacity = Some(opacity);
        self
    }

    /// Sets the marker size of un/selected points.
    pub fn size(mut self, size: usize) -> Self {
        self.marker.size = Some(Dim::Scalar(size));
        self
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone + 'static,
    Lon: Serialize + Clone + 'static,
{
    // Transcribed from https://plotly.com/python/reference/scattermapbox/.
    
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<Visible>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lat: Option<Vec<Lat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lon: Option<Vec<Lon>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textposition")]
    text_position: Option<Dim<Position>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<NumOrString>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subplot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "textfont")]
    text_font: Option<Font>,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectedpoints")]
    selected_points: Option<Vec<usize>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected: Option<Selection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unselected: Option<Selection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    below: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<Fill>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

// TODO insert _ in setter names

impl<Lat, Lon> ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone + 'static + std::default::Default,  // TODO why is "+ Default" necessary?
    Lon: Serialize + Clone + 'static + std::default::Default,
{
    pub fn new<Lat1, Lon1>(lat: Lat1, lon: Lon1) -> Box<Self>
    where
        Lat1: IntoIterator<Item = Lat>,
        Lon1: IntoIterator<Item = Lon>,
    {
        let lat = copy_iterable_to_vec(lat);
        let lon = copy_iterable_to_vec(lon);
        
        Box::new(Self {
            r#type: PlotType::ScatterMapbox,
            lat: Some(lat),
            lon: Some(lon),
            ..Default::default()
        })
    }

    /// Sets the trace name. The trace name appear as the legend item and on hover.
    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }
    
    /// Determines whether or not this trace is visible. If `Visible::LegendOnly`, the trace is not
    /// drawn, but can appear as a legend item (provided that the legend itself is visible).
    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    /// Determines whether or not an item corresponding to this trace is shown in the legend.
    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    /// Sets the legend rank for this trace. Items and groups with smaller ranks are presented on top/left
    /// side while with `"reversed" `legend.trace_order` they are on bottom/right side. The default legendrank
    /// is 1000, so that you can use ranks less than 1000 to place certain items before all unranked items,
    /// and ranks greater than 1000 to go after all unranked items.
    pub fn legendrank(mut self, legend_rank: usize) -> Box<Self> {
        self.legend_rank = Some(legend_rank);
        Box::new(self)
    }
    
    /// Sets the legend group for this trace. Traces part of the same legend group show/hide at the
    /// same time when toggling legend items.
    pub fn legendgroup(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    /// Set and style the title to appear for the legend group
    pub fn legendgrouptitle(mut self, legend_group_title: LegendGroupTitle) -> Box<Self> {
        self.legend_group_title = Some(legend_group_title);
        Box::new(self)
    }
    
    /// Sets the opacity of the trace.
    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }
    
    /// Determines the drawing mode for this scatter trace. If the provided `Mode` includes
    /// "Text" then the `text` elements appear at the coordinates. Otherwise, the `text` elements
    /// appear on hover. If there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    pub fn mode(mut self, mode: Mode) -> Box<Self> {
        self.mode = Some(mode);
        Box::new(self)
    }
    
    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    /// Sets text elements associated with each (x,y) pair. If a single string, the same string
    /// appears over all the data points. If an array of string, the items are mapped in order to
    /// the this trace's (x,y) coordinates. If the trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    /// Sets text elements associated with each (x, y, z) triplet. The items are mapped sequentially to
    /// this trace's (x, y, z) coordinates. If trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }
    
    /// Sets the positions of the `text` elements with respects to the (x,y) coordinates.
    pub fn text_position(mut self, text_position: Position) -> Box<Self> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    /// Sets the positions of the `text` elements with respects to the (x,y) coordinates.
    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Box<Self> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }
    
    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template(mut self, text_template: &str) -> Box<Self> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        Box::new(self)
    }

    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Self> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }
    
    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }
    
    /// Determines which trace information appear on hover. If `HoverInfo::None` or `HoverInfo::Skip`
    /// are set, no information is displayed upon hovering. But, if `HoverInfo::None` is set, click
    /// and hover events are still fired.
    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    /// Assigns extra meta information associated with this trace that can be used in various text
    /// attributes. Attributes such as trace `name`, graph, axis and colorbar `title.text`,
    /// annotation `text` `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same trace, simply use
    /// `%{meta[i]}` where `i` is the index or key of the `meta` item in question. To access trace
    /// `meta` in layout attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of the
    /// `meta` and `n` is the trace index.
    pub fn meta<V: Into<NumOrString>>(mut self, meta: V) -> Box<Self> {
        self.meta = Some(meta.into());
        Box::new(self)
    }
    
    /// Assigns extra data each datum. This may be useful when listening to hover, click and
    /// selection events. Note that, "scatter" traces also appends customdata items in the markers
    /// DOM elements
    pub fn custom_data<V: Into<NumOrString> + Clone>(mut self, custom_data: Vec<V>) -> Box<Self> {
        self.custom_data = Some(custom_data.into());
        Box::new(self)
    }

    /// Sets a reference between this trace's data coordinates and a mapbox subplot. If "mapbox" (the default value), the data refer to `layout.mapbox`. If "mapbox2", the data refer to `layout.mapbox2`, and so on.
    pub fn subplot(mut self, subplot: &str) -> Box<Self> {
        self.subplot = Some(subplot.to_owned());
        Box::new(self)
    }
    
    /// Determines how points are displayed and joined.
    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
        Box::new(self)
    }

    /// Line display properties.
    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }
    
    /// Sets the text font.
    pub fn text_font(mut self, text_font: Font) -> Box<Self> {
        self.text_font = Some(text_font);
        Box::new(self)
    }
    
    /// Vector containing integer indices of selected points. Has an effect only for traces that support
    /// selections. Note that an empty vector means an empty selection where the `unselected` are turned
    /// on for all points.
    pub fn selected_points(mut self, selected_points: Vec<usize>) -> Box<Self> {
        self.selected_points = Some(selected_points);
        Box::new(self)
    }

    /// Sets the style of selected points.
    pub fn selected(mut self, selected: Selection) -> Box<Self> {
        self.selected = Some(selected);
        Box::new(self)
    }

    /// Sets the style of unselected points.
    pub fn unselected(mut self, unselected: Selection) -> Box<Self> {
        self.unselected = Some(unselected);
        Box::new(self)
    }

    /// Determines if this scattermapbox trace's layers are to be inserted before the layer with the specified ID. By default, scattermapbox layers are inserted above all the base layers. To place the scattermapbox layers above every other layer, set `below` to "''".
    pub fn below(mut self, below: &str) -> Box<Self> {
        self.below = Some(below.to_owned());
        Box::new(self)
    }

    /// Determines whether or not gaps (i.e. {nan} or missing values) in the provided data arrays are connected.
    pub fn connectgaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }
    
    /// Sets the area to fill with a solid color. Defaults to "none" unless this trace is stacked,
    /// then it gets "tonexty" ("tonextx") if `orientation` is "v" ("h") Use with `fillcolor` if not
    /// "none". "tozerox" and "tozeroy" fill to x=0 and y=0 respectively. "tonextx" and "tonexty"
    /// fill between the endpoints of this trace and the endpoints of the trace before it,
    /// connecting those endpoints with straight lines (to make a stacked area graph); if there is
    /// no trace before it, they behave like "tozerox" and "tozeroy". "toself" connects the
    /// endpoints of the trace (or each segment of the trace if it has gaps) into a closed shape.
    /// "tonext" fills the space between two traces if one completely encloses the other
    /// (eg consecutive contour lines), and behaves like "toself" if there is no trace before it.
    /// "tonext" should not be used if one trace does not enclose the other. Traces in a
    /// `stackgroup` will only fill to (or be filled to) other traces in the same group. With
    /// multiple `stackgroup`s or some traces stacked and some not, if fill-linked traces are not
    /// already consecutive, the later ones will be pushed down in the drawing order.
    pub fn fill(mut self, fill: Fill) -> Box<Self> {
        self.fill = Some(fill);
        Box::new(self)
    }
    
    /// Sets the fill color. Defaults to a half-transparent variant of the line color, marker color,
    /// or marker line color, whichever is available.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Box<Self> {
        self.fill_color = Some(Box::new(fill_color));
        Box::new(self)
    }

    /// Properties of label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }
    
    /// Controls persistence of some user-driven changes to the trace: `constraintrange` in `parcoords` traces, as well as some `editable: True` modifications such as `name` and `colorbar.title`. Defaults to `layout.uirevision`. Note that other user-driven trace attribute changes are controlled by `layout` attributes: `trace.visible` is controlled by `layout.legend.uirevision`, `selectedpoints` is controlled by `layout.selectionrevision`, and `colorbar.(x|y)` (accessible with `config: {editable: True}`) is controlled by `layout.editrevision`. Trace changes are tracked by `uid`, which only falls back on trace index if no `uid` is provided. So if your app can add/remove traces before the end of the `data` array, such that the same trace has a different index, you can still preserve user-driven changes if you give each trace a `uid` that stays with it as it moves.
    pub fn uirevision<V: Into<NumOrString>>(mut self, ui_revision: V) -> Box<Self> {
        self.ui_revision = Some(ui_revision.into());
        Box::new(self)
    }
}

impl<Lat, Lon> Trace for ScatterMapbox<Lat, Lon>
where
    Lat: Serialize + Clone + 'static,
    Lon: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};
    use assert_json_diff::assert_json_eq;

    use super::*;

    #[test]
    fn test_serialize_fill() {
        assert_eq!(to_value(Fill::None).unwrap(), json!("none"));
        assert_eq!(to_value(Fill::ToSelf).unwrap(), json!("toself"));
    }
    
    #[test]
    fn test_serialize_selection() {
        let selection = Selection::new()
            .color("#123456")
            .opacity(0.5)
            .size(6);
        let expected = json!({"marker": {"color": "#123456", "opacity": 0.5, "size": 6}});

        assert_eq!(to_value(selection).unwrap(), expected);
    }

    #[test]
    fn test_serialize_scatter_mapbox() {
        let scatter_mapbox = ScatterMapbox::new(vec![45.5017], vec![-73.5673])
            .name("name")
            .visible(Visible::True)
            .show_legend(true)
            .legendrank(1000)
            .legendgroup("legend group")
            .legendgrouptitle(LegendGroupTitle::new("Legend Group Title"))
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
            .connectgaps(false)
            .fill(Fill::None)
            .fill_color("#ff0000aa")
            .hover_label(Label::new())
            .uirevision(6);
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

        assert_json_eq!(to_value(scatter_mapbox).unwrap(), expected);
    }
}
