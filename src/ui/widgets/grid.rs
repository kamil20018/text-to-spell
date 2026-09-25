use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, Texture},
    system::{Vector2f, Vector2i},
};

use crate::ui::{
    event::{EventFromUi, GridPosition},
    padding::RelativePadding,
    traits::*,
    ui_id::UiId,
    widget::*,
};

pub struct Grid<'a> {
    //actual user given stuff
    grid_size: Vector2i,
    padding: RelativePadding,
    children: Vec<Box<dyn UiElement>>,
    //calculated / processed later
    widget: WidgetData<'a>,
}

impl<'a> Grid<'a> {
    pub fn new(
        relative_size: Vector2f,
        relative_position: Vector2f,
        id: UiId,
        grid_size: Vector2i,
        padding: RelativePadding,
        children: Vec<Box<dyn UiElement>>,
    ) -> Self {
        Self {
            grid_size,
            padding,
            children,
            widget: WidgetData {
                relative_size,
                relative_position,
                id: id,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn set_bg_color(mut self, color: Color) -> Self {
        self.widget.bg_color = color;
        self
    }
}

impl<'a> Default for Grid<'a> {
    fn default() -> Self {
        Self {
            // bg_color: Color::rgb(100, 100, 100),
            grid_size: Vector2i::new(2, 2),
            padding: RelativePadding { ..Default::default() },
            children: Vec::new(),
            widget: WidgetData {
                relative_size: Vector2f::new(0.0, 0.0),
                relative_position: Vector2f::new(0.0, 0.0),
                clickable: true,
                ..Default::default()
            },
        }
    }
}

impl<'a> UiElement for Grid<'a> {}

impl<'a> CustomUi for Grid<'a> {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.widget.init(parent_size, parent_position);

        //positioning children
        for row in 0..self.grid_size.y {
            for col in 0..self.grid_size.x {
                let idx = row * self.grid_size.x + col;
                if idx < self.children.len() as i32 {
                    let child_relative_size = Vector2f::new(
                        (1.0 - self.padding.left
                            - self.padding.right
                            - self.padding.columns * (self.grid_size.x - 1) as f32)
                            / self.grid_size.x as f32,
                        (1.0 - self.padding.top
                            - self.padding.botton
                            - self.padding.rows * (self.grid_size.y - 1) as f32)
                            / self.grid_size.y as f32,
                    );
                    let child_relative_position = Vector2f::new(
                        self.padding.left + col as f32 * (self.padding.columns + child_relative_size.x),
                        self.padding.top + row as f32 * (self.padding.rows + child_relative_size.y),
                    );
                    self.children[idx as usize].overwrite_relative(child_relative_size, child_relative_position);
                    self.children[idx as usize].init(self.widget.real_size, self.widget.real_position);
                }
            }
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>> {
        if self.widget.was_clicked(click_pos) && self.widget.clickable {
            let mut events: Vec<EventFromUi> = Vec::new();
            for (counter, child) in self.children.iter().enumerate() {
                if let Some(child_events) = child.on_click(click_pos) {
                    for event in child_events {
                        events.push(event);
                    }
                }
            }
            if events.len() > 0 {
                return Some(events);
            }
        }
        None
    }

    fn is_id(&self, id: UiId) -> bool {
        self.widget.id == id
    }

    fn contains_id(&self, _id: UiId) -> bool {
        for child in &self.children {
            if child.is_id(_id) {
                return true;
            }
        }
        return false;
    }

    fn set_background_texture(&mut self, id: UiId, texture: FBox<Texture>) {
        for child in &mut self.children {
            if child.is_id(id) || child.contains_id(id) {
                child.set_background_texture(id, texture);
                return;
            }
        }
    }
}

impl<'b> Drawable for Grid<'b> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_with_renderstates(&self.widget.background, states);
        for child in &self.children {
            target.draw_with_renderstates(child.as_ref(), states);
        }
    }
}
