use std::collections::BTreeMap;

use crate::player::*;
use crate::room::Room;
use std::sync::Arc;

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub struct Id(String);

impl Id {
    pub fn unique_random_id() -> Id {
        unimplemented!()
    }
}

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

pub trait PlayerRepository: Sized + Clone {
    type K: Caster;

    fn kill_injureds(&mut self);
    fn count_alives(&self) -> PlayerCount;
    fn get(&mut self, id: &Id) -> Option<Arc<Self::K>>;
}

#[derive(Debug, Clone)]
pub struct Players<C>(BTreeMap<Id, Arc<C>>);

impl<C> Players<C> {
    fn init(waiting: impl Room) -> Players<C> {
        unimplemented!()
    }

    fn insert(&mut self, player: C) -> Id {
        let id = Id::unique_random_id();
        self.0.insert(id.clone(), Arc::new(player));
        id
    }
}

impl<C: Caster + ?Clone> PlayerRepository for Players<C> {
    type K = C;

    fn get(&mut self, id: &Id) -> Option<Arc<C>> {
        self.0.get(id).filter(|p| p.is_alive()).cloned()
    }

    fn kill_injureds(&mut self) {
        self.0
            .values_mut()
            .map(|ptr| {
                if ptr.state() == LifeState::Injured {
                    Arc::get_mut(ptr).map(|p| p.kill());
                }
            })
            .collect()
    }

    fn count_alives(&self) -> PlayerCount {
        let c = self
            .0
            .values()
            .filter(|p| p.is_alive())
            .fold((0, 0, 0), |(a, b, c), p| match p.kind() {
                RoleKind::Mafia => (a + 1, b, c),
                RoleKind::Citizen => (a, b + 1, c),
                _ => (a, b, c + 1),
            });
        PlayerCount {
            mafia: c.0,
            citizen: c.1,
            psycho: c.2,
        }
    }
}
