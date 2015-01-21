
#![feature(box_syntax)]

#[macro_use]
extern crate ecs;

/// Caused a panic when trying to access zero-sized types.
mod issue_12
{
    use ecs::WorldBuilder;
    use ecs::Components;
    use ecs::Entity;

    feature! {
        CanFly;
    }

    #[test]
    fn test()
    {
        let mut builder = WorldBuilder::new();
        builder.register_component::<CanFly>();

        let mut world = builder.build();

        let entity = world.build_entity(
            |&: c: &mut Components, e: Entity| {
                c.add(&e, CanFly);
            }
        );

        match world.try_component::<CanFly>(&entity) {
            Some(_) => println!("can fly"),
            None => println!("can't fly"),
        }
    }
}
