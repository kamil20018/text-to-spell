use std::collections::HashMap;

use sfml::{cpp::FBox, graphics::Texture};

pub struct TextureAtlas {
    textures: HashMap<String, FBox<Texture>>,
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        self.load("lava", "resources/textures/lava.png");
        self.load("rock", "resources/textures/rock.png");
        self.load("player", "resources/textures/player.png");
        self.load("portal", "resources/textures/portal.png");
        self.load("wall", "resources/textures/wall.png")
    }

    pub fn load(&mut self, name: &str, path: &str) {
        let texture = Texture::from_file(path).expect("Failed to load texture");

        self.textures.insert(name.to_string(), texture);
    }

    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name).map(|texture| &**texture)
    }
}
