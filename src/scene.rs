use crate::player_repository::{Id, PlayerCount};

pub enum Status {
    Fighting,
    Won,
    Lost,
    WTF,
}

pub trait Scene: Sized + Clone {
    fn wakeup(&mut self) -> Status;
    fn cast_on(&mut self, from: Id, on: Id);
    fn status(&mut self) -> Option<PlayerCount>;
}
