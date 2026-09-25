use sfml::{cpp::FBox, graphics::Texture, system::Vector2i};

use crate::ui::ui_id::UiId;

pub enum EventFromUi {
    ButtonClicked(UiId),
}

#[derive(Debug)]
pub struct GridPosition(pub Vector2i);

pub enum EventToUi {
    SetTexture(UiId, FBox<Texture>),
}
