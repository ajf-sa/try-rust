pub struct R {
    pub name: String,
    pub age: u32,
}

impl R {
    pub fn new(name: String, age: u32) -> R {
        R {
            name: name,
            age: age,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
