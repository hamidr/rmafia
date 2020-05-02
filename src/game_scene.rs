use std::collections::HashSet;
use std::convert::TryFrom;

use crate::models::{Id, Role};
use crate::players::Players;
use crate::scene::*;

#[derive(Clone)]
struct GameScene {
    pub status_requested: u32,
    pub players: Players,
    events: Vec<Action>,
    city: HashSet<Id>,
    cemetery: HashSet<Id>,
}

impl GameScene {
    fn init(players: &Players, status_call: u32) -> GameScene {
        GameScene {
            status_requested: status_call,
            players: players.clone(),
            events: Vec::new(),
            city: players.ids(),
            cemetery: HashSet::new(),
        }
    }

    fn role(&self, id: Id) -> Option<Role> {
        self.players.get(id).map(|p| p.role)
    }

    fn bury(&mut self, id: Id) -> bool {
        if self.city.remove(&id) {
            let res = self.cemetery.insert(id);
            return res;
        }
        false
    }

    fn mafia_count(&self) -> u32 {
        let c = self
            .city
            .iter()
            .filter(|pid| {
                self.players
                    .get(**pid)
                    .map(|p| p.role.is_mafia())
                    .unwrap_or(false)
            })
            .count();
        u32::try_from(c).unwrap_or(0)
    }
}

impl Scene for GameScene {
    fn wakeup(&self) -> State {
        unimplemented!()
    }

    fn apply(&mut self, from: Id, act: Action) -> Consequence {
        match act {
            Action::MafiaInquery(id) => {
                let mut res = false;
                let caller = self.players.get_mut(from).unwrap();
                if let Role::Doctor(leftover) = caller.role {
                    if leftover >= 1 {
                        caller.role = Role::Doctor(leftover - 1);
                        res = self.role(id).map(|r| match r {
                            Role::GodFather => false,
                            r => r.is_mafia(),
                        }).unwrap_or(false);
                    }  
                }
                Consequence::MafiaStatus(res)
            },
            _ => {
                self.events.push(act);
                unimplemented!()
            }
        }
    }
    /*    match req {
            Inquery::MafiaInquery(id) => {
                let is_mafia = self.role(id)
                    .map(|role| match role {
                        Role::GodFather => false,
                        r => r.is_mafia(),
                    })?;

                Some(InqueryResult::MafiaCheck(is_mafia))
            }*/
    fn status(&mut self) -> Option<InqueryStatus> {
        if self.status_requested == 0 {
            return None;
        }
        self.status_requested -= 1;
        Some(self.mafia_count())
    }
}
