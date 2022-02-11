use hashbrown::HashMap;
use crate::level::world::World;
use crate::prefabs::Prefab;

pub mod world;

pub struct Level {
    pub name: String,
    world: World,
    players: HashMap<u16, Prefab>
}

impl Level {
    pub fn new(name: String) -> Level {
        Level {
            name,
            world: World::new(),
            players: HashMap::new()
        }
    }

    pub fn add_player(&mut self, mut player: Prefab) {
        let (address, id) = self.world.malloc();

        player.set_entity_id(id);
        player.set_entity_addr(address);
        player.write_attr(String::from("id"), id);

        self.players.insert(address, player);
    }

    pub async fn tick(&self) {}
}
