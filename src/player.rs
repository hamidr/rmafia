use crate::id::Id;
use getset::{CopyGetters, Getters};

#[derive(Debug, Clone)]
pub struct UserInfo(pub String);

type MailBox = Vec<bool>;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Peasant,
    Armoured(u32),
    Doctor(u32),
    Detective(MailBox),
    Sniper(u32),
    Natasha,
    GodFather(u32),
    Silencer,
    Spy,
    Psycho,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LifeState {
    Alive,
    Silent,
    Injured,
    Hanged,
    Killed,
}

#[derive(Getters, CopyGetters, Debug, Clone)]
pub struct Player {
    #[getset(get = "pub")]
    user: UserInfo,
    pub role: Role,
    pub state: LifeState,
    pub wanted: u32,
}

impl Player {
    pub fn new(user: UserInfo, role: Role) -> Player {
        Player {
            user: user,
            role: role,
            state: LifeState::Alive,
            wanted: 0,
        }
    }

    fn set_state(&mut self, state: LifeState) -> bool {
        let s = match self.state {
            LifeState::Silent => matches!(state, LifeState::Alive | LifeState::Injured),
            LifeState::Alive => matches!(state, LifeState::Injured | LifeState::Silent),
            LifeState::Injured => matches!(
                state,
                LifeState::Alive | LifeState::Killed | LifeState::Hanged
            ),
            _ => false,
        };
        if s {
            self.state = state;
        }
        s
    }

    fn inc_wanted(&mut self) {
        self.wanted += 1;
    }

    fn reset_wanted(&mut self) {
        self.wanted = 0;
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
}

pub trait Caster : Sized {
    fn info(&self) -> &UserInfo;
    fn cast_on(&mut self, on: &mut Self) -> bool;
    fn is_citizen(&self) -> bool;
    fn is_alive(&self) -> bool;
    fn is_mafia(&self) -> bool;
}

impl Caster for Player {
    fn info(&self) -> &UserInfo {
        &self.user
    }

    fn cast_on(&mut self, on: &mut Self) -> bool {
        let self_call = std::ptr::eq(self, on);
        match &mut self.role {
            Role::Detective(ref mut mail) => {
                mail.push(Self::is_it_mafia(on));
                true
            }
            Role::Doctor(ref mut n) => match (self_call, Self::use_power(n)) {
                (true, true) => Self::use_power(n) && on.set_state(LifeState::Alive),
                (false, _) => on.set_state(LifeState::Alive),
                _ => false,
            },
            Role::Sniper(ref mut n) => {
                match (Self::use_power(n), on.is_citizen()) {
                    (true, true) => self.set_state(LifeState::Injured),
                    (true, false) => on.set_state(LifeState::Injured),
                    _ => false,
                }
            }
            Role::Psycho => on.set_state(LifeState::Injured),
            Role::Silencer => on.set_state(LifeState::Silent),

            _ => false,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(
            self.state,
            LifeState::Alive | LifeState::Silent | LifeState::Injured
        )
    }

    fn is_mafia(&self) -> bool {
        matches!(self.role, Role::GodFather(..) | Role::Silencer | Role::Spy)
    }

    fn is_citizen(&self) -> bool {
        match self.role {
            Role::Psycho => true,
            _ => !self.is_mafia(),
        }
    }
}
