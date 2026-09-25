use sfml::{
    cpp::FBox,
    graphics::{Drawable, Texture},
    system::Vector2f,
};

use crate::ui::{event::EventFromUi, ui_id::UiId};

pub trait UiElement: CustomUi + Drawable {}

pub trait CustomUi {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f);
    fn update(&mut self);
    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>>;

    fn is_id(&self, id: UiId) -> bool;

    fn contains_id(&self, _id: UiId) -> bool {
        false
    }

    fn overwrite_relative(&mut self, _relative_size: Vector2f, _relative_position: Vector2f) {
        println!("overwrite_rel ignored, no implementation provided");
    }

    fn set_background_texture(&mut self, _id: UiId, _texture: FBox<Texture>) {}
}

//TODO:
// trait UiElement {
//     fn init(&mut self, ...);

//     fn as_container(&mut self) -> Option<&mut dyn UiContainer> {
//         None
//     }
// }

// trait UiContainer {
//     fn children(&mut self) -> &mut Vec<Box<dyn UiElement>>;
// }
