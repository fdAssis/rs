struct Dog {
    name: String,
    age: u32,
    owner: String,
}

impl ToString for Dog {
    fn to_string(&self) -> String {
        format!(
            "{} é um cachorro de {} anos que pertence a {}.",
            self.name, self.age, self.owner
        )
    }
}
