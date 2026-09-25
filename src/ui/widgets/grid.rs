use sfml::{
    graphics::{Color, Drawable, RenderStates, RenderTarget},
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

        let cols = self.grid_size.x as usize;
        let rows = self.grid_size.y as usize;
        assert_eq!(
            cols * rows,
            self.children.len(),
            "grid cell count must match the number of children"
        );
        if cols == 0 || rows == 0 {
            return;
        }

        //invariant across every cell
        let child_relative_size = Vector2f::new(
            (1.0 - self.padding.left - self.padding.right - self.padding.columns * (cols - 1) as f32) / cols as f32,
            (1.0 - self.padding.top - self.padding.botton - self.padding.rows * (rows - 1) as f32) / rows as f32,
        );

        for (idx, child) in self.children.iter_mut().enumerate() {
            let col = idx % cols;
            let row = idx / cols;
            let child_relative_position = Vector2f::new(
                self.padding.left + col as f32 * (self.padding.columns + child_relative_size.x),
                self.padding.top + row as f32 * (self.padding.rows + child_relative_size.y),
            );
            child.overwrite_relative(child_relative_size, child_relative_position);
            child.init(self.widget.real_size, self.widget.real_position);
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>> {
        if !self.widget.clickable || !self.widget.was_clicked(click_pos) {
            return None;
        }

        //reverse order so the topmost child (drawn last) consumes the click first
        for child in self.children.iter().rev() {
            if let Some(child_events) = child.on_click(click_pos) {
                return Some(child_events);
            }
        }
        None
    }

    fn find_mut(&mut self, id: UiId) -> Option<&mut dyn UiElement> {
        if self.widget.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_mut(id) {
                return Some(found);
            }
        }
        None
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
