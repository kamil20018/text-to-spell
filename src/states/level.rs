use std::{fs::File, io::Write};

use sfml::{
    graphics::{Color, RenderTarget},
    system::{Vector2f, Vector2i},
    window::{Event, Key, mouse},
};

use crate::{
    boxed_vec,
    state_manager::{GameState, StateId, Transition},
    ui::{
        Ui,
        event::{EventFromUi, EventToUi},
        padding::RelativePadding,
        ui_id::UiId,
        widgets::{Button, Grid, TextBox},
    },
};

pub mod constant;
use constant::*;

pub mod world;
use world::{spell_parser::Object, *};

pub struct Level {
    ui: Ui<'static>,
    editor_ui: Ui<'static>,
    ui_mappings: UiMappings,
    /// Every palette button and the tool it selects.
    editor_tools: [(UiId, EditorTool); 6],
    selected_tool: EditorTool,
    editor_active: bool,
    world: World,
    transition: Option<Transition>,
}

/// What a click in the level editor does.
#[derive(Clone, Copy)]
enum EditorTool {
    /// Place the object, or remove it when one is already on the tile.
    Spawn(Object),
    /// Remove whatever object sits on the tile.
    Erase,
}

struct UiMappings {
    exit_button: UiId,
    edit_button: UiId,
    save_button: UiId,
    spell_textbox: UiId,
}

impl Level {
    pub fn new() -> Self {
        let exit_button_id = UiId::new();
        let exit_button = Button::new(Vector2f::new(0.07, 0.06), Vector2f::new(0.0, 0.0), exit_button_id)
            .set_bg_color(Color::rgb(100, 100, 100))
            .set_text("exit".to_string());

        let edit_button_id = UiId::new();
        let edit_button = Button::new(Vector2f::new(0.07, 0.06), Vector2f::new(0.0, 0.07), edit_button_id)
            .set_bg_color(Color::rgb(70, 100, 70))
            .set_text("edit".to_string());

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
        world.init(Some("resources/levels/level_3.json"));

        // Editor palette: one button per tool (placeable objects, then eraser).
        let editor_tools = [
            (UiId::new(), EditorTool::Spawn(Object::Rock)),
            (UiId::new(), EditorTool::Spawn(Object::Lava)),
            (UiId::new(), EditorTool::Spawn(Object::Wall)),
            (UiId::new(), EditorTool::Spawn(Object::Portal)),
            (UiId::new(), EditorTool::Spawn(Object::Player)),
            (UiId::new(), EditorTool::Erase),
        ];
        let palette = Grid::new(
            Vector2f::new(0.07, 0.2),
            Vector2f::new(0.02, 0.18),
            UiId::new(),
            Vector2i::new(2, 3),
            RelativePadding {
                top: 0.1,
                botton: 0.1,
                left: 0.02,
                right: 0.02,
                columns: 0.03,
                rows: 0.0,
            },
            boxed_vec![
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[0].0)
                    .set_bg_color(Color::rgb(40, 40, 40)),
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[1].0)
                    .set_bg_color(Color::rgb(40, 40, 40)),
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[2].0)
                    .set_bg_color(Color::rgb(40, 40, 40)),
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[3].0)
                    .set_bg_color(Color::rgb(40, 40, 40)),
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[4].0)
                    .set_bg_color(Color::rgb(40, 40, 40)),
                Button::new(Vector2f::new(1.0, 1.0), Vector2f::new(0.0, 0.0), editor_tools[5].0)
                    .set_bg_color(Color::rgb(110, 60, 60))
                    .set_text("erase".to_string()),
            ],
        );

        let mut editor_ui = Ui::new(Vector2f::new(SCREEN_W as f32, SCREEN_H as f32), boxed_vec![palette]);

        // Give each object tool its texture from the world atlas. The eraser
        // has no texture, which is why it is drawn as a labelled button.
        for &(id, tool) in &editor_tools {
            if let EditorTool::Spawn(object) = tool {
                let texture = world
                    .texture(object.texture_name())
                    .expect("editor palette texture must be loaded");
                editor_ui.process_incoming_event(EventToUi::SetTexture(id, texture));
            }
        }

        Level {
            ui: Ui::new(
                Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                boxed_vec![exit_button, edit_button, save_button, spell_textbox],
            ),
            editor_ui,
            ui_mappings: UiMappings {
                exit_button: exit_button_id,
                edit_button: edit_button_id,
                save_button: save_button_id,
                spell_textbox: spell_textbox_id,
            },
            editor_tools,
            selected_tool: EditorTool::Spawn(Object::Rock),
            editor_active: false,
            world,
            transition: None,
        }
    }

    fn process_ui_event(&mut self, event: &EventFromUi) {
        match event {
            EventFromUi::ButtonClicked(button_id) => {
                if *button_id == self.ui_mappings.exit_button {
                    self.transition = Some(Transition::Quit);
                } else if *button_id == self.ui_mappings.edit_button {
                    self.editor_active = !self.editor_active;
                } else if *button_id == self.ui_mappings.save_button {
                    let json = world::serialization::world_to_json(&self.world);
                    println!("{json}");
                    let mut file = File::create("resources/levels/test.json").expect("Could not create file!");

                    file.write_all(json.as_bytes()).expect("Cannot write to the file!");
                }
            }
            EventFromUi::TextSubmitted(textbox_id, text) => {
                if *textbox_id == self.ui_mappings.spell_textbox {
                    self.world.cast_spell(text);
                }
            }
        }
    }

    /// Handles a click on the editor palette by selecting its tool.
    fn process_editor_event(&mut self, event: &EventFromUi) {
        if let EventFromUi::ButtonClicked(button_id) = event {
            for &(id, tool) in &self.editor_tools {
                if id == *button_id {
                    self.selected_tool = tool;
                }
            }
        }
    }

    /// Applies the selected tool to the tile under `screen_pos`.
    fn edit_world_at(&mut self, screen_pos: Vector2f) {
        let tile = self.world.screen_to_tile(screen_pos);
        match self.selected_tool {
            EditorTool::Spawn(object) => {
                self.world.toggle_object_at(&object, tile);
            }
            EditorTool::Erase => {
                self.world.despawn_at(tile);
            }
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for Level {
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
                        Key::E => self.editor_active = !self.editor_active,
                        _ => self.world.process_keystroke(key),
                    }
                }
            }
            Event::MouseButtonPressed { button, x, y } => {
                if *button == mouse::Button::Left {
                    let click_pos = Vector2f::new(*x as f32, *y as f32);
                    let consumed = self.ui.on_click(click_pos);
                    if self.editor_active {
                        let consumed = consumed || self.editor_ui.on_click(click_pos);
                        if !consumed {
                            self.edit_world_at(click_pos);
                        }
                    }
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

        if self.editor_active {
            while let Some(event) = self.editor_ui.next_event() {
                self.process_editor_event(&event);
            }
            self.editor_ui.update();
        }

        self.world.update();
    }

    fn draw(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.world);
        target.draw(&self.ui);
        if self.editor_active {
            target.draw(&self.editor_ui);
        }
    }

    fn transition(&mut self) -> Option<Transition> {
        self.transition.take()
    }
}
