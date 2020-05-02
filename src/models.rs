#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Id(u32);

impl Id {
    pub fn unique_random_id() -> Id {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub struct UserInfo(pub String);

type Bullets = u32;
type Total = u32;
type SelfHeals = u32;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Role {
    Mafia,
    Citizen,
    Armoured(Total),
    Sniper(Bullets),
    Doctor(SelfHeals),
    GodFather,
    Detective,
    Silencer,
    Natasha,
    Psycho,
}

impl Role {
    pub fn is_mafia(&self) -> bool {
        match self {
            Role::Mafia => true,
            Role::Silencer => true,
            Role::Natasha => true,
            Role::GodFather => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub user: UserInfo,
    pub role: Role,
}

pub struct WaitingRoom(Vec<UserInfo>);
impl WaitingRoom {
    pub fn init() -> WaitingRoom {
        WaitingRoom(Vec::new())
    }

    pub fn join(&mut self, user: UserInfo) {
        self.0.push(user);
    }
}