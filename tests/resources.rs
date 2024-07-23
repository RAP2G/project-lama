use project_lama::World;
use std::ops::Deref;
#[test]
fn create_and_get_resource_immutably() {
    let mut world = World::new();
    world.add_resource(Fps(60));
    let fps = world.get_resource::<Fps>();
    assert_eq!(fps.unwrap().0, 60)
}

#[test]
fn get_resource_mutably() {
    let mut world = World::new();
    world.add_resource(Fps(60));
    {
        let fps: &mut Fps = world.get_resource_mut::<Fps>().unwrap();
        fps.0 += 1;
    }
    let fps = world.get_resource::<Fps>().unwrap();
    assert_eq!(fps.0, 61)
}

#[test]
// #[should_panic]
fn remove_resource() {
    let mut world = World::new();
    world.add_resource(Fps(60));
    world.remove_resource::<Fps>();
    let deleted_resource = world.get_resource::<Fps>();
    assert!(deleted_resource.is_none());
}

struct Fps(pub u32);

impl Deref for Fps {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
