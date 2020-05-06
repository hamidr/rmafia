extern crate nanoid;

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
pub struct Id(String);

impl Id {
    pub fn unique_random_id() -> Id {
        unimplemented!()
    }
}
