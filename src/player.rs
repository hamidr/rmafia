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

    pub fn set_state(&mut self, state: LifeState) -> bool {
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

    pub fn inc_wanted(&mut self) {
        self.wanted += 1;
    }

    pub fn reset_wanted(&mut self) {
        self.wanted = 0;
    }

    pub fn is_conscious(&self) -> bool {
        matches!(
            self.state,
            LifeState::Alive | LifeState::Silent | LifeState::Injured
        )
    }

    pub fn is_mafia(&self) -> bool {
        matches!(self.role, Role::GodFather(..) | Role::Silencer | Role::Spy)
    }

    pub fn is_citizen(&self) -> bool {
        match self.role {
            Role::Psycho => true,
            _ => !self.is_mafia(),
        }
    }
}
