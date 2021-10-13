struct Animal {
    pub name: String,
}

impl Animal {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

fn main() {
    let mut animal = Animal {
        name: String::from("cat"),
    };

    println!("this animal is {}", animal.get_name());

    animal.set_name(String::from("dog"));

    println!("this animal is {}", animal.get_name());
}
