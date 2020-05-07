use crate::player_repository::{PlayerCount, Id};

pub enum Progress {
    Fighting,
    Won,
    Lost,
    WTF,
}

pub trait Scene: Sized {
    fn wakeup(&mut self) -> Progress;
    fn cast_on(&mut self, from: Id, on: Id);
    fn status(&mut self) -> Option<PlayerCount>;
}
