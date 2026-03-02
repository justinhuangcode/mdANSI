use comrak::nodes::TableAlignment;

use crate::style::Style;
use crate::wrap::{
    center_in_width, pad_to_width, right_align_in_width, truncate_to_width, visible_width_of,
};

/// Unicode box-drawing characters for table borders.
pub mod border {
    pub const TOP_LEFT: &str = "\u{250c}"; // ┌
    pub const TOP_RIGHT: &str = "\u{2510}"; // ┐
    pub const BOTTOM_LEFT: &str = "\u{2514}"; // └
    pub const BOTTOM_RIGHT: &str = "\u{2518}"; // ┘
    pub const HORIZONTAL: &str = "\u{2500}"; // ─
    pub const VERTICAL: &str = "\u{2502}"; // │
    pub const T_DOWN: &str = "\u{252c}"; // ┬
    pub const T_UP: &str = "\u{2534}"; // ┴
    pub const T_RIGHT: &str = "\u{251c}"; // ├
    pub const T_LEFT: &str = "\u{2524}"; // ┤
    pub const CROSS: &str = "\u{253c}"; // ┼
}

/// Table border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Unicode,
    Ascii,
    None,
}

/// Configuration for table rendering.
#[derive(Debug, Clone)]
pub struct TableConfig {
    pub max_width: usize,
    pub border_style: BorderStyle,
    pub border_style_ansi: Style,
    pub header_style: Style,
    pub cell_style: Style,
    pub padding: usize,
    pub truncate: bool,
    pub max_col_width: usize,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            max_width: 80,
            border_style: BorderStyle::Unicode,
            border_style_ansi: Style::new().dim(),
            header_style: Style::new().bold(),
            cell_style: Style::new(),
            padding: 1,
            truncate: true,
            max_col_width: 40,
        }
    }
}

/// A parsed table ready for rendering.
#[derive(Debug)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub alignments: Vec<TableAlignment>,
}

impl Table {
    pub fn new(
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        alignments: Vec<TableAlignment>,
    ) -> Self {
        Self {
            headers,
            rows,
            alignments,
        }
    }

    /// Render the table to ANSI-styled lines.
    pub fn render(&self, config: &TableConfig) -> Vec<String> {
        if self.headers.is_empty() {
            return Vec::new();
        }

        let _num_cols = self.headers.len();
        let col_widths = self.compute_col_widths(config);

        match config.border_style {
            BorderStyle::Unicode => self.render_unicode(&col_widths, config),
            BorderStyle::Ascii => self.render_ascii(&col_widths, config),
            BorderStyle::None => self.render_borderless(&col_widths, config),
        }
    }

    /// Compute optimal column widths that fit within max_width.
    fn compute_col_widths(&self, config: &TableConfig) -> Vec<usize> {
        let num_cols = self.headers.len();
        let padding = config.padding;

        // Minimum column width: ensure even single-char content is visible
        let min_col_width = 3;

        // Measure natural widths (enforce minimum)
        let mut widths: Vec<usize> = self
            .headers
            .iter()
            .map(|h| {
                visible_width_of(h)
                    .min(config.max_col_width)
                    .max(min_col_width)
            })
            .collect();

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(
                        visible_width_of(cell)
                            .min(config.max_col_width)
                            .max(min_col_width),
                    );
                }
            }
        }

        // Calculate total width with borders and padding
        // Format: │ pad cell pad │ pad cell pad │ ...
        let border_overhead = num_cols + 1; // vertical bars
        let padding_overhead = num_cols * padding * 2; // padding on each side of each cell
        let content_budget = config
            .max_width
            .saturating_sub(border_overhead + padding_overhead);

        let total_content: usize = widths.iter().sum();

        if total_content <= content_budget {
            return widths;
        }

        // Iteratively shrink the widest columns
        for _ in 0..100 {
            let total: usize = widths.iter().sum();
            if total <= content_budget {
                break;
            }

            // Find the widest column
            let max_width = *widths.iter().max().unwrap_or(&0);
            if max_width <= min_col_width {
                break;
            }

            // Shrink all columns that are at the max width
            for w in widths.iter_mut() {
                if *w == max_width {
                    *w = (*w - 1).max(min_col_width);
                }
            }
        }

        widths
    }

    fn render_unicode(&self, col_widths: &[usize], config: &TableConfig) -> Vec<String> {
        let mut lines = Vec::new();

        // Top border: ┌───┬───┐
        lines.push(self.horizontal_line(
            border::TOP_LEFT,
            border::T_DOWN,
            border::TOP_RIGHT,
            col_widths,
            config,
        ));

        // Header row
        lines.push(self.render_row(&self.headers, col_widths, config, &config.header_style));

        // Header separator: ├───┼───┤
        lines.push(self.horizontal_line(
            border::T_RIGHT,
            border::CROSS,
            border::T_LEFT,
            col_widths,
            config,
        ));

        // Data rows
        for row in &self.rows {
            lines.push(self.render_row(row, col_widths, config, &config.cell_style));
        }

        // Bottom border: └───┴───┘
        lines.push(self.horizontal_line(
            border::BOTTOM_LEFT,
            border::T_UP,
            border::BOTTOM_RIGHT,
            col_widths,
            config,
        ));

        lines
    }

    fn render_ascii(&self, col_widths: &[usize], config: &TableConfig) -> Vec<String> {
        let mut lines = Vec::new();

        // Top border: +---+---+
        lines.push(self.horizontal_line_ascii("+", "+", "+", col_widths, config));

        // Header
        lines.push(self.render_row_ascii(&self.headers, col_widths, config, &config.header_style));

        // Separator: +---+---+
        lines.push(self.horizontal_line_ascii("+", "+", "+", col_widths, config));

        // Rows
        for row in &self.rows {
            lines.push(self.render_row_ascii(row, col_widths, config, &config.cell_style));
        }

        // Bottom: +---+---+
        lines.push(self.horizontal_line_ascii("+", "+", "+", col_widths, config));

        lines
    }

    fn render_borderless(&self, col_widths: &[usize], config: &TableConfig) -> Vec<String> {
        let mut lines = Vec::new();

        // Header
        let header_cells: Vec<String> = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let w = col_widths.get(i).copied().unwrap_or(10);
                let content = if config.truncate {
                    truncate_to_width(h, w)
                } else {
                    h.clone()
                };
                let aligned = self.align_cell(&content, w, i);
                config.header_style.paint(&aligned)
            })
            .collect();
        lines.push(header_cells.join("  "));

        // Underline
        let underline: Vec<String> = col_widths
            .iter()
            .map(|w| config.border_style_ansi.paint(&"\u{2500}".repeat(*w)))
            .collect();
        lines.push(underline.join("  "));

        // Rows
        for row in &self.rows {
            let cells: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let w = col_widths.get(i).copied().unwrap_or(10);
                    let content = if config.truncate {
                        truncate_to_width(cell, w)
                    } else {
                        cell.clone()
                    };
                    self.align_cell(&content, w, i)
                })
                .collect();
            lines.push(cells.join("  "));
        }

        lines
    }

    fn horizontal_line(
        &self,
        left: &str,
        mid: &str,
        right: &str,
        col_widths: &[usize],
        config: &TableConfig,
    ) -> String {
        let bs = &config.border_style_ansi;
        let pad = config.padding;
        let segments: Vec<String> = col_widths
            .iter()
            .map(|w| border::HORIZONTAL.repeat(w + pad * 2))
            .collect();
        bs.paint(&format!("{}{}{}", left, segments.join(mid), right))
    }

    fn horizontal_line_ascii(
        &self,
        left: &str,
        mid: &str,
        right: &str,
        col_widths: &[usize],
        config: &TableConfig,
    ) -> String {
        let bs = &config.border_style_ansi;
        let pad = config.padding;
        let segments: Vec<String> = col_widths.iter().map(|w| "-".repeat(w + pad * 2)).collect();
        bs.paint(&format!("{}{}{}", left, segments.join(mid), right))
    }

    fn render_row(
        &self,
        cells: &[String],
        col_widths: &[usize],
        config: &TableConfig,
        cell_style: &Style,
    ) -> String {
        let bs = &config.border_style_ansi;
        let pad = config.padding;
        let padding_str = " ".repeat(pad);

        let formatted_cells: Vec<String> = col_widths
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let content = cells.get(i).map(|s| s.as_str()).unwrap_or("");
                let content = if config.truncate {
                    truncate_to_width(content, *w)
                } else {
                    content.to_string()
                };
                let aligned = self.align_cell(&content, *w, i);
                format!(
                    "{}{}{}",
                    padding_str,
                    cell_style.paint(&aligned),
                    padding_str
                )
            })
            .collect();

        let v = bs.paint(border::VERTICAL);
        format!("{}{}{}", v, formatted_cells.join(&v), v)
    }

    fn render_row_ascii(
        &self,
        cells: &[String],
        col_widths: &[usize],
        config: &TableConfig,
        cell_style: &Style,
    ) -> String {
        let bs = &config.border_style_ansi;
        let pad = config.padding;
        let padding_str = " ".repeat(pad);

        let formatted_cells: Vec<String> = col_widths
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let content = cells.get(i).map(|s| s.as_str()).unwrap_or("");
                let content = if config.truncate {
                    truncate_to_width(content, *w)
                } else {
                    content.to_string()
                };
                let aligned = self.align_cell(&content, *w, i);
                format!(
                    "{}{}{}",
                    padding_str,
                    cell_style.paint(&aligned),
                    padding_str
                )
            })
            .collect();

        let v = bs.paint("|");
        format!("{}{}{}", v, formatted_cells.join(&v), v)
    }

    fn align_cell(&self, content: &str, width: usize, col_index: usize) -> String {
        let alignment = self
            .alignments
            .get(col_index)
            .copied()
            .unwrap_or(TableAlignment::None);

        match alignment {
            TableAlignment::Left | TableAlignment::None => pad_to_width(content, width),
            TableAlignment::Center => center_in_width(content, width),
            TableAlignment::Right => right_align_in_width(content, width),
        }
    }
}
