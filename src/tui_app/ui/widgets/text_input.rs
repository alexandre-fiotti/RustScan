//! Text input widget (render-only) that pairs naturally with shared::TextInput

use ratatui::{
    layout::{Position, Rect},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui_app::shared::TextInput;
use crate::tui_app::ui::theme::{
    active_style, normal_text_style, placeholder_style, title_selected_style,
    title_unselected_style, BORDER_NORMAL,
};

pub struct TextInputWidget<'a> {
    pub title: &'a str,
    pub text: String,
    pub is_selected: bool,
    pub is_placeholder: bool,
    pub cursor_index: Option<usize>,
}

impl<'a> TextInputWidget<'a> {
    pub fn new(
        title: &'a str,
        text: String,
        is_selected: bool,
        is_placeholder: bool,
        cursor_index: Option<usize>,
    ) -> Self {
        Self {
            title,
            text,
            is_selected,
            is_placeholder,
            cursor_index,
        }
    }

    pub fn from_model(
        title: &'a str,
        buffer: &TextInput,
        confirmed_display: Option<String>,
        is_selected: bool,
        placeholder: &'a str,
    ) -> Self {
        let (text, is_placeholder) = if !buffer.is_empty() {
            (buffer.text().to_string(), false)
        } else if let Some(display) = confirmed_display {
            (display, false)
        } else {
            (placeholder.to_string(), true)
        };

        Self::new(
            title,
            text,
            is_selected,
            is_placeholder,
            Some(buffer.cursor()),
        )
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let (border_style, title_style) = if self.is_selected {
            (active_style(), title_selected_style())
        } else {
            (
                ratatui::style::Style::default().fg(BORDER_NORMAL),
                title_unselected_style(),
            )
        };

        let content_style = if self.is_placeholder {
            placeholder_style()
        } else {
            normal_text_style()
        };

        let widget = Paragraph::new(self.text.clone())
            .style(content_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(self.title, title_style))
                    .border_style(border_style)
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            );

        f.render_widget(widget, area);

        if self.is_selected {
            if let Some(cursor) = self.cursor_index {
                f.set_cursor_position(Position::new(area.x + cursor as u16 + 2, area.y + 1));
            }
        }
    }
}
