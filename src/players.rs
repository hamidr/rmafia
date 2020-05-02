use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::models::*;

#[derive(Clone)]
pub struct Players(BTreeMap<Id, Player>);
impl Players {
    pub fn ids(&self) -> HashSet<Id> {
        self.0.keys().cloned().collect::<HashSet<Id>>()
    }

    fn append(&mut self, player: Player) -> Id {
        let id = Id::unique_random_id();
        self.0.insert(id, player);
        id
    }

    pub fn get(&self, id: Id) -> Option<&Player> {
        self.0.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut Player> {
        self.0.get_mut(&id)
    }

    pub fn init(waiting: WaitingRoom) -> Players {
        unimplemented!()
    }
}
