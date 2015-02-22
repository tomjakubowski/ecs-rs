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
    use ecs::BuildData;
    use ecs::World;

    #[derive(Copy, Clone, Debug)]
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

        let _ = world.create_entity(Box::new(
            |entity: BuildData<_>, data: &mut MyComponents| {
                entity.insert(&mut data.position, Position { x: 0.0, y: 0.0 });
                entity.insert(&mut data.respawn, Position { x: 0.0, y: 0.0 });
            }
        ));
    }
}
