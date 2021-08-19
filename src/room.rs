use crate::{scenario::*, waiting::{PlayerId}};

pub trait Room {
    fn numbers(&self) -> Vec<PlayerId>;
    fn has(&self, id: &PlayerId, power: &Power) -> bool;
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

pub type RawSpell = (PlayerId, NightAct);
pub trait Spells {
    fn get(&self, power: &Power) -> Option<&NightAct>;
    fn raw(&self, power: &Power) -> Option<(&Power, &RawSpell)>;
    fn get_kv(&self, power: &Power) -> Option<(&Power, &NightAct)>;
    fn expect1(&self, power: &Power) -> Option<&PlayerId>;
    fn expect2(&self, power: &Power) -> Option<(&PlayerId, &PlayerId)>;
    fn stop(&mut self, power: &Power) -> bool;
}
