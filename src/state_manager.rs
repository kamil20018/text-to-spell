use sfml::{graphics::RenderTarget, window::Event};

use crate::states::{game::Game, main_menu::MainMenu};

/// Identifies one of the screens the game can show.
///
/// A state is built from this id whenever the [`StateManager`] switches to it,
/// so states never need to know about each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateId {
    MainMenu,
    Game,
}

/// A request from a state to leave itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    /// Replace the current state with a fresh instance of this one.
    SwitchTo(StateId),
    /// Close the application.
    Quit,
}

/// A self-contained screen of the game.
///
/// The [`StateManager`] owns exactly one `GameState` at a time and drives it by
/// calling [`process_input`](GameState::process_input), [`update`](GameState::update)
/// and [`draw`](GameState::draw) each frame.
pub trait GameState {
    /// Handles one window event.
    fn process_input(&mut self, event: &Event);

    /// Advances the state by one frame.
    fn update(&mut self);

    /// Draws the state into `target`.
    fn draw(&self, target: &mut dyn RenderTarget);

    /// Returns the transition the state wants to make, if any. The manager
    /// polls this once per frame and resets the request afterwards.
    fn transition(&mut self) -> Option<Transition> {
        None
    }
}

/// Owns the active [`GameState`] and forwards the frame lifecycle to it.
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
            StateId::Game => Box::new(Game::new()),
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

    /// Applies the transition requested by the current state, if any.
    ///
    /// Returns `true` when the application should quit. A switch builds the new
    /// state and drops the old one, so states do not have to clean up manually.
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
