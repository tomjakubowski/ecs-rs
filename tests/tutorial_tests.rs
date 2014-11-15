// These tests are to make sure that the code in the tutorials is valid.
// If these break, make sure to fix the corresponding tutorials too.

#![deny(warnings)]
//#![forbid(warnings)]

#![feature(if_let, phase)]

#[phase(plugin, link)]
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

    world.delete_entity(&entity);
    assert!(!world.is_valid(&entity));

    let entity2 = world.build_entity(());

    // Check that indexes are reassigned properly.
    assert_eq!(*entity, *entity2);
    // Check that uuids are different.
    assert!(entity.get_id() != entity2.get_id());
}

#[allow(warnings)]
mod tutorial3
{
    use ecs::{Components, Entity, World, WorldBuilder};

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
        Team(int);
        Experience(int);
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
            |c: &mut Components, e: Entity| {
                c.add(&e, Position { x: 5.0, y: 2.0 });
                c.add(&e, Velocity { dx: 0.0, dy: 0.0 });
                c.add(&e, Team(1));
            }
        );

        assert!(world.has_component(&entity, position_id));
        assert!(world.has_component(&entity, velocity_id));

        assert!(world.get_component::<CanFly>(&entity).is_none());
        if let Some(pos) = world.get_component::<Position>(&entity) {
            assert_eq!(pos, Position { x: 5.0, y: 2.0 });
        } else {
            panic!("No Position Component")
        }

        world.modify_entity(entity,
            |c: &mut Components, e: Entity| {
                c.add(&e, CanFly);
                c.set(&e, Team(2));
                c.remove::<Velocity>(&e);
            }
        );
        assert_eq!(Team(2), world.get_component(&entity).unwrap());
        assert!(!world.has_component(&entity, velocity_id));
        assert!(world.has_component(&entity, component_id!(CanFly)));
    }
}
