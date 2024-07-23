use project_lama::World;
use std::ops::Deref;
#[test]
fn create_and_get_resource_immutably() {
    let mut world = World::new();
    world.add_resource(Fps(60));
    let fps = world.get_resource::<Fps>();
    assert_eq!(fps.unwrap().0, 60)
}

struct Fps(pub u32);

impl Deref for Fps {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
