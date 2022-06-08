//! Mesh plot

use serde::Serialize;

use crate::common::{
    color::Color,
    Calendar, ColorBar, Dim, ErrorData, Fill, Font, GroupNorm, HoverInfo, Label, LegendGroupTitle,
    Line, Marker, Mode, Orientation, PlotType, Position, Visible,
};
use crate::private;
use crate::Trace;
use crate::private::{
    copy_iterable_to_vec, NumOrString, NumOrStringCollection
};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IntensityMode {
    Vertex,
    Cell
}

// Defined for documentation purposes only.
impl Default for IntensityMode {
    fn default() -> Self { IntensityMode::Vertex }
}

// TODO line break documentation properly

#[derive(Serialize, Clone, Debug, Default)]
pub struct Mesh3D<X, Y, Z>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
    Z: Serialize + Clone + 'static,
{
    // Transcribed from https://plotly.com/python/reference/mesh3d/.
    
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
    ids: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<Vec<Z>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    i: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    j: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    k: Option<Vec<usize>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "facecolor")]
    face_color: Option<Vec<Box<dyn Color>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intensity: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "intensitymode")]
    intensity_mode: Option<IntensityMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "vertexcolor")]
    vertex_color: Option<Vec<Box<dyn Color>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xhoverformat")]
    x_hover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yhoverformat")]
    y_hover_format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<NumOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data: Option<NumOrStringCollection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "coloraxis")]
    color_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Box<dyn Color>>,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar_orientation")]
    orientation: Option<Orientation>,  // Move this into ColorBar?

    //<autocolorscale,colorscale,showscale,reversescale,zhoverformat>
    //<cauto,cmid,cmin,alphahull,delaunayaxis,contour>
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "flatshading")]
    flat_shading: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,

    //<lighting,lightposition>
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    z_calendar: Option<Calendar>,

    //<uirevision>
}

impl<X, Y, Z> Mesh3D<X, Y, Z>
where
    X: Serialize + Default + Clone + 'static,
    Y: Serialize + Default + Clone + 'static,
    Z: Serialize + Default + Clone + 'static,
{
    pub fn new<X1, Y1, Z1, I1, J1, K1>(x: X1, y: Y1, z: Z1, i: I1, j: J1, k: K1) -> Box<Self>
    where
        X1: IntoIterator<Item = X>,
        Y1: IntoIterator<Item = Y>,
        Z1: IntoIterator<Item = Z>,
        I1: IntoIterator<Item = usize>,
        J1: IntoIterator<Item = usize>,
        K1: IntoIterator<Item = usize>,
    {
        let x = copy_iterable_to_vec(x);
        let y = copy_iterable_to_vec(y);
        let z = copy_iterable_to_vec(z);
        
        let i = copy_iterable_to_vec(i);
        let j = copy_iterable_to_vec(j);
        let k = copy_iterable_to_vec(k);
        
        Box::new(Self {
            r#type: PlotType::Mesh3D,
            x: Some(x),
            y: Some(y),
            z: Some(z),
            i: Some(i),
            j: Some(j),
            k: Some(k),
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
    pub fn legend_rank(mut self, legend_rank: usize) -> Box<Self> {
        self.legend_rank = Some(legend_rank);
        Box::new(self)
    }
    
    /// Sets the legend group for this trace. Traces part of the same legend group hide/show at the
    /// same time when toggling legend items.
    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    /// Set and style the title to appear for the legend group
    pub fn legend_group_title(mut self, legend_group_title: LegendGroupTitle) -> Box<Self> {
        self.legend_group_title = Some(legend_group_title);
        Box::new(self)
    }
    
    /// Sets the opacity of the trace.
    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }
    
    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    /// Sets the color of each face. Overrides "color" and "vertexcolor".
    pub fn facecolor<C: Color + 'static>(mut self, face_color: Vec<C>) -> Box<Self> {
        let dyn_face_color: Vec::<Box::<dyn Color>> = face_color.into_iter().map(|c| Box::new(c) as _).collect();
        self.face_color = Some(dyn_face_color);
        Box::new(self)
    }
    
    /// Sets the intensity values for vertices or cells as defined by `intensitymode`.
    /// It can be used for plotting fields on meshes.
    pub fn intensity(mut self, intensity: Vec<f64>) -> Box<Self> {
        self.intensity = Some(intensity);
        Box::new(self)
    }

    /// Determines the source of `intensity` values.
    pub fn intensitymode(mut self, intensity_mode: IntensityMode) -> Box<Self> {
        self.intensity_mode = Some(intensity_mode);
        Box::new(self)
    }

    /// Sets the color of each vertex Overrides "color". While Red, green and blue colors are in the range of 0 and 255; in the case of having vertex color data in RGBA format, the alpha color should be normalized to be between 0 and 1.
    pub fn vertexcolor<C: Color + 'static>(mut self, vertex_color: Vec<C>) -> Box<Self> {
        let dyn_vertex_color: Vec::<Box::<dyn Color>> = vertex_color.into_iter().map(|c| Box::new(c) as _).collect();
        self.vertex_color = Some(dyn_vertex_color);
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
    
    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x, y, z) triplet. The items are mapped sequentially across
    /// this trace's (x,y) coordinates. To be seen, the trace `hover_info` must contain a "Text" flag.
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

    /// Sets the hover text formatting rulefor `x` using d3 formatting mini-languages which are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's date formatter: "%h" for half of the year as a decimal number as well as "%{n}f" for fractional seconds with n digits. For example, "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display "09~15~23.46"By default the values are formatted using `xaxis.hoverformat`.
    pub fn xhoverformat(mut self, x_hover_format: &str) -> Box<Self> {
        self.x_hover_format = Some(x_hover_format.to_owned());
        Box::new(self)
    }

    /// Sets the hover text formatting rulefor `y` using d3 formatting mini-languages which are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's date formatter: "%h" for half of the year as a decimal number as well as "%{n}f" for fractional seconds with n digits. For example, "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display "09~15~23.46"By default the values are formatted using `yaxis.hoverformat`.
    pub fn yhoverformat(mut self, y_hover_format: &str) -> Box<Self> {
        self.y_hover_format = Some(y_hover_format.to_owned());
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
    /// DOM elements.
    pub fn custom_data<V: Into<NumOrString> + Clone>(mut self, custom_data: Vec<V>) -> Box<Self> {
        self.custom_data = Some(custom_data.into());
        Box::new(self)
    }

    /// Sets a reference between this trace's 3D coordinate system and a 3D scene. If "scene" (the
    /// default value), the (x,y,z) coordinates refer to `layout.scene`. If "scene2", the (x, y, z)
    /// coordinates refer to `layout.scene2`, and so on.
    pub fn scene(mut self, scene: &str) -> Box<Self> {
        self.scene = Some(scene.to_string());
        Box::new(self)
    }

    /// Sets a reference between this trace's 3D coordinate system and a 3D scene. If "scene" (the default value), the (x,y,z) coordinates refer to `layout.scene`. If "scene2", the (x,y,z) coordinates refer to `layout.scene2`, and so on.
    pub fn coloraxis(mut self, color_axis: &str) -> Box<Self> {
        self.color_axis = Some(color_axis.to_string());
        Box::new(self)
    }
    
    /// Sets the color of the whole mesh.
    pub fn color<C: Color>(mut self, color: C) -> Box<Self> {
        self.color = Some(Box::new(color));
        Box::new(self)
    }
    
    pub fn colorbar(mut self, color_bar: ColorBar) -> Box<Self> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    /// Only relevant when `stackgroup` is used, and only the first `orientation` found in the
    /// `stackgroup` will be used - including if `visible` is "legendonly" but not if it is `false`.
    /// Sets the stacking direction. With "v" ("h"), the y (x) values of subsequent traces are
    /// added. Also affects the default value of `fill`.
    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    //<autocolorscale,colorscale,showscale,reversescale,zhoverformat>
    //<cauto,cmid,cmin,alphahull,delaunayaxis,contour>
    
    /// Determines whether or not normal smoothing is applied to the meshes, creating meshes with an angular, low-poly look via flat reflections.
    pub fn flatshading(mut self, flat_shading: bool) -> Box<Self> {
        self.flat_shading = Some(flat_shading);
        Box::new(self)
    }

    /// Properties of label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    //<lighting,lightposition>
    
    /// Sets the calendar system to use with `x` date data.
    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    /// Sets the calendar system to use with `y` date data.
    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    /// Sets the calendar system to use with `z` date data.
    pub fn z_calendar(mut self, z_calendar: Calendar) -> Box<Self> {
        self.z_calendar = Some(z_calendar);
        Box::new(self)
    }

    //<uirevision>
}

impl<X, Y, Z> Trace for Mesh3D<X, Y, Z>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
    Z: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
