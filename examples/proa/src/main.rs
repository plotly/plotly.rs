mod chart;
mod layout;
mod page;
mod plots;

use crate::{layout::render_page, page::MultiplePlotsPage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = render_page(MultiplePlotsPage::new()?)?;
    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", html)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}
