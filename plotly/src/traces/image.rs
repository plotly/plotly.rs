//! Image plot

use serde::Serialize;

use crate::common::{
    Dim, HoverInfo, Label, LegendGroupTitle,
    PlotType, Visible,
};
use crate::private;
use crate::Trace;
use crate::private::{
    NumOrString, NumOrStringCollection
};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PixelColor<U> {
    Color3([U; 3]),
    Color4([U; 4]),
}

/// A (marker?) trait allowing several ways to describe an image.
pub trait ImageData {}
// DynClone + ErasedSerialize + Send + Sync + std::fmt::Debug + 'static ?

//dyn_clone::clone_trait_object!(ImageData);
//erased_serde::serialize_trait_object!(ImageData);

impl<U> ImageData for Vec<Vec<[U; 3]>> {}
impl<U> ImageData for Vec<Vec<[U; 4]>> {}
#[cfg(feature = "plotly_imageio")]
impl ImageData for RgbImage {}
#[cfg(feature = "plotly_imageio")]
impl ImageData for RgbaImage {}
#[cfg(feature = "plotly_ndarray")]
impl<U> ImageData for Array<[U; 3], Ix2> {}
#[cfg(feature = "plotly_ndarray")]
impl<U> ImageData for Array<[U; 4], Ix2> {}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ColorModel {
    RGB,
    RGBA,
    RGBA256,
    HSL,
    HSLA,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ZSmooth {
    Fast,
    False
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Image<U>
where
    U: Serialize + Clone + 'static,
{
    // Transcribed from https://plotly.com/python/reference/image/.
    
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<Visible>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x0: Option<NumOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dx: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y0: Option<NumOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dy: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<Vec<Vec<PixelColor<U>>>>, // Option<Box<dyn ImageData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<NumOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data: Option<NumOrStringCollection>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "colormodel")]
    color_model: Option<ColorModel>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "zmax")]
    z_max: Option<Vec<Vec<PixelColor<U>>>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zmin")]
    z_min: Option<Vec<Vec<PixelColor<U>>>>,
    
    #[serde(skip_serializing_if = "Option::is_none", rename = "zsmooth")]
    z_smooth: Option<ZSmooth>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<U> Image<U>
where
    U: Serialize + Default + Clone + 'static,
{
    pub fn new(z: Vec<Vec<PixelColor<U>>>) -> Box<Self>
    {
        Box::new(Self {
            r#type: PlotType::Image,
            z: Some(z),
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

    /// Sets the legend rank for this trace. Items and groups with smaller ranks are presented on top/left
    /// side while with `"reversed" `legend.trace_order` they are on bottom/right side. The default legendrank
    /// is 1000, so that you can use ranks less than 1000 to place certain items before all unranked items,
    /// and ranks greater than 1000 to go after all unranked items.
    pub fn legendrank(mut self, legend_rank: usize) -> Box<Self> {
        self.legend_rank = Some(legend_rank);
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
    
    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    /// Set the image's x position.
    pub fn x0<V: Into<NumOrString>>(mut self, x0: V) -> Box<Self> {
        self.x0 = Some(x0.into());
        Box::new(self)
    }

    /// Set the pixel's horizontal size.
    pub fn dx(mut self, dx: f64) -> Box<Self> {
        self.dx = Some(dx);
        Box::new(self)
    }

    /// Set the image's y position.
    pub fn y0<V: Into<NumOrString>>(mut self, y0: V) -> Box<Self> {
        self.y0 = Some(y0.into());
        Box::new(self)
    }

    /// Set the pixel's vertical size.
    pub fn dy(mut self, dy: f64) -> Box<Self> {
        self.dy = Some(dy);
        Box::new(self)
    }

    /// A 2-dimensional array in which each element is an array of 3 or 4 numbers representing a
    /// color.
    pub fn z(mut self, z: Vec<Vec<PixelColor<U>>>) -> Box<Self> {
        self.z = Some(z);
        Box::new(self)
    }

    /// Specifies the data URI of the image to be visualized. The URI consists of "data:image/[<media subtype>][;base64],<data>"
    pub fn source(mut self, source: &str) -> Box<Self> {
        self.source = Some(source.to_owned());
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

    /// Sets a reference between this trace's x coordinates and a 2D cartesian x axis. If "x" (
    /// the default value), the x coordinates refer to `Layout::x_axis`. If "x2", the x coordinates
    /// refer to `Layout::x_axis2`, and so on.
    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }
    
    /// Sets a reference between this trace's y coordinates and a 2D cartesian y axis. If "y"
    /// (the default value), the y coordinates refer to `Layout::y_axis`. If "y2", the y coordinates
    /// refer to `Layout::y_axis2`, and so on.
    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_owned());
        Box::new(self)
    }

    /// Color model used to map the numerical color components described in `z` into colors. If `source` is specified, this attribute will be set to `rgba256` otherwise it defaults to `rgb`.
    pub fn colormodel(mut self, color_model: ColorModel) -> Box<Self> {
        self.color_model = Some(color_model);
        Box::new(self)
    }

    /// Array defining the higher bound for each color component. Note that the default value will depend on the colormodel. For the `rgb` colormodel, it is [255, 255, 255]. For the `rgba` colormodel, it is [255, 255, 255, 1]. For the `rgba256` colormodel, it is [255, 255, 255, 255]. For the `hsl` colormodel, it is [360, 100, 100]. For the `hsla` colormodel, it is [360, 100, 100, 1].
    pub fn zmax(mut self, z_max: Vec<Vec<PixelColor<U>>>) -> Box<Self> {
        self.z_max = Some(z_max);
        Box::new(self)
    }

    /// Array defining the lower bound for each color component. Note that the default value will depend on the colormodel. For the `rgb` colormodel, it is [0, 0, 0]. For the `rgba` colormodel, it is [0, 0, 0, 0]. For the `rgba256` colormodel, it is [0, 0, 0, 0]. For the `hsl` colormodel, it is [0, 0, 0]. For the `hsla` colormodel, it is [0, 0, 0, 0].
    pub fn zmin(mut self, z_min: Vec<Vec<PixelColor<U>>>) -> Box<Self> {
        self.z_min = Some(z_min);
        Box::new(self)
    }

    /// Picks a smoothing algorithm used to smooth `z` data. This only applies for image traces that use the `source` attribute.
    pub fn zsmooth(mut self, z_smooth: ZSmooth) -> Box<Self> {
        self.z_smooth = Some(z_smooth);
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

impl<U> Trace for Image<U>
where
    U: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
