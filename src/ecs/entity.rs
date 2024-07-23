pub struct Entity {
    identifier: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { identifier: id }
    }
    pub fn get_id(&self) -> u32 {
        self.identifier
    }
}
