#![allow(unused)]
use std::collections::VecDeque;

use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::Vector2f,
};

pub mod event;
pub mod padding;
pub mod traits;
pub mod ui_id;
pub mod style;
pub mod widget;
#[macro_use]
pub mod macros;
pub mod widgets;
pub use event::EventFromUi;

use crate::ui::{event::EventToUi, traits::UiElement, widget::WidgetData};

pub struct Ui<'a> {
    parent_size: Vector2f,
    children: Vec<Box<dyn UiElement>>,
    event_queue: VecDeque<EventFromUi>,
    widget: WidgetData<'a>,
    render_texture: FBox<RenderTexture>,
}

impl<'a> Ui<'a> {
    pub fn new(parent_size: Vector2f, children: Vec<Box<dyn UiElement>>) -> Self {
        let mut ui = Ui {
            parent_size,
            children,
            ..Default::default()
        };
        ui.render_texture = RenderTexture::new(parent_size.x as u32, parent_size.y as u32).unwrap();
        ui.init();
        ui
    }

    fn init(&mut self) {
        self.widget.init(self.parent_size, Vector2f::new(0.0, 0.0));

        for child in &mut self.children {
            child.init(self.widget.real_size, self.widget.real_position);
        }
    }

    pub fn update(&mut self) {
        self.render_texture.clear(Color::TRANSPARENT);
        for child in &mut self.children {
            child.update();
        }

        for child in &self.children {
            self.render_texture.draw(child.as_ref());
        }
        self.render_texture.display();
    }

    pub fn on_click(&mut self, click_pos: Vector2f) {
        if !self.widget.clickable || !self.widget.was_clicked(click_pos) {
            return;
        }

        //reverse order so the topmost child (drawn last) consumes the click first
        for child in self.children.iter().rev() {
            if let Some(child_events) = child.on_click(click_pos) {
                self.event_queue.extend(child_events);
                return;
            }
        }
    }

    pub fn process_incoming_event(&mut self, event: EventToUi) {
        match event {
            EventToUi::SetTexture(ui_id, texture) => {
                for child in &mut self.children {
                    if let Some(target) = child.find_mut(ui_id) {
                        target.set_background_texture(texture);
                        return;
                    }
                }
            }
        }
    }

    pub fn next_event(&mut self) -> Option<EventFromUi> {
        self.event_queue.pop_front()
    }
}

impl<'a> Default for Ui<'a> {
    fn default() -> Self {
        Self {
            event_queue: VecDeque::new(),
            parent_size: Vector2f::new(0.0, 0.0),
            render_texture: RenderTexture::new(1, 1).unwrap(),
            widget: WidgetData {
                relative_size: Vector2f::new(1.0, 1.0),
                relative_position: Vector2f::new(0.0, 0.0),
                clickable: true,
                ..Default::default()
            },

            children: Vec::new(),
        }
    }
}

impl<'b> Drawable for Ui<'b> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(Vector2f::new(0.0, 0.0));
        target.draw_with_renderstates(&sprite, states);
    }
}
