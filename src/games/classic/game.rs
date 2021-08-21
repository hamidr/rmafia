
use std::{collections::BTreeMap};

use crate::{in_memory_room::{InMemoryRoom, OneToOneSpells}, room::{Room, Spells}, scenario::*, waiting::{GodWayRef, PlayerId}};

use super::{play::*};
use crate::elections::ballot::Ballots;
use rand::{prelude::SliceRandom, thread_rng};

pub struct Classic {
    strategy: Play,
    room: InMemoryRoom,
    state: CityState,
    events: Vec<(Day, CityState, Declaration)>,
    enquery: u8,
    day: usize,
}

impl Classic {
    pub fn new(players: BTreeMap<PlayerId, GodWayRef>) -> Result<Self, Error> {
        if players.len() != 10 {
            return Err("Invalid number of players".to_owned())
        }

        let mut res = Self {
            strategy: Play::new(),
            room: InMemoryRoom::new(players),
            state: CityState::Debate,
            events: vec![],
            enquery: 3,
            day: 0,
        };
        res.assign_roles();
        Ok(res)
    }

    fn assign(&mut self, players: &mut Vec<PlayerId>, powers: Vec<Power>) {
        let id = players.pop().unwrap();
        assert!(self.room.assign(&id, powers.clone()));
    }

    fn assign_roles(&mut self) {
        use Power::*;
        let mut players = self.room.numbers();
        players.shuffle(&mut thread_rng());

        self.assign(&mut players, vec![Mafia, NightKill, DodgeCommando, Disguise]);
        self.assign(&mut players, vec![Mafia, Paralyze]);
        self.assign(&mut players, vec![Mafia, Reveal]);

        self.assign(&mut players, vec![HandFakeGun, HandGun]);
        self.assign(&mut players, vec![Heal]);
        self.assign(&mut players, vec![Guard]);
        self.assign(&mut players, vec![ShotOnKill]);
        self.assign(&mut players, vec![Enquery]);
        self.assign(&mut players, vec![]);
        self.assign(&mut players, vec![]);
    }

    fn enquery(&mut self) -> Option<usize> {
        if self.enquery == 0 {
            None
        } else {
            self.enquery -= 1;
            Some(self.room.count(&Power::Mafia))
        }
    }

    fn make_ballot(&self) -> Ballots {
        Ballots::new(self.room.total())
    }

    fn first_id_by_power(&mut self, power: &Power) -> Option<PlayerId> {
        let mut x = self.room.by_power(power).pop()?;
        x.on().clone().pop()
    }

    fn pass_night_kill(&mut self) -> bool {
        let pid = self.first_id_by_power(&Power::Reveal)
        .or_else(|| self.first_id_by_power(&Power::Paralyze));

        if let Some(id) = pid {
            self.room.assign(&id, vec![Power::NightKill]);
            return true
        }
        false
    }
    
    fn kick_out(&mut self, id: &PlayerId) -> bool {
        let out = self.room.remove(id);
        if out.contains(&Power::NightKill) {
            self.pass_night_kill();
        }

        if out.len() >= 1 {
            self.declare(Declaration::Out(id.clone()));
            return true
        }
        false
    }

    fn is_it(&self, state: CityState) -> bool {
        self.state == state
    }

    fn declare(&mut self, s: Declaration) {
        self.events.push((self.day, self.state, s));
    }
    
    fn on_shooting(&mut self, res: ShootingResult) {
        match res {
            ShootingResult::EmptyGun(id) => {
                self.declare(Declaration::FakeGun(id));
            },
            ShootingResult::Killed(id) => {
                if self.kick_out(&id) {
                    self.declare(Declaration::Out(id));
                }
            },
            ShootingResult::NotAllowed => {}
        }
    }

    fn darkness(&mut self, spells: &impl Spells) -> Result<NightResult, Error> {
        let spells = self.room.read_all();
        let s = OneToOneSpells::new();
        self.strategy.apply_night(&mut self.room, s)
    }

    fn sunrise(&mut self, n: &impl News) {
        for n in n.kicked_out() {
            self.kick_out(n);
        }
    }

    fn sunset(&mut self, d: &impl DeathBallot) {
        if let Some(ref id) = d.dead() {
            if self.room.has(id, &Power::Guard) {
                self.kick_out(id);
            }
        }
    }

    fn defend(&self) -> Option<Ballots> {
        if self.is_it(CityState::Defend) {
            return Some(self.make_ballot());
        }
        None
    }

    fn game_state(&self) -> State {
        let mafia = self.room.count(&Power::Mafia);
        let city = self.room.total() - mafia;
        if city <= mafia {
            State::MafiaWon
        } else if mafia == 0 {
            State::CityWon
        } else {
            State::Undecided
        }
    }
}


impl Scenario for Classic {
    fn state(&self) -> CityState {
        self.state
    }

    fn day(&self) -> usize {
        self.day
    }

    fn next(&mut self) -> CityState {
        let game = self.game_state();
        if game != State::Undecided {
            return CityState::Done(game);
        }
        self.state = match self.state {
            CityState::Debate => CityState::Defend,
            CityState::Defend => CityState::Hang,
            CityState::Hang => CityState::Night,
            CityState::Night => {
                self.day += 1;
                CityState::Debate
            },
            d@CityState::Done {..} => d
        };
        self.declare(Declaration::StateChanged(self.state));
        self.state
    }

    fn process(&mut self) {
    }

    fn events(&self) -> &Vec<(Day, CityState, Declaration)> {
        &self.events
    }
}
