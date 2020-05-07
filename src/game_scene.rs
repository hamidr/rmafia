use std::sync::Arc;

use crate::player::*;
use crate::player_repository::*;
use crate::scene::*;

type ActionRequest = (Id, Id);

#[derive(Clone)]
struct GameScene<R: PlayerRepository> {
    status_requested: u32,
    players: R,
    events: Vec<ActionRequest>,
}

impl<R: PlayerRepository> GameScene<R> {
    fn new(players: R, status_limit: u32) -> GameScene<R> {
        GameScene {
            status_requested: status_limit,
            players: players,
            events: Vec::new(),
        }
    }

    fn eval_events(&mut self) -> Option<()> {
        for (a, b) in &self.events {
            let mut ra = self.players.get(a)?;
            let mut rb = self.players.get(b)?;
            let from = Arc::get_mut(&mut ra)?;
            let on = Arc::get_mut(&mut rb)?;
            from.cast_on(on);
        }

        Some(())
    }

    fn state(&self) -> Status {
        let count = self.players.count_alives();
        match (count.mafia, count.citizen, count.psycho) {
            (m, c, p) if m == c && m >= 1 && p == 0 => Status::Lost,
            (m, _, p) if m == 0 && p == 0 => Status::Won,
            (m, c, p) if (m + c) == 0 && p != 0 => Status::WTF,
            _ => Status::Fighting,
        }
    }
}

impl<R: PlayerRepository> Scene for GameScene<R> {
    fn wakeup(&mut self) -> Status {
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
        Some(self.players.count_alives())
    }
}
