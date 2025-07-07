// Utility functions for printing PTCR records with source context
use std::fs;
use std::path::{Path};

use anyhow::{anyhow, Result};

use crate::{read_file, Record, Span};

/// Print all records from a PTCR file with surrounding code context.
pub fn print_ptcr_file(ptcr_file: &Path, context: usize) -> Result<()> {
    let records = read_file(ptcr_file)?;
    let base = ptcr_file.parent().unwrap_or_else(|| Path::new(""));
    for rec in records {
        let file_path = base.join(&rec.path);
        print_record(&file_path, &rec, context)?;
        println!();
    }
    Ok(())
}

fn print_record(path: &Path, rec: &Record, context: usize) -> Result<()> {
    let contents = fs::read_to_string(path)
        .map_err(|e| anyhow!("failed to read {}: {e}", path.display()))?;
    let lines: Vec<&str> = contents.lines().collect();
    let (start, end) = line_bounds(&rec.span, lines.len());
    let start = start.saturating_sub(1 + context);
    let end = (end + context).min(lines.len());

    println!("{}:{}", rec.path.display(), span_to_string(&rec.span));
    for i in start..end {
        println!("{:>5} | {}", i + 1, lines[i]);
    }
    println!("---");
    for line in &rec.body {
        println!("{}", line);
    }
    Ok(())
}

fn line_bounds(span: &Span, max: usize) -> (usize, usize) {
    let (mut start, mut end) = match span {
        Span::File => (1, max),
        Span::Line(l) => (*l, *l),
        Span::Point { line, .. } => (*line, *line),
        Span::LineRange { start, end } => (*start, *end),
        Span::ColumnRange { line, .. } => (*line, *line),
        Span::MultiLine { start_line, end_line, .. } => (*start_line, *end_line),
    };
    if end > max {
        end = max;
    }
    (start, end)
}

fn span_to_string(span: &Span) -> String {
    match span {
        Span::File => "*".to_string(),
        Span::Line(l) => format!("{}", l),
        Span::Point { line, col } => format!("{}.{}", line, col),
        Span::LineRange { start, end } => format!("{}-{}", start, end),
        Span::ColumnRange { line, start_col, end_col } => {
            format!("{}.{}-{}.{}", line, start_col, line, end_col)
        }
        Span::MultiLine { start_line, start_col, end_line, end_col } => {
            format!("{}.{}-{}.{}", start_line, start_col, end_line, end_col)
        }
    }
}
