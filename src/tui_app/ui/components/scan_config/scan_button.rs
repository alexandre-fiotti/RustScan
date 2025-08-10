//! Scan Button Component
//!
//! A custom scan button widget that can be themed and supports different states.

use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget, Frame};

use crate::tui_app::ui::theme::{
    button_active_background, button_active_highlight, button_active_shadow,
    button_normal_background, button_normal_highlight, button_normal_shadow,
    button_selected_background, button_selected_highlight, button_selected_shadow,
};

/// Component for managing the scan button
#[derive(Default)]
pub struct ScanButtonComponent;

impl ScanButtonComponent {
    /// Render the scan button
    pub fn render(&self, f: &mut Frame, area: Rect, state: &State) {
        let button = Button::new("Scan").state(state);
        f.render_widget(button, area);
    }
}

// TODO: Move that to widgets (https://github.com/ratatui/ratatui/blob/main/examples/apps/custom-widget/src/main.rs)
// TODO: And add theme in the struct
/// A custom button widget with label and state
#[derive(Debug, Clone)]
pub struct Button<'a> {
    label: Line<'a>,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Selected,
    Active,
}

impl Default for State {
    fn default() -> Self {
        Self::Normal
    }
}

impl<'a> Button<'a> {
    /// Create a new button with the specified label
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            state: State::Normal,
        }
    }

    /// Set the button state
    pub const fn state(mut self, state: &State) -> Self {
        self.state = *state;
        self
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (bg_style, highlight_style, shadow_style) = self.styles();

        // Set the background style for the entire area
        buf.set_style(area, bg_style);

        // Render top line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                highlight_style,
            );
        }

        // Render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                shadow_style,
            );
        }

        // Render label centered with state-specific style
        let label_width = self.label.width() as u16;
        let label_style = match self.state {
            State::Normal => button_normal_background(),
            State::Selected => button_selected_background(),
            State::Active => button_active_background(),
        };
        buf.set_string(
            area.x + (area.width.saturating_sub(label_width)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            "Scan",
            label_style,
        );
    }
}

// TODO: Make this generic and implem the specific for this button
impl Button<'_> {
    /// Get the appropriate styles based on button state, using theme functions
    fn styles(&self) -> (Style, Style, Style) {
        match self.state {
            State::Normal => (
                button_normal_background(),
                button_normal_highlight(),
                button_normal_shadow(),
            ),
            State::Selected => (
                button_selected_background(),
                button_selected_highlight(),
                button_selected_shadow(),
            ),
            State::Active => (
                button_active_background(),
                button_active_highlight(),
                button_active_shadow(),
            ),
        }
    }
}
