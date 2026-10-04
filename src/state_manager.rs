use sfml::{graphics::RenderTarget, window::Event};

use crate::states::{level::Level, main_menu::MainMenu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateId {
    MainMenu,
    Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    SwitchTo(StateId),
    Quit,
}

pub trait GameState {
    fn process_input(&mut self, event: &Event);

    fn update(&mut self);

    fn draw(&self, target: &mut dyn RenderTarget);

    fn transition(&mut self) -> Option<Transition> {
        None
    }
}

pub struct StateManager {
    current: Box<dyn GameState>,
}

impl StateManager {
    pub fn new(initial: StateId) -> Self {
        Self {
            current: Self::build(initial),
        }
    }

    fn build(id: StateId) -> Box<dyn GameState> {
        match id {
            StateId::MainMenu => Box::new(MainMenu::new()),
            StateId::Game => Box::new(Level::new()),
        }
    }

    pub fn process_input(&mut self, event: &Event) {
        self.current.process_input(event);
    }

    pub fn update(&mut self) {
        self.current.update();
    }

    pub fn draw(&self, target: &mut dyn RenderTarget) {
        self.current.draw(target);
    }

    pub fn apply_pending_transition(&mut self) -> bool {
        match self.current.transition() {
            Some(Transition::SwitchTo(id)) => {
                self.current = Self::build(id);
                false
            }
            Some(Transition::Quit) => true,
            None => false,
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new(StateId::MainMenu)
    }
}
