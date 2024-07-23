use std::any::Any;

use resource::Resource;
mod entities;
mod resource;
#[derive(Default, Debug)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
    ///Adds a resource to the
    pub fn add_resource(&mut self, data: impl Any) {
        self.resources.add(data);
    }
    ///Returns an immutable reference to the queried resource. The type of the resource mut be provided
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
    ///Returns a mutable reference to the queried resource. The type of the resource mut be provided
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
    pub fn remove_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }
}
