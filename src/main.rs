use std::collections::BTreeMap;

use scenario::Scenario;

use crate::{games::classic::game::Classic, waiting::{WaitingBuilder, WaitingRoom}};

extern crate nanoid;
mod scenario;
mod room;
mod games;
mod elections;
mod waiting;
mod in_memory_room;
mod oracle;

fn main() -> Result<(), String> {
    let mut waiting_room = WaitingBuilder::new(10);
    let mut ps = BTreeMap::new();
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));
    waiting_room.reserve().and_then(|(id, c)| ps.insert(id, c));

    ps.get_mut(&1).unwrap().tell(todo!());
    let room = waiting_room.get().unwrap();
    let mut game = Classic::new(room)?;
    let state = game.state();
    
    println!("Hello, world!");
    Ok(())
}
