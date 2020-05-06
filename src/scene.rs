use crate::id::Id;
use crate::player_repository::PlayerCount;

pub enum State {
    Fighting,
    Won,
    Lost,
    WTF,
}

#[derive(Clone, Debug)]
pub enum Action {
    Kill,
    HeadShot,
    Heal,
    Slience,
    MafiaInquery,
}

pub trait Scene {
    fn wakeup(&mut self) -> State;
    fn cast_on(&mut self, action: Action, from: Id, on: Id);
    fn status(&mut self) -> Option<PlayerCount>;
}
