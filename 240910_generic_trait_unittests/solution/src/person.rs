use crate::company::{CanGetOlder, CanSpeak};

pub struct Person {
    pub age: i16,
}

impl CanGetOlder for Person {
    fn get_older(&mut self, by_how_much: i16) {
        self.age += by_how_much;
    }
}

impl CanSpeak for Person {}