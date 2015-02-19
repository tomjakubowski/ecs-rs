
#![feature(box_syntax)]

#[macro_use]
extern crate ecs;

use ecs::Aspect;
use ecs::{BuildData, ModifyData};
use ecs::{World, DataHelper};
use ecs::{Process, System};
use ecs::system::{EntityProcess, EntitySystem};
use ecs::EntityIter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position
{
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Team(u8);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SomeFeature;

components! {
    TestComponents {
        #[hot] position: Position,
        #[cold] team: Team,
        #[hot] feature: SomeFeature
    }
}

systems! {
    TestSystems<TestComponents> {
        hello_world: HelloWorld = HelloWorld("Hello, World!"),
        print_position: EntitySystem<TestComponents, PrintPosition> = EntitySystem::new(PrintPosition,
                aspect!(<TestComponents>
                    all: [position, feature]
                )
            )
    }
}

pub struct HelloWorld(&'static str);
impl Process<TestComponents> for HelloWorld
{
    fn process(&mut self, _: &mut DataHelper<TestComponents>)
    {
        println!("{}", self.0);
    }
}
impl System<TestComponents> for HelloWorld {}

pub struct PrintPosition;
impl EntityProcess<TestComponents> for PrintPosition
{
    fn process(&mut self, en: EntityIter<TestComponents>, co: &mut DataHelper<TestComponents>)
    {
        for e in en
        {
            println!("{:?}", e.borrow(&mut co.position));
        }
    }
}
impl System<TestComponents> for PrintPosition {
    fn is_active(&self) -> bool { false }
}

#[test]
fn test_general_1()
{
    let mut world = World::<TestComponents, TestSystems>::new();

    // Test entity builders
    let entity = world.create_entity(box |e: BuildData<_>, c: &mut TestComponents| {
        e.insert(&mut c.position, Position { x: 0.5, y: 0.7 });
        e.insert(&mut c.team, Team(4));
    });
    world.create_entity(box |e: BuildData<_>, c: &mut TestComponents| {
        e.insert(&mut c.position, Position { x: 0.6, y: 0.8 });
        e.insert(&mut c.team, Team(3));
        e.insert(&mut c.feature, SomeFeature);
    });

    // Test passive systems
    world.systems.print_position.process(&mut world.data);

    // Test entity modifiers
    world.modify_entity(entity, box |e: ModifyData<_>, c: &mut TestComponents| {
        assert_eq!(Some(Position { x: 0.5, y: 0.7 }), e.insert(&mut c.position, Position { x: -2.5, y: 7.6 }));
        assert_eq!(Some(Team(4)), e.remove(&mut c.team));
        assert!(!e.has(&mut c.feature));
        assert!(e.insert(&mut c.feature, SomeFeature).is_none());
    });
    world.systems.print_position.process(&mut world.data);
    world.modify_entity(entity, box |e: ModifyData<_>, c: &mut TestComponents| {
        assert_eq!(Some(Position { x: -2.5, y: 7.6 }), e.get(&mut c.position));
        assert_eq!(None, e.remove(&mut c.team));
        assert!(e.insert(&mut c.feature, SomeFeature).is_some());
    });

    // Test external entity iterator
    for e in world.entities()
    {
        assert!(e.has(&world.position));
    }

    // Test external entity iterator with aspect filtering
    for e in world.entities().filter(aspect!(<TestComponents> all: [team]), &world)
    {
        assert!(e.has(&world.team));
    }

    // Test active systems
    world.update();

    // Test system modification
    world.systems.hello_world.0 = "Goodbye, World!";
    world.update();
}
