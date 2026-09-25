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

    /// Dispatches a click through this element and its children.
    ///
    /// `None` means this element did not consume the click, so dispatch must
    /// continue with the remaining siblings. `Some` - even when the vector is
    /// empty - means the click was consumed and no further sibling sees it.
    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>>;

    /// Finds the element carrying `id`, searching this element and every
    /// descendant. This is the single lookup used to route events to a target.
    fn find_mut(&mut self, id: UiId) -> Option<&mut dyn UiElement>;

    /// Called on the element resolved by [`CustomUi::find_mut`]. Only widgets that
    /// own a background texture slot implement this; containers ignore it.
    fn set_background_texture(&mut self, _texture: FBox<Texture>) {}

    fn overwrite_relative(&mut self, _relative_size: Vector2f, _relative_position: Vector2f) {
        println!("overwrite_rel ignored, no implementation provided");
    }
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
