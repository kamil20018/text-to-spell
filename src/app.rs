use sfml::{
    cpp::FBox,
    graphics::{RenderTarget, RenderWindow},
    system::Vector2i,
    window::{self, ContextSettings, Event, VideoMode},
};

use crate::{
    state_manager::{StateId, StateManager},
    states::game::constant::{SCREEN_H, SCREEN_W},
    ui::style::BACKGROUND_DARK_BLUE,
};

/// Owns the window and the main loop, and hands each frame to the
/// [`StateManager`].
pub struct App {
    window: FBox<RenderWindow>,
    state_manager: StateManager,
}

impl App {
    pub fn new() -> Self {
        let mut window: FBox<RenderWindow> = RenderWindow::new(
            VideoMode::new(SCREEN_W, SCREEN_H, 32),
            "Text To Spell",
            window::Style::CLOSE,
            &ContextSettings::default(),
        )
        .expect("Cannot create a new Render Window.");
        window.set_framerate_limit(60);
        window.set_position(Vector2i::new(270, 190));

        App {
            window,
            state_manager: StateManager::new(StateId::MainMenu),
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.process_input();
            self.update();
            self.draw();
        }
    }

    fn process_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                event => self.state_manager.process_input(&event),
            }
        }
    }

    fn update(&mut self) {
        self.state_manager.update();
        if self.state_manager.apply_pending_transition() {
            self.window.close();
        }
    }

    fn draw(&mut self) {
        self.window.clear(BACKGROUND_DARK_BLUE);
        self.state_manager.draw(&mut *self.window);
        self.window.display();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
