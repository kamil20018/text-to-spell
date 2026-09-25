use sfml::{
    graphics::{Color, FloatRect, RectangleShape, Shape, Transformable},
    system::Vector2f,
};

use crate::ui::ui_id::UiId;

pub struct WidgetData<'a> {
    pub real_size: Vector2f,
    pub real_position: Vector2f,
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub texture_position: Vector2f,
    pub id: UiId,
    pub background: RectangleShape<'a>,
    pub bg_color: Color,
    pub clickable: bool,
}

impl<'a> Default for WidgetData<'a> {
    fn default() -> Self {
        Self {
            real_size: Vector2f::new(0.0, 0.0),
            real_position: Vector2f::new(0.0, 0.0),
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            texture_position: Vector2f::new(0.0, 0.0),
            id: UiId::new_none(),
            background: RectangleShape::new(),
            bg_color: Color::TRANSPARENT,
            clickable: true,
        }
    }
}

impl<'a> WidgetData<'a> {
    pub fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.real_size = Vector2f::new(
            parent_size.x * self.relative_size.x,
            parent_size.y * self.relative_size.y,
        );
        self.real_position = Vector2f::new(
            parent_position.x + self.relative_position.x * parent_size.x,
            parent_position.y + self.relative_position.y * parent_size.y,
        );
        self.texture_position = Vector2f::new(
            parent_size.x * self.relative_position.x,
            parent_size.y * self.relative_position.y,
        );
        self.background.set_size(self.real_size);
        self.background.set_position(self.real_position);
        self.background.set_fill_color(self.bg_color);
    }

    pub fn center_text(&self, rect: FloatRect) -> Vector2f {
        Vector2f {
            x: (self.real_size.x - rect.width) / 2.0 - rect.left,
            y: (self.real_size.y - rect.height) / 2.0 - rect.top,
        }
    }

    pub fn was_clicked(&self, click_pos: Vector2f) -> bool {
        self.real_position.x < click_pos.x
            && click_pos.x < self.real_position.x + self.real_size.x
            && self.real_position.y < click_pos.y
            && click_pos.y < self.real_position.y + self.real_size.y
    }
}
