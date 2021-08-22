use std::vec;

use crate::{scenario::*, waiting::{PlayerId}};

pub trait Room {
    fn numbers(&self) -> Vec<PlayerId>;
    fn has(&self, id: &PlayerId, power: &Power) -> bool;
    fn drop_kink(&mut self, id: &PlayerId, power: &Power) -> bool;
    fn kinks(&self, id: &PlayerId) -> Vec<Power>;
    fn total(&self) -> usize;
    fn count(&self, power: &Power) -> usize;
    fn remove(&mut self, id: &PlayerId) -> Vec<Power>;
}

#[derive(Clone)]
pub enum NightAct {
    One(PlayerId),
    Two(PlayerId, PlayerId),
    Wicked(PlayerId, Power)
}

impl NightAct {
    pub fn ids(&self) -> Vec<PlayerId> {
        match *self {
            Self::One(p1) => vec![p1],
            Self::Two(p1, p2) => vec![p1, p2],
            _ => vec![]
        }
    }
}

pub type RawSpell = (PlayerId, Power, NightAct);
pub trait Spells {
    fn get(&self, power: &Power) -> Option<&NightAct>;
    fn raw(&self, power: &Power) -> Option<&RawSpell>;
    fn raw_vec(&self, power: &Power) -> Option<Vec<&RawSpell>>;
    fn one(&self, power: &Power) -> Option<(PlayerId, Power, PlayerId)>;
    fn two(&self, power: &Power) -> Option<(PlayerId, Power, (PlayerId, PlayerId))>;
    fn all(&self, power: &Power) -> Option<(PlayerId, Power, Vec<PlayerId>)>;
    fn get_kv(&self, power: &Power) -> Option<(&Power, &NightAct)>;
    fn expect1(&self, power: &Power) -> Option<&PlayerId>;
    fn expect2(&self, power: &Power) -> Option<(&PlayerId, &PlayerId)>;
    fn stop(&mut self, power: &Power) -> bool;
}
