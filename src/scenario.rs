use std::{collections::{BTreeMap, BTreeSet}, vec};

use btreemultimap::BTreeMultiMap;

use crate::waiting::PlayerId;

pub type Error = String;

pub enum ShootingResult {
    Killed(PlayerId),
    EmptyGun(PlayerId),
    NotAllowed
}


#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Power {
    Guard,
    Paralyze,
    NightKill,
    Reveal,
    Heal,
    Enquery,
    HandGun,
    HandFakeGun,
    ShotOnKill,

    Disguise,
    DodgeCommando,
    Mafia,
    DayShield,
}

impl Power {
    pub fn night(&self) -> bool {
        self.active().contains(self)
    }

    pub fn active(&self) -> [Power; 9] {
        use Power::*;
        [NightKill, Reveal, Paralyze, Heal, Enquery, Guard, HandGun, HandFakeGun, ShotOnKill]
    }

    pub fn passive(&self) -> [Power; 4] {
        use Power::*;
        [Disguise, DodgeCommando, Mafia, DayShield]
    }
}

#[derive(Clone)]
pub enum HolyMessage {
    Assigned(Vec<Power>),
    YouHaveGun,
    YouAreBoss,
    IsMafia(PlayerId, bool),
}

pub enum Meta {
    Has(Power)
}

pub struct Pray {
    pub action: Power,
    pub query: Vec<PlayerId>,
    pub meta: Option<Meta>
}

// impl Pray {
//     fn is_consistent(&self) -> bool {
//         use Power::*;
//         match (&self.action, &self.query, &self.meta) {
//             (Reveal, ids, Some(Meta::Has(Power::Heal|Power::DayShield|Power::Enquery|Power::ShotOnKill))) if ids.len() == 1 => true,
//             (NightKill|Paralyze|Heal|Enquery|HandGun|HandFakeGun|DayShield, ids, None) if ids.len() <= 2 => true,
//             (ShotOnKill, ids, None) if ids.len() == 1 => true,
//             _ => false
//         }
//     }

//     pub fn on(&self) -> &Vec<PlayerId> {
//         &self.query
//     }
// }

pub type Messages = BTreeMultiMap<PlayerId, HolyMessage>;

// pub type Spells<'a> = BTreeMap<Role, Spell<'a>>;

pub trait News {
    fn messages(&self) -> &Messages;
    fn kicked_out(&self) -> &BTreeSet<PlayerId>;
}

pub trait DeathBallot {
    fn list(&self) -> &BTreeSet<PlayerId>;
    fn hang(&mut self, from: PlayerId, on: PlayerId) -> bool;
    fn dead(&self) -> Option<PlayerId>;
}

pub trait Defendence {
    type Ballot: DeathBallot;

    fn nominate(&mut self, from: PlayerId, on: PlayerId) -> bool;
    fn result(&self) -> Option<Self::Ballot>;
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum CityState {
    Debate,
    Defend,
    Hang,
    Night,
    Done(State)
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum State {
    MafiaWon,
    CityWon,
    Undecided
}

#[derive(Clone)]
pub enum Declaration {
    Out(PlayerId),
    StateChanged(CityState),
    FakeGun(PlayerId)
}

pub type Day = usize;

pub trait Scenario {
    fn state(&self) -> CityState;
    fn day(&self) -> Day;
    fn next(&mut self) -> CityState;
    fn events(&self) -> &Vec<(Day, CityState, Declaration)>;

    fn process(&mut self);
}