
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
    powers: BTreeMap<PlayerId, Player>,
}

impl InMemoryRoom {
    pub fn new(cons: BTreeMap<PlayerId, GodWayRef>) -> Self {
        let mut players = BTreeMap::new();
        
        for (k, con) in cons.into_iter() {
            players.insert(k, Player::new(con));
        }

        InMemoryRoom {
            powers: players
        }
    }

    pub fn assign(&mut self, id: &PlayerId, powers: Vec<Power>) -> bool {
        if let Some(p) = self.powers.get_mut(id) {
            p.powers.extend(powers.into_iter());
            p.connection.tell(HolyMessage::Assigned(p.powers.clone()));
            true
        } else {
            false
        }
    }

    pub fn drop_kink(&mut self, id: &PlayerId, kink: &Power) -> bool {
        if let Some(p) = self.powers.get_mut(id) {
            let res = match p.powers.binary_search_by(|p| p.cmp(kink)) {
                Ok(index) => p.powers.remove(index) == *kink,
                Err(_) => false,
            };
            p.connection.tell(HolyMessage::Assigned(p.powers.clone()));
            return res;
        }
        false
    }

    fn text_to(&mut self, id: &PlayerId, msg: HolyMessage) -> bool {
         self.powers.get_mut(id)
         .map(|c| c.connection.tell(msg))
         .unwrap_or(false)
    }

    pub fn read_all(&mut self) -> Vec<Pray> {
        self.powers.values_mut()
        .into_iter()
        .filter_map(|p| p.connection.read())
        .collect()
    }

    pub fn by_power(&mut self, power: &Power) -> Vec<Pray> {
        self.powers.values_mut().filter_map(|p| 
            if p.powers.contains(power) { p.connection.read() } else { None }
        ).collect()
    }
}

impl Room for InMemoryRoom {
    fn numbers(&self) -> Vec<PlayerId> {
        self.powers.keys().cloned().collect()
    }
    
    fn count(&self, power: &Power) -> usize {
        self.powers.values().filter(|p| p.powers.contains(power)).count()
    }

    fn remove(&mut self, id: &PlayerId) -> Vec<Power> {
        self.powers.remove(id).map(|p| p.powers)
        .unwrap_or(Vec::new())
    }

    fn has(&self, id: &PlayerId, power: &Power) -> bool {
        self.powers.get(id)
        .map(|p| p.powers.contains(power))
        .unwrap_or(false)
    }

    fn total(&self) -> usize {
        self.powers.len()
    }

    fn kinks(&self, id: &PlayerId) -> Vec<Power> {
        self.powers.get(id).map(|p| p.powers.clone()).unwrap_or(Vec::new())
    }
}

pub struct OneToOneSpells(BTreeMap<Power, Pray>);
impl OneToOneSpells {
    pub fn new() -> OneToOneSpells {
        Self(todo!())
    }
}

impl Spells for OneToOneSpells {

    fn stop(&mut self, power: &Power) -> bool {
        todo!()
    }

    fn get(&self, power: &Power) -> Option<&NightAct> {
        todo!()
    }

    fn raw(&self, power: &Power) -> Option<(&Power, &RawSpell)> {
        todo!()
    }

    fn get_kv(&self, power: &Power) -> Option<(&Power, &NightAct)> {
        todo!()
    }

    fn expect1(&self, power: &Power) -> Option<&PlayerId> {
        todo!()
    }

    fn expect2(&self, power: &Power) -> Option<(&PlayerId, &PlayerId)> {
        todo!()
    }
}