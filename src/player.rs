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

#[derive(Debug, Clone)]
pub struct Player {
    user: UserInfo,
    role: Role,
    state: LifeState,
}

impl Player {
    pub fn new(user: UserInfo, role: Role) -> Player {
        Player {
            user: user,
            role: role,
            state: LifeState::Alive
        }
    }

    fn is_it_mafia(player: &mut Player) -> bool {
        if let Role::GodFather(ref mut n) = player.role {
            Self::use_power(n)
        } else {
            player.is(RoleKind::Mafia)
        }
    }

    fn use_power(n: &mut u32) -> bool {
        if *n == 0 {
            return false;
        }
        *n -= 1;
        true
    }

    pub fn kind(&self) -> RoleKind {
        match self.role {
            Role::GodFather(..) | Role::Silencer | Role::Spy => RoleKind::Mafia,
            Role::Psycho => RoleKind::Psycho,
            _ => RoleKind::Citizen
        }
    }
}

pub enum RoleKind {
    Citizen,
    Mafia,
    Psycho
}

pub trait Caster: Sized {
    fn info(&self) -> &UserInfo;
    fn is_alive(&self) -> bool;
    fn is(&self, k: RoleKind) -> bool;
    fn kind(&self) -> RoleKind;
    fn state(&self) -> LifeState;

    fn set_state(&mut self, state: LifeState) -> bool;
    fn cast_on(&mut self, on: &mut Self) -> bool;
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
            Role::Sniper(ref mut n) => match (Self::use_power(n), on.is(RoleKind::Citizen)) {
                (true, true) => self.set_state(LifeState::Injured),
                (true, _) => on.set_state(LifeState::Injured),
                _ => false,
            },
            Role::Psycho => on.set_state(LifeState::Injured),
            Role::Silencer => on.set_state(LifeState::Silent),

            _ => false,
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self.state, LifeState::Alive | LifeState::Injured | LifeState::Silent)
    }

    fn is(&self, k: RoleKind) -> bool {
        matches!(self.kind(), k)
    }

    fn set_state(&mut self, state: LifeState) -> bool {
        let s = match self.state {
            LifeState::Silent => matches!(state, LifeState::Alive | LifeState::Injured),
            LifeState::Alive => matches!(state, LifeState::Injured | LifeState::Silent),
            LifeState::Injured => matches!(
                state,
                LifeState::Alive | LifeState::Killed | LifeState::Hanged
            ),
            _ => state == self.state,
        };
        if s {
            self.state = state;
        }
        s
    }

    fn state(&self) -> LifeState {
        self.state
    }

    fn kind(&self) -> RoleKind {
        Player::kind(self)
    }
}
