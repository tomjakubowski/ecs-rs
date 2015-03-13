
#[macro_use]
extern crate ecs;

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
        #[hot] blank_data: (),
        #[hot] position: Position,
        #[cold] team: Team,
        #[hot] feature: SomeFeature
    }
}

systems! {
    TestSystems<TestComponents, ()> {
        hello_world: HelloWorld = HelloWorld("Hello, World!"),
        print_position: EntitySystem<PrintPosition> = EntitySystem::new(PrintPosition,
                aspect!(<TestComponents>
                    all: [position, feature]
                )
            )
    }
}

pub struct HelloWorld(&'static str);
impl Process for HelloWorld
{
    fn process(&mut self, _: &mut DataHelper<TestComponents, ()>)
    {
        println!("{}", self.0);
    }
}
impl System for HelloWorld { type Components = TestComponents; type Services = (); }

pub struct PrintPosition;
impl EntityProcess for PrintPosition
{
    fn process(&mut self, en: EntityIter<TestComponents>, co: &mut DataHelper<TestComponents, ()>)
    {
        for e in en
        {
            println!("{:?}", co.position.borrow(&e));
        }
    }
}
impl System for PrintPosition {
    type Components = TestComponents;
    type Services = ();
    fn is_active(&self) -> bool { false }
}

#[test]
fn test_general_1()
{
    let mut world = World::<TestSystems>::new();

    // Test entity builders
    let entity = world.create_entity(|e: BuildData<TestComponents>, c: &mut TestComponents| {
        c.position.add(&e, Position { x: 0.5, y: 0.7 });
        c.team.add(&e, Team(4));
    });
    world.create_entity(|e: BuildData<TestComponents>, c: &mut TestComponents| {
        c.position.add(&e, Position { x: 0.6, y: 0.8 });
        c.team.add(&e, Team(3));
        c.feature.add(&e, SomeFeature);
    });

    // Test passive systems
    world.systems.print_position.process(&mut world.data);

    // Test entity modifiers
    world.modify_entity(entity, |e: ModifyData<TestComponents>, c: &mut TestComponents| {
        assert_eq!(Some(Position { x: 0.5, y: 0.7 }), c.position.insert(&e, Position { x: -2.5, y: 7.6 }));
        assert_eq!(Some(Team(4)), c.team.remove(&e));
        assert!(!c.feature.has(&e));
        assert!(c.feature.insert(&e, SomeFeature).is_none());
    });
    process!(world, print_position);
    world.modify_entity(entity, |e: ModifyData<TestComponents>, c: &mut TestComponents| {
        assert_eq!(Position { x: -2.5, y: 7.6 }, c.position[e]);
        assert_eq!(None, c.team.remove(&e));
        assert!(c.feature.insert(&e, SomeFeature).is_some());
    });

    // Test external entity iterator
    for e in world.entities()
    {
        assert!(world.position.has(&e));
    }

    // Test external entity iterator with aspect filtering
    for e in world.entities().filter(aspect!(<TestComponents> all: [team]), &world)
    {
        assert!(world.team.has(&e));
    }

    // Test active systems
    world.update();

    // Test system modification
    world.systems.hello_world.0 = "Goodbye, World!";
    world.update();
}
