// These tests are to make sure that the code in the tutorials is valid.
// If these break, make sure to fix the corresponding tutorials too.

#![feature(box_syntax)]

//#![deny(warnings)]
#![forbid(warnings)]

#[macro_use]
extern crate ecs;

#[test]
fn tutorial1()
{
    use ecs::{WorldBuilder};

    let builder = WorldBuilder::new();
    let mut world = builder.build();

    // Checks the world is actually finalised.
    world.update();
}

#[test]
fn tutorial2()
{
    use ecs::{WorldBuilder};

    let mut world = WorldBuilder::new().build();

    let entity = world.build_entity(());

    let index1 = entity.get_index();

    entity.get_id(); // Unused, but needs to be checked anyway.

    let index2 = *entity;
    assert_eq!(index1, index2);

    assert!(world.is_valid(&entity));

    world.modify_entity(entity, ());

    world.remove_entity(&entity);
    assert!(!world.is_valid(&entity));

    let entity2 = world.build_entity(());

    // Check that indexes are reassigned properly.
    assert_eq!(*entity, *entity2);
    // Check that unique ids are different.
    assert!(entity.get_id() != entity2.get_id());
}

mod tutorial3
{
    use ecs::{Components, Entity, WorldBuilder};

    component! {
        Position {
            x: f32,
            y: f32
        }

        Velocity {
            dx: f32,
            dy: f32
        }
    }

    new_type! {
        Team(i32);
        Experience(i32);
    }

    feature! {
        CanFly;
        IsPlayer;
    }

    #[test]
    fn tutorial3()
    {
        let team1 = Team(1);
        let team2 = Team(2);
        assert_eq!(*team1, 1);
        assert_eq!(*team2, 2);

        let mut builder = WorldBuilder::new();
        builder.register_component::<Position>();
        builder.register_component::<Velocity>();
        builder.register_component::<Team>();
        builder.register_component::<CanFly>();

        let position_id = component_id!(Position);
        let velocity_id = component_id!(Velocity);

        let mut world = builder.build();
        let entity = world.build_entity(
            |&: c: &mut Components, e: Entity| {
                c.add(&e, Position { x: 5.0, y: 2.0 });
                c.add(&e, Velocity { dx: 0.0, dy: 0.0 });
                c.add(&e, Team(1));
            }
        );

        assert!(world.has_component(&entity, position_id));
        assert!(world.has_component(&entity, velocity_id));

        assert_eq!(world.get_component::<Position>(&entity), Position { x: 5.0, y: 2.0 });
        assert!(world.try_component::<CanFly>(&entity).is_none());

        world.modify_entity(entity,
            |&: c: &mut Components, e: Entity| {
                c.add(&e, CanFly);
                c.set(&e, Team(2));
                c.remove::<Velocity>(&e);
            }
        );
        assert_eq!(Team(2), world.get_component(&entity));
        assert!(!world.has_component(&entity, velocity_id));
        assert!(world.has_component(&entity, component_id!(CanFly)));
    }
}

mod tutorial4
{
    use ecs::{Aspect, EntityData, System, WorldBuilder};
    use ecs::system::{EntityIter, EntityProcess, EntitySystem};

    pub struct PrintEntityID;

    impl EntityProcess for PrintEntityID
    {
        fn process(&self, mut entities: EntityIter, _: &mut EntityData)
        {
            for entity in entities
            {
                println!("Processed Entity: {}", entity.get_id());
            }
        }
    }

    impl System for PrintEntityID {}

    #[test]
    fn tutorial4()
    {
        let process = PrintEntityID;
        let system = EntitySystem::new(process, Aspect::nil());

        let mut builder = WorldBuilder::new();
        builder.register_system(box system);
        let mut world = builder.build();

        for _ in 0..3
        {
            world.build_entity(());
        }

        world.update();
    }
}
