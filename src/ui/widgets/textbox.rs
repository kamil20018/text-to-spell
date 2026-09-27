use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text, Transformable},
    system::Vector2f,
    window::Key,
};

use crate::ui::{event::EventFromUi, traits::*, ui_id::UiId, widget::*};

pub const DEFAULT_FONT_PATH: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf";

/// Caret width, in pixels.
const CARET_WIDTH: f32 = 2.0;
/// Gap between the end of the text and the caret, in pixels.
const CARET_GAP: f32 = 3.0;

pub struct TextBox<'a> {
    //actual user given stuff
    text: String,
    font: FBox<Font>,
    character_size: u32,
    text_color: Color,
    padding: f32,
    //calculated / processed later
    cursor: usize,
    focused: bool,
    widget: WidgetData<'a>,
}

impl<'a> TextBox<'a> {
    pub fn new(relative_size: Vector2f, relative_position: Vector2f, id: UiId) -> Self {
        Self {
            text: String::new(),
            font: load_default_font(),
            character_size: 24,
            text_color: Color::WHITE,
            padding: 6.0,
            cursor: 0,
            focused: false,
            widget: WidgetData {
                relative_size,
                relative_position,
                id,
                clickable: true,
                ..Default::default()
            },
        }
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.cursor = text.chars().count();
        self.text = text;
        self
    }

    pub fn set_font(mut self, font: FBox<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn set_character_size(mut self, character_size: u32) -> Self {
        self.character_size = character_size;
        self
    }

    pub fn set_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Horizontal and vertical inset between the border and the text, in pixels.
    pub fn set_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_bg_color(mut self, color: Color) -> Self {
        self.widget.bg_color = color;
        self
    }

    /// The current contents, without the trailing caret.
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    fn insert_char(&mut self, c: char) {
        self.text.insert(byte_index(&self.text, self.cursor), c);
        self.cursor += 1;
    }

    fn remove_char_at(&mut self, char_idx: usize) {
        let start = byte_index(&self.text, char_idx);
        if let Some(len) = self.text[start..].chars().next().map(char::len_utf8) {
            self.text.replace_range(start..start + len, "");
        }
    }
}

impl<'a> UiElement for TextBox<'a> {}

impl<'a> CustomUi for TextBox<'a> {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.widget.init(parent_size, parent_position);
    }

    fn update(&mut self) {}

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>> {
        if self.widget.clickable && self.widget.was_clicked(click_pos) {
            // Swallow the click so it cannot reach a widget underneath, but emit
            // nothing: gaining focus is handled by Ui::on_click.
            return Some(Vec::new());
        }
        None
    }

    fn on_key_pressed(&mut self, key: Key) -> Option<Vec<EventFromUi>> {
        match key {
            Key::Enter => {
                let submitted = EventFromUi::TextSubmitted(self.widget.id, std::mem::take(&mut self.text));
                self.cursor = 0;
                Some(vec![submitted])
            }
            Key::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.remove_char_at(self.cursor);
                }
                Some(Vec::new())
            }
            Key::Delete => {
                if self.cursor < self.text.chars().count() {
                    self.remove_char_at(self.cursor);
                }
                Some(Vec::new())
            }
            Key::Left => {
                self.cursor = self.cursor.saturating_sub(1);
                Some(Vec::new())
            }
            Key::Right => {
                self.cursor = (self.cursor + 1).min(self.text.chars().count());
                Some(Vec::new())
            }
            Key::Home => {
                self.cursor = 0;
                Some(Vec::new())
            }
            Key::End => {
                self.cursor = self.text.chars().count();
                Some(Vec::new())
            }
            other => {
                let c = key_to_char(other)?;
                self.insert_char(c);
                Some(Vec::new())
            }
        }
    }

    fn focusable_at(&self, pos: Vector2f) -> Option<UiId> {
        if self.widget.clickable && self.widget.was_clicked(pos) {
            return Some(self.widget.id);
        }
        None
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        if focused {
            self.cursor = self.text.chars().count();
        }
    }

    fn find_mut(&mut self, id: UiId) -> Option<&mut dyn UiElement> {
        if self.widget.id == id {
            return Some(self);
        }
        None
    }

    fn overwrite_relative(&mut self, relative_size: Vector2f, relative_position: Vector2f) {
        self.widget.relative_size = relative_size;
        self.widget.relative_position = relative_position;
    }
}

impl<'b> Drawable for TextBox<'b> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(&self.widget.background, states);

        let mut text = Text::new(self.text.as_str(), &self.font, self.character_size);
        text.set_fill_color(self.text_color);

        // Anchor the text on its line box rather than on the glyph ink bounds.
        // The ink height and top offset change with every character typed, so
        // centring the ink makes the text jump around. SFML always puts the
        // baseline exactly `character_size` below the origin, and line_spacing
        // gives the real line box height for this font and size, so both are
        // stable regardless of the contents.
        let line_height = self.font.line_spacing(self.character_size);
        let bounds = text.local_bounds();
        let origin = Vector2f::new(
            self.widget.real_position.x + self.padding - bounds.left,
            self.widget.real_position.y + (self.widget.real_size.y - line_height) / 2.0,
        );
        text.set_position(origin);

        if self.focused {
            // The caret spans a nominal line box and is centred in the widget,
            // rather than tracking the glyph ink bounds: that way it brackets
            // the text instead of starting flush with the tallest letter, and it
            // stays visible while the box is still empty.
            let caret_height = self.character_size as f32;
            let mut caret = RectangleShape::with_size(Vector2f::new(CARET_WIDTH, caret_height));
            caret.set_fill_color(self.text_color);
            caret.set_position(Vector2f::new(
                origin.x + bounds.width + CARET_GAP,
                self.widget.real_position.y + (self.widget.real_size.y - caret_height) / 2.0,
            ));
            target.draw_with_renderstates(&caret, states);
        }

        target.draw_with_renderstates(&text, states);
    }
}

fn load_default_font() -> FBox<Font> {
    Font::from_file(DEFAULT_FONT_PATH).unwrap_or_else(|_| panic!("could not load a font from {DEFAULT_FONT_PATH}"))
}

/// Byte offset of the given character index, clamped to the end of the string.
fn byte_index(s: &str, char_idx: usize) -> usize {
    s.char_indices().nth(char_idx).map_or(s.len(), |(i, _)| i)
}

/// Maps a key to the character it produces. SFML reports physical keys, so this
/// is a plain US/ASCII mapping: it ignores shift, caps lock and other layouts.
fn key_to_char(key: Key) -> Option<char> {
    Some(match key {
        Key::Space => ' ',
        Key::A => 'a',
        Key::B => 'b',
        Key::C => 'c',
        Key::D => 'd',
        Key::E => 'e',
        Key::F => 'f',
        Key::G => 'g',
        Key::H => 'h',
        Key::I => 'i',
        Key::J => 'j',
        Key::K => 'k',
        Key::L => 'l',
        Key::M => 'm',
        Key::N => 'n',
        Key::O => 'o',
        Key::P => 'p',
        Key::Q => 'q',
        Key::R => 'r',
        Key::S => 's',
        Key::T => 't',
        Key::U => 'u',
        Key::V => 'v',
        Key::W => 'w',
        Key::X => 'x',
        Key::Y => 'y',
        Key::Z => 'z',
        Key::Num0 => '0',
        Key::Num1 => '1',
        Key::Num2 => '2',
        Key::Num3 => '3',
        Key::Num4 => '4',
        Key::Num5 => '5',
        Key::Num6 => '6',
        Key::Num7 => '7',
        Key::Num8 => '8',
        Key::Num9 => '9',
        Key::Semicolon => ';',
        Key::Comma => ',',
        Key::Period => '.',
        Key::Slash => '/',
        Key::Backslash => '\\',
        Key::Quote => '\'',
        Key::LBracket => '[',
        Key::RBracket => ']',
        Key::Hyphen => '-',
        Key::Equal => '=',
        Key::Tilde => '`',
        _ => return None,
    })
}
