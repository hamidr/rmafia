use crate::player::UserInfo;

pub trait Room {
    fn users(&self) -> &Vec<UserInfo>;
}

pub struct RoomSpace {
    names: Vec<UserInfo>,
    locked: bool,
}

impl RoomSpace {
    pub fn new() -> RoomSpace {
        RoomSpace {
            names: Vec::new(),
            locked: false,
        }
    }

    pub fn join(&mut self, user: UserInfo) -> Result<&RoomSpace, &'static str> {
        if self.locked {
            Err("Locked")
        } else {
            self.names.push(user);
            Ok(self)
        }
    }

    pub fn switch(&mut self) -> &RoomSpace {
        self.locked = !self.locked;
        self
    }
}

impl Room for RoomSpace {
    fn users(&self) -> &Vec<UserInfo> {
        &self.names
    }
}
