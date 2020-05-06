use crate::id::Id;
use crate::player::*;
use crate::player_repository::*;
use crate::scene::*;

type ActionRequest = (Id, Action, Id);

#[derive(Clone)]
struct GameScene<P: PlayerRepository> {
    pub status_requested: u32,
    pub players: P,
    events: Vec<ActionRequest>,
}

impl<P> GameScene<P>
where
    P: PlayerRepository,
{
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

    fn is_it_mafia(player: &mut Player) -> bool {
        if let Role::GodFather(ref mut n) = player.role {
            Self::use_power(n)
        } else {
            player.is_mafia()
        }
    }

    fn use_power(n: &mut u32) -> bool {
        if *n == 0 {
            return false;
        }
        *n -= 1;
        true
    }

    fn cast_on(action: Action, by: &mut Player, on: &mut Player) -> bool {
        let self_call = std::ptr::eq(by, on);
        match (&mut by.role, action) {
            (Role::Detective(ref mut mail), Action::MafiaInquery) => {
                mail.push(Self::is_it_mafia(on));
                true
            }
            (Role::Doctor(ref mut n), Action::Heal) => match (self_call, Self::use_power(n)) {
                (true, true) => Self::use_power(n) && on.set_state(LifeState::Alive),
                (false, _) => on.set_state(LifeState::Alive),
                _ => false,
            },
            (Role::Sniper(ref mut n), Action::HeadShot) => {
                match (Self::use_power(n), on.is_citizen()) {
                    (true, true) => by.set_state(LifeState::Injured),
                    (true, false) => on.set_state(LifeState::Injured),
                    _ => false,
                }
            }
            (Role::Psycho, Action::Kill) => on.set_state(LifeState::Injured),
            (Role::Silencer, Action::Slience) => on.set_state(LifeState::Silent),

            _ => false,
        }
    }
}

pub trait NightEvent {
    fn night_event(from: &mut Player, action: Action, on: &mut Player) -> bool;
}

impl<P> Scene for GameScene<P>
where
    P: PlayerRepository,
{
    fn wakeup(&mut self) -> State {
        self.eval_events();
        self.events.clear();
        self.state()
    }

    fn cast_on(&mut self, action: Action, from: Id, on: Id) {
        self.events.push((from, action, on));
    }

    fn status(&mut self) -> Option<PlayerCount> {
        if self.status_requested <= 0 {
            return None;
        }
        self.status_requested -= 1;
        Some(self.players.count())
    }
}
