
use crate::room::*;
use crate::scenario::*;
use crate::waiting::GodWayRef;
use crate::waiting::PlayerId;

use std::collections::BTreeMap;


struct Player {
    powers: Vec<Power>,
    connection: GodWayRef
}

impl Player {
    fn new(con: GodWayRef) -> Self {
        Self {
            powers: Vec::new(),
            connection: con
        }
    }
}

pub struct InMemoryRoom {
    players: BTreeMap<PlayerId, Player>,
}

impl InMemoryRoom {
    pub fn new(cons: BTreeMap<PlayerId, GodWayRef>) -> Self {
        let mut players = BTreeMap::new();
        for (k, con) in cons.into_iter() {
            players.insert(k, Player::new(con));
        }
        
        InMemoryRoom { players }
    }

    pub fn assign(&mut self, id: &PlayerId, powers: Vec<Power>) -> bool {
        if let Some(p) = self.players.get_mut(id) {
            p.powers.extend(powers.into_iter());
            p.connection.tell(HolyMessage::Assigned(p.powers.clone()));
            true
        } else {
            false
        }
    }

    fn text_to(&mut self, id: &PlayerId, msg: HolyMessage) -> bool {
         self.players.get_mut(id)
         .map(|c| c.connection.tell(msg))
         .unwrap_or(false)
    }

    pub fn read_all(&mut self) -> BTreeMap<PlayerId, Vec<Pray>> {
        let mut res = BTreeMap::new();
        for (id, p) in self.players.iter_mut() {
            let mut vec = Vec::new();
            while let Some(r) = p.connection.read() {
                vec.push(r);
            }
            res.insert(id.clone(), vec);
        }
        res
    }
}

impl Room for InMemoryRoom {
    fn numbers(&self) -> Vec<PlayerId> {
        self.players.keys().cloned().collect()
    }
    
    fn count(&self, power: &Power) -> usize {
        self.players.values().filter(|p| p.powers.contains(power)).count()
    }

    fn remove(&mut self, id: &PlayerId) -> Vec<Power> {
        self.players.remove(id).map(|p| p.powers)
        .unwrap_or(Vec::new())
    }

    fn has(&self, id: &PlayerId, power: &Power) -> bool {
        self.players.get(id)
        .map(|p| p.powers.contains(power))
        .unwrap_or(false)
    }

    fn total(&self) -> usize {
        self.players.len()
    }

    fn kinks(&self, id: &PlayerId) -> Vec<Power> {
        self.players.get(id).map(|p| p.powers.clone()).unwrap_or(Vec::new())
    }

    fn drop_kinks<const N: usize>(&mut self, id: &PlayerId, kinks: [Power; N]) {
        if let Some(p) = self.players.get_mut(id) {
            for kink in kinks {
                match p.powers.binary_search_by(|p| p.cmp(&kink)) {
                    Ok(index) => { p.powers.remove(index); },
                    Err(_) => {},
                };
            }
            p.connection.tell(HolyMessage::Assigned(p.powers.clone()));
        }
    }

    fn messages(&mut self, id: &PlayerId) -> Vec<Pray> {
        let mut msgs = Vec::new();
        match self.players.get_mut(id) {
            Some(user) => {
                while let Some(pray) = user.connection.read() {
                    if user.powers.contains(&pray.action) {
                        msgs.push(pray);
                    }
                }
            },
            None => ()
        };
        msgs
    }

    fn by_power(&self, power: &Power) -> Vec<PlayerId> {
        self.players.iter().filter_map(|(id, p)|
            if p.powers.contains(power) {
                Some(id.clone())
            } else {
                None
            }
        ).collect()
    }
}