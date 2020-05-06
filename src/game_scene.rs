use crate::id::Id;
use crate::player::*;
use crate::player_repository::*;
use crate::scene::*;

type ActionRequest = (Id, Id);

#[derive(Clone)]
struct GameScene<P> {
    pub status_requested: u32,
    pub players: P, 
    events: Vec<ActionRequest>,
}

impl<P: PlayerRepository> GameScene<P> {
    fn new(players: P, status_limit: u32) -> GameScene<P> {
        GameScene {
            status_requested: status_limit,
            players: players,
            events: Vec::new(),
        }
    }

    fn eval_events(&mut self) {
        unimplemented!()
    }

    fn state(&self) -> State {
        let count = self.players.count();
        match (count.mafia, count.citizen, count.psycho) {
            (m, c, p) if m == c && m >= 1 && p == 0 => State::Lost,
            (m, _, p) if m == 0 && p == 0 => State::Won,
            (m, c, p) if (m + c) == 0 && p == 1 => State::WTF,
            _ => State::Fighting,
        }
    }
}
impl<P: PlayerRepository> Scene for GameScene<P> {
    fn wakeup(&mut self) -> State {
        self.eval_events();
        self.events.clear();
        self.state()
    }

    fn cast_on(&mut self, from: Id, on: Id) {
        self.events.push((from, on));
    }

    fn status(&mut self) -> Option<PlayerCount> {
        if self.status_requested <= 0 {
            return None;
        }
        self.status_requested -= 1;
        Some(self.players.count())
    }
}