use std::{collections::{BTreeMap, BTreeSet}, iter::FromIterator};


pub type PlayerId = u32;

// impl Ord for Player {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.id.cmp(&other.id)
//     }
// }

// impl PartialOrd for Player {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for Player {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }

// impl Eq for Player {}

use crate::{oracle::{Oracle, TwoWayRing}, scenario::{HolyMessage, Power, Pray}};

pub type OracleRef = TwoWayRing<Pray, HolyMessage>;
pub type GodWayRef = TwoWayRing<HolyMessage, Pray>;

pub trait WaitingRoom {
    type Oracle;
    type God;
    fn reserve(&mut self) -> Option<(PlayerId, Self::Oracle)>;
    fn opt_out(&mut self, id: &PlayerId) -> bool;
    fn limit(&self) -> usize;
    fn ready(&self) -> bool;
    fn get(self) -> Option<BTreeMap<PlayerId, Self::God>>;
}

pub struct WaitingBuilder {
    limit: usize,
    waiting: BTreeMap<PlayerId, GodWayRef>,
}

impl WaitingBuilder {
    pub fn new(limit: usize) -> WaitingBuilder {
        WaitingBuilder {
            waiting: BTreeMap::new(),
            limit
        }
    }

    #[inline]
    fn total(&self) -> usize {
        self.waiting.len()
    }

    fn next_id(&self) -> Option<PlayerId> {
        let range = 1..self.limit as u32;
        let mut ids: BTreeSet<u32> = BTreeSet::from_iter(range.into_iter());
        for i in self.waiting.keys() {
            ids.remove(i);
        }
        ids.into_iter().next()
    }
}

impl WaitingRoom for WaitingBuilder {
    type Oracle = TwoWayRing<Pray, HolyMessage>;
    type God = TwoWayRing<HolyMessage, Pray>;

    fn reserve(&mut self) -> Option<(PlayerId, Self::Oracle)> {
        let id = self.next_id()?;
        if self.ready() || self.waiting.contains_key(&id) {
            return None
        }

        let (channel, oracle) = Oracle::create();
        self.waiting.insert(id, channel);
        Some((id, oracle))
    }

    #[inline]
    fn limit(&self) -> usize {
        self.limit
    }

    fn opt_out(&mut self, id: &PlayerId) -> bool {
        self.waiting.remove(id).is_some()
    }

    #[inline]
    fn ready(&self) -> bool {
        self.total() == self.limit()
    }

    fn get(self) -> Option<BTreeMap<PlayerId, Self::God>> {
        if self.ready() {
            Some(self.waiting)
        } else {
            None
        }
    }
}