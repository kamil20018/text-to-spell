use std::collections::HashSet;

use sfml::{
    cpp::FBox,
    graphics::{Color, RenderTarget, RenderWindow},
    system::{Vector2f, Vector2i},
    window::{self, ContextSettings, Event, Key, VideoMode, mouse},
};

use crate::boxed_vec;
use crate::ui::{
    // macros,
    Ui,
    event::EventFromUi,
    traits::UiElement,
    ui_id::UiId,
    widgets::{Button, TextBox},
};

pub mod constant;
use constant::*;

pub mod world;
use world::*;

pub struct Game<'a> {
    window: FBox<RenderWindow>,
    ui: Ui<'a>,
    ui_mappings: UiMappings,
    world: World,
    // ui_state: UiState,
}

pub struct UiMappings {
    exit_button: UiId,
    spell_textbox: UiId,
}

impl UiMappings {
    fn button_press(&self, id: UiId) -> Option<UiAction> {
        if id == self.exit_button {
            return Some(UiAction::ExitGame);
        }
        None
    }
}

enum UiAction {
    ExitGame,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let mut window: FBox<RenderWindow> = RenderWindow::new(
            VideoMode::new(SCREEN_W, SCREEN_H, 32),
            "Spellcaster Supreme",
            window::Style::CLOSE,
            &ContextSettings::default(),
        )
        .expect("Cannot create a new Render Window.");
        window.set_framerate_limit(60);
        window.set_position(Vector2i::new(270, 190));

        let exit_button_id = UiId::new();
        let exit_button = Button::new(Vector2f::new(0.07, 0.06), Vector2f::new(0.0, 0.0), exit_button_id)
            .set_bg_color(Color::rgb(100, 100, 100));

        let mut spell_component_grid_mappings = HashSet::new();
        let mut grid_buttons: Vec<Box<dyn UiElement>> = Vec::new();
        for _row in 0..11 {
            for _col in 0..11 {
                let id = UiId::new();
                spell_component_grid_mappings.insert(id);
                grid_buttons.push(Box::new(Button::new_dynamic(id).set_bg_color(Color::WHITE)));
            }
        }

        let spell_textbox_id = UiId::new();
        let spell_textbox = TextBox::new(Vector2f::new(0.46, 0.08), Vector2f::new(0.27, 0.9), spell_textbox_id)
            .set_bg_color(Color::rgb(30, 30, 60))
            .set_text_color(Color::WHITE)
            .set_character_size(28);

        Game {
            window: window,
            ui: Ui::new(
                Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                boxed_vec![
                    // exit button
                    exit_button,
                    // spell textbox
                    spell_textbox,
                ],
            ),
            ui_mappings: UiMappings {
                exit_button: exit_button_id,
                spell_textbox: spell_textbox_id,
            },
            world: World::new(),
        }
    }

    pub fn run(&mut self) {
        self.init();
        while self.window.is_open() {
            self.process_input();
            self.update();
            self.draw();
        }
    }

    fn init(&mut self) {
        self.world.init();
    }

    pub fn process_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => self.window.close(),
                    key => {
                        if let Some(key) = self.ui.on_key_pressed(key) {
                            //process keystroke if not consumed by ui
                            match key {
                                Key::T => self.ui.move_focus_to(Some(self.ui_mappings.spell_textbox)),
                                _ => self.world.process_keystroke(key),
                            }
                        }
                    }
                },
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::Left => self.ui.on_click(Vector2f::new(x as f32, y as f32)),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        while let Some(event) = self.ui.next_event() {
            self.process_ui_event(&event);
        }
        self.ui.update();
        self.world.update();
    }

    fn process_ui_event(&mut self, event: &EventFromUi) {
        match event {
            EventFromUi::ButtonClicked(button_id) => {
                if let Some(ui_action) = self.ui_mappings.button_press(*button_id) {
                    match ui_action {
                        UiAction::ExitGame => self.window.close(),
                        // _ => {}
                    }
                }
            }
            EventFromUi::TextSubmitted(textbox_id, text) => {
                if *textbox_id == self.ui_mappings.spell_textbox {
                    self.world.cast_spell(text);
                }
            }
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(2, 9, 46));
        self.window.draw(&self.world);
        self.window.draw(&self.ui);
        self.window.display();
    }
}
