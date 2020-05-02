use crate::models::{Id, Role};

pub enum State {
    Playing,
    CityWon,
    CityLost,
}

#[derive(Clone, Debug)]
pub enum Action {
    Kill(Id),
    HeadShot(Id),
    Heal(Id),
    Slience(Id),
    MafiaInquery(Id),
}

pub enum Consequence {
    Deferred,
    MafiaStatus(bool),
    Rejected,
}

pub type InqueryStatus = u32;

pub trait Scene {
    fn wakeup(&self) -> State;
    fn apply(&mut self, fromId: Id, act: Action) -> Consequence;
    fn status(&mut self) -> Option<InqueryStatus>;
}
