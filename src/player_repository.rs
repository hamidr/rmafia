use std::collections::BTreeMap;

use crate::id::Id;
use crate::player::*;
use crate::room::Room;

pub struct PlayerCount {
    pub mafia: usize,
    pub citizen: usize,
    pub psycho: usize,
}

impl PlayerCount {
    pub fn all(&self) -> usize {
        self.mafia + self.citizen + self.psycho
    }
}

pub trait PlayerRepository {
    fn get(&self, id: &Id) -> Option<&Player>;
    fn get_mut(&mut self, id: &Id) -> Option<&mut Player>;
    fn count(&self) -> PlayerCount;
    fn total(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Players(BTreeMap<Id, Player>);

impl Players {
    fn init(waiting: impl Room) -> Players {
        unimplemented!()
    }

    fn insert(&mut self, player: Player) -> Id {
        let id = Id::unique_random_id();
        self.0.insert(id.clone(), player);
        id
    }
}

impl PlayerRepository for Players {
    fn get(&self, id: &Id) -> Option<&Player> {
        self.0.get(id).filter(|p| p.is_conscious())
    }

    fn get_mut(&mut self, id: &Id) -> Option<&mut Player> {
        self.0.get_mut(id).filter(|p| p.is_conscious())
    }

    fn count(&self) -> PlayerCount {
        let c =
            self.0
                .values()
                .filter(|p| p.is_conscious())
                .fold((0, 0, 0), |(a, b, c), p| {
                    match (p.is_mafia(), p.is_citizen()) {
                        (true, false) => (a + 1, b, c),
                        (false, true) => (a, b + 1, c),
                        _ => (a, b, c + 1),
                    }
                });
        PlayerCount {
            mafia: c.0,
            citizen: c.1,
            psycho: c.2,
        }
    }

    fn total(&self) -> usize {
        self.0.len()
    }
}
