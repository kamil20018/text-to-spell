use sfml::{
    graphics::{Color, RenderTarget},
    system::{Vector2f, Vector2i},
    window::{Event, Key, mouse},
};

use crate::{
    boxed_vec,
    state_manager::{GameState, StateId, Transition},
    states::game::constant::{SCREEN_H, SCREEN_W},
    ui::{
        Ui,
        event::EventFromUi,
        padding::RelativePadding,
        ui_id::UiId,
        widgets::{Button, Grid},
    },
};

/// The title screen: a vertical stack with "Play" and "Quit".
pub struct MainMenu {
    ui: Ui<'static>,
    play_button: UiId,
    quit_button: UiId,
    transition: Option<Transition>,
}

impl MainMenu {
    pub fn new() -> Self {
        let play_button = UiId::new();
        let quit_button = UiId::new();

        let play = Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), play_button)
            .set_bg_color(Color::rgb(60, 60, 110))
            .set_text("Play".to_string());
        let quit = Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), quit_button)
            .set_bg_color(Color::rgb(110, 60, 60))
            .set_text("Quit".to_string());

        let menu = Grid::new(
            Vector2f::new(0.3, 0.4),
            Vector2f::new(0.35, 0.3),
            UiId::new(),
            Vector2i::new(1, 2),
            RelativePadding {
                top: 0.1,
                botton: 0.1,
                left: 0.1,
                right: 0.1,
                columns: 0.0,
                rows: 0.2,
            },
            boxed_vec![play, quit],
        );

        MainMenu {
            ui: Ui::new(Vector2f::new(SCREEN_W as f32, SCREEN_H as f32), boxed_vec![menu]),
            play_button,
            quit_button,
            transition: None,
        }
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for MainMenu {
    fn process_input(&mut self, event: &Event) {
        match event {
            Event::KeyPressed { code: Key::Escape, .. } => self.transition = Some(Transition::Quit),
            Event::KeyPressed { code, .. } => {
                self.ui.on_key_pressed(*code);
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
            match event {
                EventFromUi::ButtonClicked(id) if id == self.play_button => {
                    self.transition = Some(Transition::SwitchTo(StateId::Game));
                }
                EventFromUi::ButtonClicked(id) if id == self.quit_button => {
                    self.transition = Some(Transition::Quit);
                }
                _ => {}
            }
        }
        self.ui.update();
    }

    fn draw(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.ui);
    }

    fn transition(&mut self) -> Option<Transition> {
        self.transition.take()
    }
}
