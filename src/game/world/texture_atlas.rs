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
        let texture = Texture::from_file("resources/textures/lava.png").expect("Failed to load texture");
        self.textures.insert("lava".to_string(), texture);
        let texture = Texture::from_file("resources/textures/stone.png").expect("Failed to load texture");
        self.textures.insert("stone".to_string(), texture);
        let texture = Texture::from_file("resources/textures/player.png").expect("Failed to load texture");
        self.textures.insert("player".to_string(), texture);
    }

    pub fn load(&mut self, name: &str, path: &str) {
        let texture = Texture::from_file(path).expect("Failed to load texture");

        self.textures.insert(name.to_string(), texture);
    }

    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name).map(|texture| &**texture)
    }
}
