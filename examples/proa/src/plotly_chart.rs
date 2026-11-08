//! A local adapter: borrow a Plot and serialize it into the page's output.

use std::io;

use plotly::Plot;
use proa_core::{DataLoader, RenderBuffer, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;
use serde::Serialize;

pub struct PlotlyChart<'a> {
    pub id: &'a str,
    pub plot: &'a Plot,
    pub class: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for PlotlyChart<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div id={self.id} data-plotly-chart class={self.class}></div>
            <template data-plotly-seed>{HtmlJson(self.plot)}</template>
        }
        .render(cx)
    }
}

// Keep the template content as JSON, with no HTML elements or entities. Write
// directly into Proa's buffer, encoding HTML-sensitive bytes as JSON escapes.
struct HtmlJson<'a, T>(&'a T);

impl<L: DataLoader, T: Serialize> WebRenderSync<L> for HtmlJson<'_, T> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        serde_json::to_writer(HtmlJsonWriter(cx.output_mut()), self.0)
            // proa-perf-ok: allocate a diagnostic only on serialization failure
            .map_err(|error| WriteError::render(error.to_string()))
    }
}

struct HtmlJsonWriter<'a>(&'a mut RenderBuffer);

impl io::Write for HtmlJsonWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut start = 0;
        for (index, byte) in bytes.iter().enumerate() {
            let escaped: &[u8] = match byte {
                b'<' => b"\\u003c",
                b'>' => b"\\u003e",
                b'&' => b"\\u0026",
                b'\'' => b"\\u0027",
                _ => continue,
            };
            self.0
                .extend(&bytes[start..index])
                .map_err(io::Error::other)?;
            self.0.extend(escaped).map_err(io::Error::other)?;
            start = index + 1;
        }
        self.0.extend(&bytes[start..]).map_err(io::Error::other)?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_seed_preserves_data_without_html_boundaries() {
        let value = "</template></script><script>injected()</script><!-- & ' \\\" café 🦀";
        let mut cx = WebContext::new();
        HtmlJson(&value).render(&mut cx).unwrap();
        let json = cx.into_output().into_string();
        assert!(!json.contains(['<', '>', '&', '\'']));
        assert_eq!(serde_json::from_str::<String>(&json).unwrap(), value);
    }

    #[test]
    fn chart_ids_are_attributes_and_never_interpolated_into_javascript() {
        let plot = Plot::new();
        let mut cx = WebContext::new();
        PlotlyChart {
            id: "chart\"<&\\name",
            plot: &plot,
            class: None,
        }
        .render(&mut cx)
        .unwrap();
        let html = cx.into_output().into_string();
        assert!(html.contains("&quot;"));
        assert!(!html.contains("id=\"chart\"<"));
        let seed = html.split("data-plotly-seed>").nth(1).unwrap();
        let json = seed.split("</template>").next().unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(json).unwrap(),
            serde_json::to_value(plot).unwrap()
        );
    }
}
