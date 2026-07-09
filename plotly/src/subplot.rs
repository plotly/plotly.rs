use std::collections::HashMap;

use crate::common::Anchor;
use crate::layout::{Annotation, Axis, GridPattern, Layout, LayoutGrid, RowOrder};
use crate::Plot;

#[derive(Debug, Clone, Copy)]
pub enum StartCell {
    TopLeft,
    BottomLeft,
}

#[derive(Clone)]
pub struct SubplotsBuilder {
    rows: usize,
    cols: usize,
    start_cell: StartCell,
    shared_xaxes: bool,
    shared_yaxes: bool,
    column_widths: Option<Vec<f64>>,
    row_heights: Option<Vec<f64>>,
    horizontal_spacing: Option<f64>,
    vertical_spacing: Option<f64>,
    subplot_titles: Vec<String>,
    x_axis_titles: Option<Vec<String>>,
    y_axis_titles: Option<Vec<String>>,
    axis_index_by_cell: HashMap<(usize, usize), usize>,
    next_index: usize,
    plot: Plot,
    layout: Layout,
    layout_initialized: bool,
    layout_relayout: bool,
    base_annotations: Option<Vec<crate::layout::Annotation>>,
    x_title_by_cell: std::collections::HashMap<(usize, usize), String>,
    y_title_by_cell: std::collections::HashMap<(usize, usize), String>,
}

impl SubplotsBuilder {
    pub fn new(rows: usize, cols: usize) -> Self {
        let layout = Layout::new().grid(
            LayoutGrid::new()
                .rows(rows)
                .columns(cols)
                .pattern(GridPattern::Independent)
                .row_order(RowOrder::TopToBottom),
        );
        Self {
            rows,
            cols,
            start_cell: StartCell::TopLeft,
            shared_xaxes: false,
            shared_yaxes: false,
            column_widths: None,
            row_heights: None,
            horizontal_spacing: None,
            vertical_spacing: None,
            subplot_titles: Vec::new(),
            x_axis_titles: None,
            y_axis_titles: None,
            axis_index_by_cell: HashMap::new(),
            next_index: 1,
            plot: Plot::new(),
            layout,
            layout_initialized: false,
            layout_relayout: true,
            base_annotations: None,
            x_title_by_cell: std::collections::HashMap::new(),
            y_title_by_cell: std::collections::HashMap::new(),
        }
    }

    pub fn make_subplots(self) -> Plot {
        self.into()
    }

    pub fn start_cell(mut self, start_cell: StartCell) -> Self {
        self.layout_relayout = true;
        self.start_cell = start_cell;
        let row_order = match start_cell {
            StartCell::TopLeft => RowOrder::TopToBottom,
            StartCell::BottomLeft => RowOrder::BottomToTop,
        };
        self.layout = self.layout.grid(LayoutGrid::new().row_order(row_order));
        self
    }

    pub fn shared_xaxes(mut self, shared: bool) -> Self {
        self.layout_relayout = true;
        self.shared_xaxes = shared;
        self
    }

    pub fn shared_yaxes(mut self, shared: bool) -> Self {
        self.layout_relayout = true;
        self.shared_yaxes = shared;
        self
    }

    pub fn subplot_titles<T: Into<String>>(mut self, titles: Vec<T>) -> Self {
        self.layout_relayout = true;
        self.subplot_titles = titles.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn horizontal_spacing(mut self, spacing: f64) -> Self {
        self.layout_relayout = true;
        self.horizontal_spacing = Some(spacing);
        self
    }

    pub fn vertical_spacing(mut self, spacing: f64) -> Self {
        self.layout_relayout = true;
        self.vertical_spacing = Some(spacing);
        self
    }

    pub fn x_axis_titles<T: Into<String>>(mut self, titles: Vec<T>) -> Self {
        self.layout_relayout = true;
        self.x_axis_titles = Some(titles.into_iter().map(|t| t.into()).collect());
        self
    }

    pub fn y_axis_titles<T: Into<String>>(mut self, titles: Vec<T>) -> Self {
        self.layout_relayout = true;
        self.y_axis_titles = Some(titles.into_iter().map(|t| t.into()).collect());
        self
    }

    pub fn spacing(mut self, horizontal: f64, vertical: f64) -> Self {
        self.layout_relayout = true;
        self.horizontal_spacing = Some(horizontal);
        self.vertical_spacing = Some(vertical);
        self
    }

    // Per-axis title setters (explicit cell targeting similar to Plotly.py
    // update_xaxes/yaxes)
    pub fn x_title_at<T: Into<String>>(mut self, row: usize, col: usize, title: T) -> Self {
        self.layout_relayout = true;
        self.x_title_by_cell.insert((row, col), title.into());
        self
    }

    pub fn y_title_at<T: Into<String>>(mut self, row: usize, col: usize, title: T) -> Self {
        self.layout_relayout = true;
        self.y_title_by_cell.insert((row, col), title.into());
        self
    }

    fn axis_name(kind: char, idx: usize) -> String {
        if idx == 1 {
            kind.to_string()
        } else {
            format!("{kind}{idx}")
        }
    }
    fn update_layout(&mut self) {
        if !self.layout_initialized || self.layout_relayout {
            self.axis_index_by_cell.clear();
            self.next_index = 1;

            self.evaluate_subplots_layout();

            self.layout_initialized = true;
            self.layout_relayout = false;
        }
    }

    fn evaluate_subplots_layout(&mut self) {
        // Calculate domains with proper spacing and sensible defaults.
        // Accept any non-negative spacing but cap it so we always leave
        // a minimum subplot fraction
        const MIN_SUBPLOT_FRACTION: f64 = 0.12; // ~12% per subplot

        let row_iter: Box<dyn Iterator<Item = usize>> = match self.start_cell {
            StartCell::BottomLeft => Box::new(1..=self.rows),
            StartCell::TopLeft => Box::new((1..=self.rows).rev()),
        };

        for r in row_iter {
            for c in 1..=self.cols {
                let idx = self.next_index;
                self.axis_index_by_cell.insert((r, c), idx);
                let mut x = Axis::new();
                let mut y = Axis::new();

                let req_h = self.horizontal_spacing.unwrap_or(0.0).max(0.0);
                let req_v = self.vertical_spacing.unwrap_or(0.0).max(0.0);

                // Max spacing that still leaves MIN_SUBPLOT_FRACTION per subplot
                let max_h = if self.cols > 1 {
                    ((1.0 - self.cols as f64 * MIN_SUBPLOT_FRACTION).max(0.0))
                        / (self.cols as f64 - 1.0)
                } else {
                    0.0
                };
                let max_v = if self.rows > 1 {
                    ((1.0 - self.rows as f64 * MIN_SUBPLOT_FRACTION).max(0.0))
                        / (self.rows as f64 - 1.0)
                } else {
                    0.0
                };

                let h_spacing = if self.cols > 1 { req_h.min(max_h) } else { 0.0 };
                let v_spacing = if self.rows > 1 { req_v.min(max_v) } else { 0.0 };

                // Subplot dimensions
                let total_h_gaps = h_spacing * (self.cols - 1) as f64;
                let total_v_gaps = v_spacing * (self.rows - 1) as f64;
                let subplot_width = (1.0 - total_h_gaps) / self.cols as f64;
                let subplot_height = (1.0 - total_v_gaps) / self.rows as f64;

                // Calculate domain positions
                let x_domain_start = (c - 1) as f64 * (subplot_width + h_spacing);
                let x_domain_end = x_domain_start + subplot_width;
                let y_domain_start = (r - 1) as f64 * (subplot_height + v_spacing);
                let y_domain_end = y_domain_start + subplot_height;

                x = x.domain(&[x_domain_start, x_domain_end]);
                y = y.domain(&[y_domain_start, y_domain_end]);

                if self.shared_xaxes && r > 1 {
                    let base = *self.axis_index_by_cell.get(&(1, c)).unwrap();
                    x = x.matches(&Self::axis_name('x', base));
                }
                if self.shared_yaxes && c > 1 {
                    let base = *self.axis_index_by_cell.get(&(r, 1)).unwrap();
                    y = y.matches(&Self::axis_name('y', base));
                }

                // Explicit per-axis titles take precedence
                if let Some(t) = self.x_title_by_cell.get(&(r, c)) {
                    x = x.title(t.clone());
                } else {
                    // Default: only bottom row gets column titles
                    let is_bottom_row = match self.start_cell {
                        StartCell::TopLeft => r == 1,
                        StartCell::BottomLeft => r == self.rows,
                    };
                    if is_bottom_row {
                        if let Some(ref x_titles) = self.x_axis_titles {
                            if let Some(title) = x_titles.get(c - 1) {
                                x = x.title(title.clone());
                            }
                        }
                    }
                }
                if let Some(t) = self.y_title_by_cell.get(&(r, c)) {
                    y = y.title(t.clone());
                } else {
                    // Default: only left column gets row titles
                    if c == 1 {
                        if let Some(ref y_titles) = self.y_axis_titles {
                            if let Some(title) = y_titles.get(r - 1) {
                                y = y.title(title.clone());
                            }
                        }
                    }
                }
                self.layout.set_x_axis(idx, x).set_y_axis(idx, y);
                self.next_index += 1;
            }
        }

        // Update grid configuration (spacing is handled by domain calculation above)
        let grid = LayoutGrid::new()
            .rows(self.rows)
            .columns(self.cols)
            .pattern(GridPattern::Independent)
            .row_order(match self.start_cell {
                StartCell::TopLeft => RowOrder::TopToBottom,
                StartCell::BottomLeft => RowOrder::BottomToTop,
            });
        self.layout = self.layout.clone().grid(grid);

        let mut title_annotations: Vec<Annotation> = Vec::new();

        if !self.subplot_titles.is_empty() {
            let total = self.rows * self.cols;
            for (i, title) in self.subplot_titles.iter().enumerate().take(total) {
                let (r, c) = match self.start_cell {
                    StartCell::BottomLeft => (1 + i / self.cols, 1 + i % self.cols),
                    StartCell::TopLeft => (self.rows - (i / self.cols), 1 + i % self.cols),
                };
                let idx = *self.axis_index_by_cell.get(&(r, c)).unwrap();
                let xref = format!("{} domain", Self::axis_name('x', idx));
                let yref = format!("{} domain", Self::axis_name('y', idx));
                title_annotations.push(
                    Annotation::new()
                        .x_ref(xref)
                        .y_ref(yref)
                        .x(0.5)
                        .y(1.0)
                        .x_anchor(Anchor::Center)
                        .y_anchor(Anchor::Bottom)
                        .text(title.clone())
                        .show_arrow(false),
                );
            }
            let user = self.base_annotations.clone().unwrap_or_default();
            let mut final_annotations = user;
            final_annotations.extend(title_annotations);
            self.layout = self.layout.clone().annotations(final_annotations);
            if self.base_annotations.is_none() {
                self.base_annotations = Some(Vec::new());
            }
        }
    }

    pub fn add_trace<T>(&mut self, trace: T, row: usize, col: usize) -> &mut Self
    where
        T: crate::subplot::HasXYAxes + crate::Trace + Clone + 'static,
    {
        self.update_layout();
        let idx = *self
            .axis_index_by_cell
            .get(&(row, col))
            .expect("Row/col out of range");
        let trace = trace
            .with_x_axis(Self::axis_name('x', idx))
            .with_y_axis(Self::axis_name('y', idx));
        self.plot.add_trace(Box::new(trace));
        self
    }
}

impl From<SubplotsBuilder> for Plot {
    fn from(mut ms: SubplotsBuilder) -> Self {
        ms.update_layout();
        ms.plot.set_layout(ms.layout);
        ms.plot
    }
}

// Helper trait for 2D cartesian traces
pub trait HasXYAxes {
    fn with_x_axis(self, axis: impl Into<String>) -> Self;
    fn with_y_axis(self, axis: impl Into<String>) -> Self;
}

// Implement for common traces;
// TODO: add the rest later as needed
impl<X, Y> HasXYAxes for Box<crate::traces::Scatter<X, Y>>
where
    X: serde::Serialize + Clone + 'static,
    Y: serde::Serialize + Clone + 'static,
{
    fn with_x_axis(self, a: impl Into<String>) -> Self {
        self.x_axis(a.into())
    }
    fn with_y_axis(self, a: impl Into<String>) -> Self {
        self.y_axis(a.into())
    }
}

impl<X, Y> HasXYAxes for Box<crate::traces::Bar<X, Y>>
where
    X: serde::Serialize + Clone + 'static,
    Y: serde::Serialize + Clone + 'static,
{
    fn with_x_axis(self, a: impl Into<String>) -> Self {
        self.x_axis(a.into())
    }
    fn with_y_axis(self, a: impl Into<String>) -> Self {
        self.y_axis(a.into())
    }
}

// Implement Trace for Box<T> where T: Trace to make Box<Trace> work seamlessly
impl<T: crate::Trace + serde::Serialize + Clone> crate::Trace for Box<T> {
    fn to_json(&self) -> String {
        self.as_ref().to_json()
    }
}
