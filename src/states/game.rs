use sfml::{
    graphics::{Color, RenderTarget},
    system::Vector2f,
    window::{Event, Key, mouse},
};

use crate::{
    boxed_vec,
    state_manager::{GameState, StateId, Transition},
    ui::{
        Ui,
        event::EventFromUi,
        ui_id::UiId,
        widgets::{Button, TextBox},
    },
};

pub mod constant;
use constant::*;

pub mod world;
use world::*;

/// The playable screen: the world plus the in-game UI (a spell textbox and an
/// exit button).
pub struct Game {
    ui: Ui<'static>,
    ui_mappings: UiMappings,
    world: World,
    transition: Option<Transition>,
}

struct UiMappings {
    exit_button: UiId,
    save_button: UiId,
    spell_textbox: UiId,
}

impl Game {
    pub fn new() -> Self {
        let exit_button_id = UiId::new();
        let exit_button = Button::new(Vector2f::new(0.07, 0.06), Vector2f::new(0.0, 0.0), exit_button_id)
            .set_bg_color(Color::rgb(100, 100, 100))
            .set_text("exit".to_string());

        let save_button_id = UiId::new();
        let save_button = Button::new(Vector2f::new(0.07, 0.06), Vector2f::new(0.93, 0.0), save_button_id)
            .set_bg_color(Color::rgb(100, 100, 100))
            .set_text("save".to_string());

        let spell_textbox_id = UiId::new();
        let spell_textbox = TextBox::new(Vector2f::new(0.46, 0.08), Vector2f::new(0.27, 0.9), spell_textbox_id)
            .set_bg_color(Color::rgb(30, 30, 60))
            .set_text_color(Color::WHITE)
            .set_character_size(28);

        let mut world = World::new();
        world.init();

        Game {
            ui: Ui::new(
                Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                boxed_vec![exit_button, save_button, spell_textbox,],
            ),
            ui_mappings: UiMappings {
                exit_button: exit_button_id,
                save_button: save_button_id,
                spell_textbox: spell_textbox_id,
            },
            world,
            transition: None,
        }
    }

    fn process_ui_event(&mut self, event: &EventFromUi) {
        match event {
            EventFromUi::ButtonClicked(button_id) => {
                if *button_id == self.ui_mappings.exit_button {
                    self.transition = Some(Transition::Quit);
                } else if *button_id == self.ui_mappings.save_button {
                    println!("save");
                }
            }
            EventFromUi::TextSubmitted(textbox_id, text) => {
                if *textbox_id == self.ui_mappings.spell_textbox {
                    self.world.cast_spell(text);
                }
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for Game {
    fn process_input(&mut self, event: &Event) {
        match event {
            Event::KeyPressed { code: Key::Escape, .. } => {
                // Esc first blurs whatever UI element is focused; only when
                // nothing has focus does it leave the game.
                if self.ui.has_focus() {
                    self.ui.move_focus_to(None);
                } else {
                    self.transition = Some(Transition::SwitchTo(StateId::MainMenu));
                }
            }
            Event::KeyPressed { code, .. } => {
                if let Some(key) = self.ui.on_key_pressed(*code) {
                    //process keystroke if not consumed by ui
                    match key {
                        Key::T => self.ui.move_focus_to(Some(self.ui_mappings.spell_textbox)),
                        _ => self.world.process_keystroke(key),
                    }
                }
            }
            Event::MouseButtonPressed { button, x, y } => {
                if *button == mouse::Button::Left {
                    self.ui.on_click(Vector2f::new(*x as f32, *y as f32));
                }
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        while let Some(event) = self.ui.next_event() {
            self.process_ui_event(&event);
        }
        self.ui.update();
        self.world.update();
    }

    fn draw(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.world);
        target.draw(&self.ui);
    }

    fn transition(&mut self) -> Option<Transition> {
        self.transition.take()
    }
}
