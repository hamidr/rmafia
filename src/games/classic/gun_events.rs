use std::collections::BTreeSet;

use crate::waiting::PlayerId;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum GunOwner {
    Real, Fake, Invalid
}

pub struct GunEvents {
    limit: usize,
    fakes: BTreeSet<PlayerId>,
    reals: BTreeSet<PlayerId>,
}

impl GunEvents {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            fakes: BTreeSet::new(),
            reals: BTreeSet::new()
        }
    }

    fn len(&self) -> usize {
        self.fakes.len() + self.reals.len()
    }

    pub fn pass_real(&mut self, id: PlayerId) -> bool {
        if self.len() >= self.limit {
            return false
        }
        self.reals.insert(id)
    }

    pub fn pass_fake(&mut self, id: PlayerId) -> bool {
        if self.len() >= 2 {
            return false
        }
        self.fakes.insert(id)
    }

    pub fn try_shooting(&mut self, id: &PlayerId) -> GunOwner {
        if self.reals.contains(&id) {
            self.reals.clear();
            GunOwner::Real
        } else if self.fakes.contains(id) {
            self.fakes.clear();
            GunOwner::Fake
        } else {
            GunOwner::Invalid
        }
    }

    pub fn owners(&self) -> BTreeSet<PlayerId> {
        let mut set: BTreeSet<PlayerId> = BTreeSet::new();
        set.extend(self.fakes.clone());
        set.extend(self.reals.clone());
        set
    }
}
