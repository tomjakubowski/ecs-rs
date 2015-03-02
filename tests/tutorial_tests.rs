// If these tests fail, make sure to update the corresponding code in doc/tutorial.md

#![forbid(warnings)]

#[macro_use]
extern crate ecs;

pub mod chapter2
{
    use ecs::World;

    components! {
        MyComponents;
    }

    systems! {
        MySystems<MyComponents>;
    }

    #[test]
    fn test() {
        let _ = World::<MyComponents, MySystems>::new();
    }
}

pub mod chapter3
{
    use ecs::World;
    use chapter2::{MyComponents, MySystems};

    #[test]
    fn test() {
        let mut world = World::<MyComponents, MySystems>::new();

        let entity = world.create_entity(Box::new(()));
        println!("{:?}", entity);
        world.remove_entity(entity);
        let entity2 = world.create_entity(Box::new(()));
        assert!(entity != entity2);
        println!("{:?}", entity2);
    }
}

pub mod chapter4
{
    use ecs::{BuildData, ModifyData};
    use ecs::World;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Position {
        x: f32,
        y: f32,
    }

    components! {
        MyComponents {
            #[hot] position: Position,
            #[cold] respawn: Position
        }
    }

    systems! {
        MySystems<MyComponents>;
    }

    #[test]
    fn test() {
        let mut world = World::<MyComponents, MySystems>::new();

        let entity = world.create_entity(Box::new(
            |entity: BuildData, data: &mut MyComponents| {
                data.position.add(&entity, Position { x: 0.0, y: 0.0 });
                data.respawn.add(&entity, Position { x: 0.0, y: 0.0 });
            }
        ));

        world.with_entity_data(&entity, |entity, data| {
            data.position[entity].x += 5.0;
            data.position[entity].y += 8.0;
        });

        world.modify_entity(entity, Box::new(
            |entity: ModifyData, data: &mut MyComponents| {
                data.respawn[entity].x -= 4.0;
                data.position[entity] = data.respawn[entity];
                data.respawn.remove(&entity);
                assert_eq!(data.respawn.get(&entity), None);
                data.respawn.insert(&entity, Position { x: 1.0, y: 2.0});
            }
        ));
    }
}
