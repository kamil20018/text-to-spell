use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, Font, RenderStates, RenderTarget, Sprite, Text, Texture, Transformable},
    system::Vector2f,
};

use crate::ui::{event::EventFromUi, traits::*, ui_id::UiId, widget::*};

pub struct Button<'a> {
    //actual user given stuff
    texture: Option<FBox<Texture>>,
    text: Option<String>,
    //calculated / processed later
    widget: WidgetData<'a>,
}

impl<'a> Button<'a> {
    pub fn new(relative_size: Vector2f, relative_position: Vector2f, id: UiId) -> Self {
        Self {
            widget: WidgetData {
                relative_size,
                relative_position,
                id: id,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn new_dynamic(id: UiId) -> Self {
        Self {
            widget: WidgetData {
                id,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn set_bg_color(mut self, color: Color) -> Self {
        self.widget.bg_color = color;
        self
    }
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Self {
            texture: None,
            text: None,
            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },
        }
    }
}

impl<'a> UiElement for Button<'a> {}

impl<'a> CustomUi for Button<'a> {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.widget.init(parent_size, parent_position);
    }

    fn update(&mut self) {}

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>> {
        if self.widget.was_clicked(click_pos) {
            return Some(vec![EventFromUi::ButtonClicked(self.widget.id)]);
        }
        None
    }

    fn is_id(&self, id: UiId) -> bool {
        id == self.widget.id
    }

    fn set_background_texture(&mut self, _id: UiId, texture: FBox<Texture>) {
        self.texture = Some(texture);
    }

    fn overwrite_relative(&mut self, relative_size: Vector2f, relative_position: Vector2f) {
        self.widget.relative_size = relative_size;
        self.widget.relative_position = relative_position;
    }
}

impl<'b> Drawable for Button<'b> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(&self.widget.background, states);

        if let Some(texture) = &self.texture {
            let tex_size = texture.size();
            let mut sprite = Sprite::with_texture(texture);
            sprite.scale((
                self.widget.real_size.x / tex_size.x as f32,
                self.widget.real_size.y / tex_size.y as f32,
            ));
            sprite.set_position(self.widget.real_position);
            target.draw_with_renderstates(&sprite, states);
        }

        if let Some(button_text) = &self.text {
            let font = Font::from_file("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf").unwrap();
            let mut text = Text::new(button_text, &font, 10);
            text.set_position(self.widget.center_text(text.local_bounds()) + self.widget.real_position);
            target.draw_with_renderstates(&text, states);
        }
    }
}
