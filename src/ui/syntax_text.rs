use ratatui::text::{Line, Span};
use std::ops::Range;
use syntect::{
    highlighting::{
        FontStyle, HighlightState, Highlighter, RangedHighlightIterator, Style, ThemeSet,
    },
    parsing::{ParseState, ScopeStack, SyntaxSet},
};

struct SyntaxLine {
    items: Vec<(Style, usize, Range<usize>)>,
}

pub struct SyntaxText {
    text: String,
    lines: Vec<SyntaxLine>,
}

impl SyntaxText {
    pub fn new(text: String) -> Self {
        let syntax_set: SyntaxSet = SyntaxSet::load_defaults_nonewlines();
        let theme_set: ThemeSet = ThemeSet::load_defaults();

        let mut state = ParseState::new(syntax_set.find_syntax_by_extension("sql").unwrap());
        let highlighter = Highlighter::new(&theme_set.themes["base16-eighties.dark"]);
        let mut syntax_lines: Vec<SyntaxLine> = Vec::new();
        let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());

        for (number, line) in text.lines().enumerate() {
            let ops = state.parse_line(line, &syntax_set);
            if let Ok(vec) = ops {
                let iter =
                    RangedHighlightIterator::new(&mut highlight_state, &vec, line, &highlighter);
                syntax_lines.push(SyntaxLine {
                    items: iter
                        .map(|(style, _, range)| (style, number, range))
                        .collect(),
                });
            }
        }

        Self {
            text,
            lines: syntax_lines,
        }
    }

    pub fn convert(&self) -> ratatui::text::Text<'_> {
        let mut result_lines: Vec<Line> = Vec::with_capacity(self.lines.len());

        for (syntax_line, line_content) in self.lines.iter().zip(self.text.lines()) {
            let mut line_span = Line::from(Vec::with_capacity(syntax_line.items.len()));

            for (style, _, range) in &syntax_line.items {
                let item_content = &line_content[range.clone()];
                let item_style = syntact_style_to_tui(style);

                line_span.spans.push(Span::styled(item_content, item_style));
            }

            result_lines.push(line_span);
        }

        result_lines.into()
    }
}

impl<'a> From<&'a SyntaxText> for ratatui::text::Text<'a> {
    fn from(v: &'a SyntaxText) -> Self {
        let mut result_lines: Vec<Line> = Vec::with_capacity(v.lines.len());

        for (syntax_line, line_content) in v.lines.iter().zip(v.text.lines()) {
            let mut line_span = Line::from(Vec::with_capacity(syntax_line.items.len()));

            for (style, _, range) in &syntax_line.items {
                let item_content = &line_content[range.clone()];
                let item_style = syntact_style_to_tui(style);

                line_span.spans.push(Span::styled(item_content, item_style));
            }

            result_lines.push(line_span);
        }

        result_lines.into()
    }
}

fn syntact_style_to_tui(style: &Style) -> ratatui::style::Style {
    let mut res = ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ));

    if style.font_style.contains(FontStyle::BOLD) {
        res = res.add_modifier(ratatui::style::Modifier::BOLD);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        res = res.add_modifier(ratatui::style::Modifier::ITALIC);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        res = res.add_modifier(ratatui::style::Modifier::UNDERLINED);
    }

    res
}
