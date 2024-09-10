pub struct Person {
    pub age: i16,
}

impl Person {
    pub fn get_older(&mut self, by_how_much: i16) {
        self.age += by_how_much;
    }
}