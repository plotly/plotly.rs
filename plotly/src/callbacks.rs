use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Function, window, HtmlElement};

/// Provides utilities for binding Plotly.js click events to Rust closures
/// via `wasm-bindgen`.  
///  
/// This module defines a `PlotlyDiv` foreign type for the Plotly `<div>` element,  
/// a high-level `bind_click` function to wire up Rust callbacks, and  
/// the `ClickPoint`/`ClickEvent` data structures to deserialize event payloads.

#[wasm_bindgen]
extern "C" {

    /// A wrapper around the JavaScript `HTMLElement` representing a Plotly `<div>`.
    ///
    /// This type extends `web_sys::HtmlElement` and exposes Plotly’s  
    /// `.on(eventName, callback)` method for attaching event listeners.

    #[wasm_bindgen(extends= HtmlElement, js_name=HTMLElement)]
    type PlotlyDiv;

    /// Attach a JavaScript event listener to this Plotly `<div>`.
    ///
    /// # Parameters
    /// - `event`: The Plotly event name (e.g., `"plotly_click"`).
    /// - `cb`: A JS `Function` to invoke when the event fires.
    ///
    /// # Panics
    /// This method assumes the underlying element is indeed a Plotly div  
    /// and that the Plotly.js library has been loaded on the page.

    #[wasm_bindgen(method,structural,js_name=on)]
    fn on(this: &PlotlyDiv, event: &str, cb: &Function);
}

/// Bind a Rust callback to the Plotly `plotly_click` event on a given `<div>`.
///
/// # Type Parameters
/// - `F`: A `'static + FnMut(ClickEvent)` closure type to handle the click data.
///
/// # Parameters
/// - `div_id`: The DOM `id` attribute of the Plotly `<div>`.
/// - `cb`: A mutable Rust closure that will be called with a `ClickEvent`.
///
/// # Details
/// 1. Looks up the element by `div_id`, converts it to `PlotlyDiv`.  
/// 2. Wraps a `Closure<dyn FnMut(JsValue)>` that deserializes the JS event  
///    into our `ClickEvent` type via `serde_wasm_bindgen`.  
/// 3. Calls `plot_div.on("plotly_click", …)` to register the listener.  
/// 4. Forgets the closure so it lives for the lifetime of the page.
///
/// # Example
/// ```ignore
/// bind_click("my-plot", |evt| {
///     web_sys::console::log_1(&format!("{:?}", evt).into());
/// });
/// ```


pub fn bind_click<F>(div_id: &str, mut cb: F)
where 
    F: 'static + FnMut(ClickEvent)
{

    let plot_div: PlotlyDiv = window().unwrap()
        .document().unwrap()
        .get_element_by_id(div_id).unwrap()
        .unchecked_into();
    let closure = Closure::wrap(Box::new(move |event: JsValue| {
        let event: ClickEvent = serde_wasm_bindgen::from_value(event)
            .expect("\n Couldn't serialize the event \n");
        cb(event);
    }) as Box<dyn FnMut(JsValue)>);
    plot_div.on("plotly_click", &closure.as_ref().unchecked_ref());
    closure.forget();
}


/// Represents a single point from a Plotly click event.
///
/// Fields mirror Plotly’s `event.points[i]` properties, all optional
/// where appropriate:
///
/// - `curve_number`: The zero-based index of the trace that was clicked.
/// - `point_numbers`: An optional list of indices if multiple points were selected.
/// - `point_number`: The index of the specific point clicked (if singular).
/// - `x`, `y`, `z`: Optional numeric coordinates in data space.
/// - `lat`, `lon`: Optional geographic coordinates (for map plots).
///
/// # Serialization
/// Uses `serde` with `camelCase` field names to match Plotly’s JS API.


#[derive(Debug,Deserialize,Serialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct ClickPoint {
    pub curve_number: usize,
    pub point_numbers: Option<Vec<usize>>,
    pub point_number: Option<usize>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>
}


/// Provide a default single-point vector for `ClickEvent::points`.
///
/// Returns `vec![ClickPoint::default()]` so deserialization always yields
/// at least one element rather than an empty vector.

fn default_click_event() -> Vec<ClickPoint> {vec![ClickPoint::default()]}


/// The top-level payload for a Plotly click event.
///
/// - `points`: A `Vec<ClickPoint>` containing all clicked points.
///   Defaults to the result of `default_click_event` to ensure
///   `points` is non-empty even if Plotly sends no data.
///
/// # Serialization
/// Uses `serde` with `camelCase` names and a custom default so you can
/// call `event.points` without worrying about missing values.

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all="camelCase",default)]
pub struct ClickEvent {
    #[serde(default="default_click_event")]
    pub points: Vec<ClickPoint>
}

/// A `Default` implementation yielding an empty `points` vector.
///
/// Useful when you need a zero-event placeholder (e.g., initial state).

impl Default for ClickEvent {
    fn default() -> Self {
        ClickEvent { points: vec![] }
    }
}