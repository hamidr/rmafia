use std::collections::{BTreeSet,BTreeMap};
use crate::{scenario::*, waiting::PlayerId};

pub struct DeathVote {
    total: u8,
    list: BTreeSet<PlayerId>,
    votes: BTreeMap<PlayerId, BTreeSet<PlayerId>>
}

impl DeathVote {
    fn new(nomonies: BTreeSet<PlayerId>, total: u8) -> Result<DeathVote, Error> {
        assert!(total >= 3);
        if nomonies.len() == 0 {
            return Err("No nominies found".to_owned())
        }
        let mut list = BTreeMap::new();
        for p in nomonies.iter() {
            list.insert(p.clone(), BTreeSet::new());
        }
        
        let vote = Self {
            total: total,
            list: nomonies,
            votes: list
        };
        Ok(vote)
    }
}

impl DeathBallot for DeathVote {
    fn list(&self) -> &BTreeSet<PlayerId> {
        &self.list
    }

    fn hang(&mut self, from: PlayerId, on: PlayerId) -> bool {
        if from == on || !self.list.contains(&on) {
            return false;
        }
        if self.list.len() == 2 && self.list.contains(&from) {
            return false;
        }
        if let Some(votes) = self.votes.get_mut(&on) {
            votes.insert(from)
        } else {
            false
        }
    }

    fn dead(&self) -> Option<PlayerId> {
        let half = (self.total / 2) as usize;
        let mut ns =  self.votes.iter().filter_map(|(p, v)| {
            if v.len() >= half {
                Some((p, v.len()))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        ns.sort_by(|(_, v1), (_, v2)| {
            v1.cmp(&v2)
        });
        ns.last().map(|&(p, _)| p.clone())
    }
}

pub struct Ballots {
    size: u8,
    votes: BTreeMap<PlayerId, BTreeSet<PlayerId>>
}

impl Ballots {
    pub fn new(total: usize) -> Ballots {
        assert!(total >= 3);
        Self {
            size: total as u8,
            votes: BTreeMap::new()
        }
    }    
}

impl Defendence for Ballots {
    type Ballot = DeathVote;

    fn nominate(&mut self, from: PlayerId, on: PlayerId) -> bool {
        if from == on {
            return false
        }

        if let Some(votes) = self.votes.get_mut(&on) {
            votes.insert(from)
        } else {
            let mut new = BTreeSet::new();
            new.insert(from);
            self.votes.insert(on, new).is_none()
        }
    }

    fn result(&self) -> Option<Self::Ballot> {
        let half = self.size / 2;
        let nomonies = self.votes.iter().filter_map(|(player, votes)| {
            if votes.len() >= (half as usize) {
                Some(player.clone())
            } else {
                None                
            }
        }).collect::<BTreeSet<_>>();
        
        if nomonies.len() != 0 {
            Some(DeathVote::new(nomonies, self.size).unwrap())
        } else {
            None
        }
    }
}