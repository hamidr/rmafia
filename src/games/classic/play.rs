use crate::scenario::*;
use crate::room::*;
use crate::waiting::PlayerId;

use std::collections::{BTreeSet, BTreeMap};
use crate::games::classic::gun_events::*;

pub struct Play {
    shotgun_done: bool,
    wicked_used: bool,
    commando_used: bool,
    gun: Option<GunEvents>,
}

pub enum KillingStatus {
    BossKilled(PlayerId),
    WickedActed(Option<PlayerId>),
    CommandoActed(Option<PlayerId>),
}


impl Play {
    pub fn new() -> Self {
        Self {
            shotgun_done: false,
            wicked_used: false,
            commando_used: false,
            gun: None,
        }
    }
    
    pub fn has_gun(&self) -> bool {
        self.gun.is_some()
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

    fn did_hit_mafia(room: &impl Room, target: PlayerId) -> Option<KillingStatus> {
        use KillingStatus::*;
        let kinks = room.kinks(&target);
        if kinks.contains(&Power::DodgeCommando) {
            Some(CommandoActed(None))
        } else if kinks.contains(&Power::Mafia) {
            Some(CommandoActed(Some(target)))
        } else {
            None
        }
    }

    fn can_commando_act(&self, room: &impl Room, player: &PlayerId) -> bool {
        self.commando_used && room.has(player, &Power::ShotOnKill)
    }

    fn is_wicked_or_boss_killing<'a>(&self, room: &impl Room, spells: &impl Spells) -> Option<KillingStatus> {
        let killing = if self.wicked_used { None }  else { spells.get_kv(&Power::Reveal) };
        killing.or_else(|| spells.get_kv(&Power::NightKill))
        .and_then(|night_act| match night_act {
            (Power::Reveal, NightAct::Wicked(citizen, role)) => 
                Some(Self::who_wicked_kills(room, citizen.clone(), &role)),

            (Power::NightKill, NightAct::One(player)) if self.can_commando_act(room, player) => {
                let res = spells.expect1(&Power::ShotOnKill)
                .map_or(KillingStatus::BossKilled(player.clone()), |commando_act| {
                    Self::did_hit_mafia(room , commando_act.clone())
                    .unwrap_or(KillingStatus::BossKilled(player.clone()))
                });
                Some(res)
            },
            (Power::NightKill, NightAct::One(citizen)) => 
                Some(KillingStatus::BossKilled(citizen.clone())),
            _ => None
        })
    }

    fn who_wicked_kills(room:& impl Room, killee: PlayerId, power: &Power) -> KillingStatus {
        if room.has(&killee, &power) {
            KillingStatus::WickedActed(Some(killee))
        } else {
            KillingStatus::WickedActed(None)
        }
    }

    fn get_player_ids(&self, spells: &impl Spells, power: &Power) -> Vec<PlayerId> {
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

    fn heals(&self, total: usize, spells: &impl Spells) -> BTreeSet<PlayerId> {
        let ids = Self::get_player_ids(&self, spells, &Power::Heal).into_iter();
        let n = if total >= 8 { 2 } else { 1 };
        ids.take(n).collect()
    }

    fn remove_killed_one(&mut self, room: &impl Room, spells: &impl Spells)  -> Result<Option<PlayerId>, Error> {
        use self::KillingStatus::*;
        let heals = self.heals(room.total(), spells);
        let killing = self.is_wicked_or_boss_killing(room, spells)
            .ok_or("No Kill command".to_owned())?;

        let res = match killing {
            BossKilled(p) if !heals.contains(&p) => Some(p),
            CommandoActed(Some(p)) if !heals.contains(&p) => {
                self.commando_used = true;
                Some(p)
            },
            WickedActed(Some(p)) => {
                self.wicked_used = true;
                Some(p)
            },
            CommandoActed(None) => {
                self.commando_used = true;
                None
            },
            _ => None,
        };
        Ok(res)
    }

    fn detective(&self, room: &impl Room, msgs: &mut Messages, spells: &impl Spells) {
        let guess = spells.raw(&Power::Enquery);

        if let Some((_, (from, NightAct::One(on)))) = guess {
            let kinks = room.kinks(on);
            let mut is_mafia = kinks.contains(&Power::Mafia);
            is_mafia &= !kinks.contains(&Power::Disguise);
            msgs.insert(from.clone(), HolyMessage::IsMafia(on.clone(), is_mafia));
        }
    }
    
    fn gunman(&self, spells: &impl Spells, msgs: &mut Messages) -> Option<GunEvents> {
        if self.shotgun_done {
            return None
        }

        let act = spells.get_kv(&Power::HandGun)
        .or_else(|| spells.get_kv(&Power::HandFakeGun))?;

        let mut guns = GunEvents::new(2);
        match act {
            (Power::HandGun, NightAct::One(p1)) => {
                guns.pass_real(p1.clone());
            },
            (Power::HandGun, NightAct::Two(p1, p2)) => {
                guns.pass_real(p1.clone());
                guns.pass_fake(p2.clone());
            },
            (Power::HandFakeGun, NightAct::One(p1)) => {
                guns.pass_fake(p1.clone());
            },
            (Power::HandFakeGun, NightAct::Two(p1, p2)) => {
                guns.pass_fake(p1.clone());
                guns.pass_fake(p2.clone());
            },
            _ => {}
        }
        Some(guns)
    }

    pub fn apply_night(&mut self, room: &impl Room, mut spells: impl Spells) -> Result<NightResult, Error> {
        let mut deads = BTreeSet::new();
        let mut msgs = BTreeMap::new();

        Self::remove_paralyzed_unguarded_spell(room, &mut spells);

        if let Some(killed) = self.remove_killed_one(room, &mut spells)? {
            deads.insert(killed);
        }

        self.gun = self.gunman(&spells, &mut msgs);
        self.detective(room, &mut msgs, &spells);

        let result = NightResult::new(msgs, deads);
        Ok(result)
    }   

    pub fn shoot(&mut self, shooter: &PlayerId, on: PlayerId) -> ShootingResult {
        use ShootingResult::*;
        if self.shotgun_done { 
            return NotAllowed
        }
        if let Some(ref mut events) = self.gun {
            return match events.try_shooting(shooter) {
                GunOwner::Real => {
                    self.shotgun_done = true;
                    Killed(on)
                },
                GunOwner::Fake => EmptyGun(on),
                GunOwner::Invalid => NotAllowed
            }
        }
        NotAllowed
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