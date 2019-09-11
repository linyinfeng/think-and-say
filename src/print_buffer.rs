use std::fmt;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrintLocation<'a> {
    Transparent,
    GraphemeBefore,
    Grapheme(&'a str),
}

#[derive(Debug, Clone)]
pub struct PrintBuffer<'a> {
    pub lines: Vec<Vec<PrintLocation<'a>>>,
}

impl<'a> Default for PrintBuffer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PrintBuffer<'a> {
    pub fn new() -> Self {
        PrintBuffer { lines: Vec::new() }
    }

    pub fn set_grapheme(&mut self, line_index: usize, column_index: usize, grapheme: &'a str) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);

        if line_index >= self.lines.len() {
            self.lines.resize(line_index + 1, Vec::new());
        }
        {
            let line = &mut self.lines[line_index];
            if column_index >= line.len() {
                line.resize(column_index + grapheme_width, PrintLocation::Transparent);
            }
        }

        for c in column_index..column_index + grapheme_width {
            self.clear_location(line_index, c);
        }
        for c in column_index..column_index + grapheme_width {
            if c == column_index {
                self.lines[line_index][c] = PrintLocation::Grapheme(grapheme);
            } else {
                self.lines[line_index][c] = PrintLocation::GraphemeBefore;
            }
        }
    }

    pub fn clear_location(&mut self, line_index: usize, column_index: usize) {
        log::trace!(
            "clear location: line_index = {}, column_index = {}",
            line_index,
            column_index
        );
        if line_index >= self.lines.len() {
            return;
        }
        let line = &mut self.lines[line_index];
        if column_index >= line.len() {
            return;
        }
        match line[column_index] {
            PrintLocation::Transparent => (),
            PrintLocation::Grapheme(g) => {
                let w = UnicodeWidthStr::width(g);
                for l in line.iter_mut().skip(column_index).take(w) {
                    *l = PrintLocation::Transparent;
                }
            }
            PrintLocation::GraphemeBefore => {
                log::trace!(
                    "find grapheme: line_index = {}, column_index = {}",
                    line_index,
                    column_index
                );
                let grapheme_column_index = self.find_grapheme(line_index, column_index).unwrap();
                self.clear_location(line_index, grapheme_column_index);
            }
        }
    }

    fn find_grapheme(&mut self, line_index: usize, mut column_index: usize) -> Option<usize> {
        if line_index >= self.lines.len() {
            return None;
        }
        let line = &mut self.lines[line_index];
        if column_index >= line.len() {
            return None;
        }
        match line[column_index] {
            PrintLocation::Transparent => None,
            PrintLocation::Grapheme(_) => Some(column_index),
            PrintLocation::GraphemeBefore => loop {
                column_index -= 1;
                match line.get(column_index).expect("invalid GraphemeBefore") {
                    PrintLocation::Transparent => panic!("invalid grapheme"),
                    PrintLocation::Grapheme(_) => break Some(column_index),
                    PrintLocation::GraphemeBefore => (),
                }
            },
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        let mut buffer = Self::new();
        s.lines()
            .enumerate()
            .flat_map(|(line_index, line)| {
                UnicodeSegmentation::graphemes(line, true).scan(0, move |index, grapheme| {
                    let w = UnicodeWidthStr::width(grapheme);
                    let current_index = *index;
                    *index += w;
                    Some((line_index, current_index, w, grapheme))
                })
            })
            .for_each(|(line_index, column_index, width, grapheme)| {
                log::trace!(
                    "set grapheme: line_index = {}, column_index = {}, width = {}, grapheme = {}",
                    line_index,
                    column_index,
                    width,
                    grapheme
                );
                buffer.set_grapheme(line_index, column_index, grapheme)
            });
        buffer
    }

    pub fn lines_len(&self) -> usize {
        self.lines.len()
    }

    pub fn columns_len(&self) -> usize {
        self.lines.iter().map(|s| s.len()).max().unwrap_or(0)
    }
}

impl<'a> fmt::Display for PrintBuffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.lines {
            for p in line {
                match p {
                    PrintLocation::Transparent => write!(f, " ")?,
                    PrintLocation::Grapheme(g) => write!(f, "{}", g)?,
                    PrintLocation::GraphemeBefore => (), // just skip
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
