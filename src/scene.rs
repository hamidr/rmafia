use crate::id::Id;
use crate::player_repository::PlayerCount;

pub enum State {
    Fighting,
    Won,
    Lost,
    WTF,
}

pub trait Scene : Sized {
    fn wakeup(&mut self) -> State;
    fn cast_on(&mut self, from: Id, on: Id);
    fn status(&mut self) -> Option<PlayerCount>;
}
