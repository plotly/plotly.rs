mod layout;
mod page;
mod plotly_chart;

use proa_core::WebContext;
use proa_framework_core::WebContextDocumentExt;

use crate::{layout::PlotLayout, page::MultiplePlotsPage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cx = WebContext::new();
    cx.render(PlotLayout {
        class: None,
        children: Some(MultiplePlotsPage::new()?),
    })?;

    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", cx.finish()?)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}
