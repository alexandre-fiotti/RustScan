//! Button widget (UI-only)

use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::tui_app::shared::button_mode::ButtonMode;
use crate::tui_app::ui::theme::{
    button_active_background, button_active_highlight, button_active_shadow,
    button_normal_background, button_normal_highlight, button_normal_shadow,
    button_selected_background, button_selected_highlight, button_selected_shadow,
};

#[derive(Debug, Clone)]
pub struct ButtonWidget<'a> {
    label: &'a str,
    mode: ButtonMode,
}

impl<'a> ButtonWidget<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            mode: ButtonMode::Normal,
        }
    }

    pub const fn mode(mut self, mode: &ButtonMode) -> Self {
        self.mode = *mode;
        self
    }

    fn styles(&self) -> (Style, Style, Style) {
        match self.mode {
            ButtonMode::Normal => (
                button_normal_background(),
                button_normal_highlight(),
                button_normal_shadow(),
            ),
            ButtonMode::Selected => (
                button_selected_background(),
                button_selected_highlight(),
                button_selected_shadow(),
            ),
            ButtonMode::Active => (
                button_active_background(),
                button_active_highlight(),
                button_active_shadow(),
            ),
        }
    }
}

impl<'a> Widget for ButtonWidget<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (bg_style, highlight_style, shadow_style) = self.styles();

        buf.set_style(area, bg_style);

        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                highlight_style,
            );
        }

        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                shadow_style,
            );
        }

        let label_width = self.label.chars().count() as u16;
        let label_style = match self.mode {
            ButtonMode::Normal => button_normal_background(),
            ButtonMode::Selected => button_selected_background(),
            ButtonMode::Active => button_active_background(),
        };

        let x = area.x + (area.width.saturating_sub(label_width)) / 2;
        let y = area.y + (area.height.saturating_sub(1)) / 2;
        buf.set_string(x, y, self.label, label_style);
    }
}
