use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Vector2f, Vector2i},
    window::Key,
};

use crate::game::constant;

/// Grid line thickness in pixels.
const GRID_LINE_THICKNESS: f32 = 1.0;
pub struct World {
    render_texture: FBox<RenderTexture>,
}

impl World {
    pub fn new() -> Self {
        World {
            render_texture: RenderTexture::new(constant::SCREEN_W, constant::SCREEN_H).unwrap(),
        }
    }

    pub fn update(&mut self) {
        self.draw();
    }

    fn draw(&mut self) {
        self.render_texture.clear(Color::TRANSPARENT);
        self.draw_grid_lines(Vector2i::new(32, 18));
        self.render_texture.display();
    }

    fn draw_grid_lines(&mut self, grid_size: Vector2i) {
        let size = self.render_texture.size();
        let width = size.x as f32;
        let height = size.y as f32;
        let cell_width = width / grid_size.x as f32;
        let cell_height = height / grid_size.y as f32;

        let target = &mut *self.render_texture;

        let mut column = RectangleShape::with_size(Vector2f::new(GRID_LINE_THICKNESS, height));
        column.set_fill_color(Color::WHITE);
        for col in 1..grid_size.x {
            column.set_position(Vector2f::new(col as f32 * cell_width, 0.0));
            target.draw(&column);
        }

        let mut row = RectangleShape::with_size(Vector2f::new(width, GRID_LINE_THICKNESS));
        row.set_fill_color(Color::WHITE);
        for row_index in 1..grid_size.y {
            row.set_position(Vector2f::new(0.0, row_index as f32 * cell_height));
            target.draw(&row);
        }
    }
}

impl Drawable for World {
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
