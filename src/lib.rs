use std::any::Any;

use resource::Resource;
mod resource;
#[derive(Default, Debug)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, data: impl Any) {
        self.resources.add(data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
}
