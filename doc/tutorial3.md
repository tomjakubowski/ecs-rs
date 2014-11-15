Tutorial 3: Components
======================
The next important part of an entity component system is the component part.
A component is just pure data. Ideally, it should contain no logic. Of course,
using basic helper methods to extract or change data are usually fine.

The `Component` trait is automatically derived for valid component types.
Valid components must fulfill `Copy`, and `'static`. Further requirements may
be added if new library functions require it (eg: serialization). However, any
new requirements should be derivable traits.

## Defining Components

The best way to define new components is through the `component!` macro.
```rust
// #[phase(plugin, link)]
// extern crate ecs;

component! {
    Position {
        x: f32,
        y: f32 // Due to macro parsing problems, trailing commas do not work.
    }

    Velocity {
        dx: f32,
        dy: f32
    }

    // etc.
}
```
There is also a special macro for newtype components:
```rust
new_type! {
    Team(int);
    Experience(int);
    // etc.
}
```
Using this macro has the advantage that Deref is automatically implemented,
and so the single field can be easily accessed:
```rust
let team1 = Team(1);
let team2 = Team(2);
assert_eq!(*team1, 1);
assert_eq!(*team2, 2);
```
Finally, there is yet another macro for components that contains no data (ie:
flags). These unit components are called 'features'.
```rust
feature! {
    CanFly;
    IsPlayer; // There are better ways of sorting players. More info later.
    // etc.
}
```
As of this writing, all the above macros auto-derive the following traits for
convenience:
_(This list will probably change as new features are needed)_
- Default
- PartialEq
- Show

## Registering Components

A component cannot be used unless it is registered by the `World` object. This
is done through the `WorldBuilder` during setup.
```rust
// let mut builder = WorldBuilder::new();
builder.register_component::<Position>();
builder.register_component::<Velocity>();
builder.register_component::<Team>();
// etc.
```

## Getting a Component's ComponentId

Components are distinguished by their ComponentId (which is really just taken
from their TypeID. See std::intrinsics::TypeId). To easily obtain this, use
the `component_id!` macro:
```rust
let position_id = component_id!(Position);
let velocity_id = component_id!(Velocity);
```

## Adding Components to Entities

Adding a component to an entity is done through an `EntityBuilder` using the
`Components` interface. To save time, `EntityBuilder` is already implemented
for the closure type `|&mut Components, Entity|`, so we just need to write a
closure.
```rust
// let mut world = builder.build();
let entity = world.build_entity(
    |c: &mut Components, e: Entity| {
        c.add(&e, Position { x: 5.0, y: 2.0 });
        c.add(&e, Velocity { dx: 0.0, dy: 0.0 });
        c.add(&e, Team(1));
    }
);
```
This will create an entity located at `(5.0, 2.0)`, not moving, on team 1.
It does not have any experience element, nor does it have the `CanFly` or
`IsPlayer` features enabled.

## Getting an Entity's Components

There are two methods in `World` related to getting an entity's components.

The first is `World.has_component(&Entity, ComponentId)`, which is just a
check to see if an entity has a component.
```rust
assert!(world.has_component(&entity, position_id));
```
The second is `World.get_component::<T>(&Entity)`, which returns an
`Option<T>`, where `T: Component`.
```rust
assert!(world.get_component::<CanFly>(&entity).is_none());
if let Some(pos) = world.get_component::<Position>(&entity) {
    assert_eq!(pos, Position { x: 5.0, y: 2.0 });
}
```

## Modifying an Entity's Components

Changing an entity's components is done through `EntityModifier`s. These are
used almost exactly the same as `EntityBuilder`s. Once again, `EntityModifier`
is already implemented for `|&mut Components, Entity|`.
```rust
world.modify_entity(entity,
    |c: &mut Components, e: Entity| {
        c.add(&e, CanFly);
        c.set(&e, Team(2));
        c.remove::<Velocity>(&e);
    }
);
```
This will change the entity to team 2, give it the `CanFly` feature, and
remove the velocity component.


<table style="width:100%">
<tr>
<td style="text-align:left"><a href="tutorial2.md">Previous Tutorial</a></td>
<td style="text-align:center"><a href="tutorials.md">Top</a></td>
<td style="text-align:right"><a href="tutorial4.md">Next Tutorial</a></td>
</tr>
</table>
