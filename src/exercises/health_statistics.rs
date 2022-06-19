/**
 *  Your goal is to implement the stubbed out methods on the
 *  User struct defined in the impl block.
 *  For example, the new method should return an
 *  instance of the User struct with the specified
 *  name, age, and weight values.
 *
 *  Exercise link: https://exercism.org/tracks/rust/exercises/health-statistics
 */

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}
impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn weight(&self) -> f32 {
        self.weight
    }
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

pub fn main() {
    let mut user1 = User::new(String::from("Sr.User"), 99, 1.75);
}
