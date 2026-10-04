#![allow(unused)]
use std::collections::VecDeque;

use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::Vector2f,
    window::Key,
};

pub mod event;
pub mod padding;
pub mod style;
pub mod traits;
pub mod ui_id;
pub mod widget;
#[macro_use]
pub mod macros;
pub mod widgets;
pub use event::EventFromUi;

use crate::ui::{event::EventToUi, traits::UiElement, ui_id::UiId, widget::WidgetData};

pub struct Ui<'a> {
    parent_size: Vector2f,
    children: Vec<Box<dyn UiElement>>,
    event_queue: VecDeque<EventFromUi>,
    focused: Option<UiId>,
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

    /// Dispatches a click to this UI's children.
    ///
    /// Returns `true` when a child consumed the click, so the game can tell a
    /// click on the UI apart from one on the world behind it.
    pub fn on_click(&mut self, click_pos: Vector2f) -> bool {
        if !self.widget.clickable || !self.widget.was_clicked(click_pos) {
            return false;
        }

        self.move_focus_to(self.children.iter().rev().find_map(|c| c.focusable_at(click_pos)));

        //reverse order so the topmost child (drawn last) consumes the click first
        for child in self.children.iter().rev() {
            if let Some(child_events) = child.on_click(click_pos) {
                self.event_queue.extend(child_events);
                return true;
            }
        }
        false
    }

    /// Routes a key press to the focused element, if there is one.
    pub fn on_key_pressed(&mut self, key: Key) -> Option<Key> {
        if let Some(events) = self.focused_mut().and_then(|el| el.on_key_pressed(key)) {
            self.event_queue.extend(events);
            return None;
        }
        Some(key)
    }

    /// Whether any element currently holds keyboard focus.
    pub fn has_focus(&self) -> bool {
        self.focused.is_some()
    }

    /// Focuses the element with `new_focus`, or clears focus when it is `None`.
    /// Clicking a non-focusable widget therefore blurs whatever had focus.
    pub fn move_focus_to(&mut self, new_focus: Option<UiId>) {
        if new_focus == self.focused {
            return;
        }
        if let Some(el) = self.focused_mut() {
            el.set_focused(false);
        }
        self.focused = new_focus;
        if let Some(el) = self.focused_mut() {
            el.set_focused(true);
        }
    }

    fn focused_mut(&mut self) -> Option<&mut dyn UiElement> {
        let id = self.focused?;
        for child in &mut self.children {
            if let Some(found) = child.find_mut(id) {
                return Some(found);
            }
        }
        None
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
            focused: None,
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
