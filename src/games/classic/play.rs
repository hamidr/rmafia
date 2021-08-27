use btreemultimap::BTreeMultiMap;

use crate::scenario::*;
use crate::room::*;
use crate::waiting::PlayerId;

use std::collections::{BTreeSet, BTreeMap};

pub enum DayEvent {
    RealGun(PlayerId),
    FakeGun
}

pub enum KillingStatus {
    BossKilled(PlayerId),
    WickedActed(PlayerId, Option<PlayerId>),
    CommandoActed(PlayerId, Option<PlayerId>),
}


pub struct Play {
    day: BTreeMultiMap<PlayerId, DayEvent>
}

impl Play {
    pub fn new() -> Self {
        Self { day: BTreeMultiMap::new() }
    }
    
    fn remove_paralyzed_unguarded_spell(room: &impl Room, spells: &mut impl Spells) {
        let guarded = spells.expect1(&Power::Guard);
        let paralyzed = spells.expect1(&Power::Paralyze);

        match (paralyzed, guarded) {
            (Some(p), Some(g)) if p == g => return,
            (Some(p), _) => {
                for ref n in room.kinks(&p) {
                    spells.stop(n);
                }
            }
            _ => {},
        }
    }

    fn did_hit_mafia(room: &impl Room, commando: PlayerId, target: PlayerId) -> Option<KillingStatus> {
        use KillingStatus::*;
        let kinks = room.kinks(&target);
        if kinks.contains(&Power::DodgeCommando) {
            Some(CommandoActed(commando, None))
        } else if kinks.contains(&Power::Mafia) {
            Some(CommandoActed(commando, Some(target)))
        } else {
            None
        }
    }

    fn is_wicked_or_boss_killing(room: &mut impl Room, spells: &impl Spells) -> Option<KillingStatus> {
        spells.raw(&Power::Reveal)
        .or_else(|| spells.raw(&Power::NightKill))
        .and_then(|night_act| match night_act {
            (wicked, Power::Reveal, NightAct::Wicked(citizen, role)) =>
                Some(Self::who_wicked_kills(room, wicked.clone(), citizen.clone(), &role)),

            (gf, Power::NightKill, NightAct::One(player)) if room.has(player, &Power::ShotOnKill) => {
                let res = spells.one(&Power::ShotOnKill)
                .map_or(KillingStatus::BossKilled(player.clone()), |(commando, _, target)| {
                    Self::did_hit_mafia(room , commando, target)
                    .unwrap_or(KillingStatus::BossKilled(player.clone()))
                });
                Some(res)
            },
            (_, Power::NightKill, NightAct::One(citizen)) => Some(KillingStatus::BossKilled(citizen.clone())),
            _ => None
        })
    }

    fn who_wicked_kills(room:& impl Room, wicked: PlayerId, killee: PlayerId, power: &Power) -> KillingStatus {
        if room.has(&killee, &power) {
            KillingStatus::WickedActed(wicked, Some(killee))
        } else {
            KillingStatus::WickedActed(wicked, None)
        }
    }

    fn get_player_ids(spells: &impl Spells, power: &Power) -> Vec<PlayerId> {
        let mut data= Vec::new();
        spells.get(power).map(|n| match n {
            NightAct::One( a) => data.push(a.clone()),
            NightAct::Two(a, b) => {
                data.push(a.clone());
                data.push(b.clone());
            },
            _ => {}
        });
        data
    }

    fn heals(spells: &impl Spells, total: usize, ) -> BTreeSet<PlayerId> {
        let ids = Self::get_player_ids(spells, &Power::Heal).into_iter();
        let n = if total >= 8 { 2 } else { 1 };
        ids.take(n).collect()
    }

    fn remove_killed_one(room: &mut impl Room, spells: &impl Spells)  -> Result<Option<PlayerId>, Error> {
        use self::KillingStatus::*;
        let heals = Self::heals(spells, room.total());
        let killing = Self::is_wicked_or_boss_killing(room, spells)
            .ok_or("No Kill command".to_owned())?;

        let res = match killing {
            BossKilled(p) if !heals.contains(&p) => Some(p),
            CommandoActed(from, res)  => {
                room.drop_kinks(&from, [Power::ShotOnKill]);
                match res {
                    Some(id) if heals.contains(&id) => None,
                    p => p
                }
            },
            WickedActed(from, p) => {
                room.drop_kinks(&from, [Power::Reveal]); p
            },
            _ => None,
        };
        Ok(res)
    }

    fn detective(room: &impl Room, msgs: &mut Messages, spells: &impl Spells) {
        let guess = spells.one(&Power::Enquery);

        if let Some((from, _, on)) = guess {
            let kinks = room.kinks(&on);
            let mut is_mafia = kinks.contains(&Power::Mafia);
            is_mafia &= !kinks.contains(&Power::Disguise);
            msgs.insert(from.clone(), HolyMessage::IsMafia(on.clone(), is_mafia));
        }
    }
    
    fn gunman(&mut self, spells: &impl Spells) {
        let act = spells.raw_vec(&Power::HandGun)
        .or_else(|| spells.raw_vec(&Power::HandFakeGun));

        for (gunman, power, night_act) in act.unwrap_or(Vec::new()) {
            match (power, night_act) {
                (Power::HandGun, NightAct::One(p1)) => {
                    self.day.insert(p1.clone(), DayEvent::RealGun(gunman.clone()));
                },
                (Power::HandGun, NightAct::Two(p1, p2)) => {
                    self.day.insert(p1.clone(), DayEvent::RealGun(gunman.clone()));
                    self.day.insert(p2.clone(), DayEvent::FakeGun);
                },
                (Power::HandFakeGun, NightAct::One(p1)) => {
                    self.day.insert(p1.clone(), DayEvent::FakeGun);
                },
                (Power::HandFakeGun, NightAct::Two(p1, p2)) => {
                    self.day.insert(p2.clone(), DayEvent::FakeGun);
                    self.day.insert(p2.clone(), DayEvent::FakeGun);
                },
                _ => {}
            };
        };
    }

    pub fn apply_night(&mut self, room: &mut impl Room, mut spells: impl Spells) -> Result<NightResult, Error> {
        self.day.clear();
        Self::remove_paralyzed_unguarded_spell(room, &mut spells);

        let mut deads = BTreeSet::new();
        if let Some(killed) = Self::remove_killed_one(room, &mut spells)? {
            deads.insert(killed);
        }

        let mut msgs = BTreeMultiMap::new();
        self.gunman(&spells);
        Self::detective(room, &mut msgs, &spells);

        let result = NightResult::new(msgs, deads);
        Ok(result)
    }   

    pub fn shoot(&mut self, room: &mut impl Room, shooter: &PlayerId, on: PlayerId) -> ShootingResult {
        self.day.get_vec(shooter).unwrap_or(&Vec::new())
        .iter()
        .find_map(|p| match p {
            DayEvent::RealGun(gunman) =>  Some(ShootingResult::Killed(on)),
            DayEvent::FakeGun => Some(ShootingResult::EmptyGun(on))
        }).unwrap_or(ShootingResult::NotAllowed)
    }   
}

pub struct NightResult {
    msgs: Messages,
    removed: BTreeSet<PlayerId>
}

impl NightResult {
    fn new(msgs: Messages, removed: BTreeSet<PlayerId>) -> Self {
        Self { msgs, removed }
    }
}

impl News for NightResult {
    fn messages(&self) -> &Messages {
        &self.msgs
    }

    fn kicked_out(&self) -> &BTreeSet<PlayerId> {
       &self.removed 
    }
}