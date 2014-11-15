Tutorial 2: Entities
====================
Entities are the main part of an entity-component system, and in this
library, are one of the simplest parts. They are simply two identifiers:
An index, and a unique ID (specifically, a UUID).

The index is used internally to store entity information.
No two valid entities will have the same index at the same time, or in other
words, if two entities have the same index, one (if not both) has been
deleted and is no longer valid.  
Indexes are recycled when an entity is removed.

The unique identifier is used to check if two entities are truly equal.
While it is theoretically possible for two entities to accidentally be
assigned the same UUID, the player is more likely to be hit by an asteroid
<sup>_[citation needed]_</sup>, therefore making any potential bugs caused by
this behaviour redundant.  
In all seriousness, checking for uuid collisions is a
waste of precious processing time for an event that will probably not happen.

## Creation

Anyway, to create an entity you get a `World` object and call
`build_entity()` with an implementation of `EntityBuilder`. Don't worry too
much about entity builders for now. Just pass a unit (`()`) to create a blank
entity:
```rust
// let mut world = WorldBuilder::new().build();

let entity = world.build_entity(());
```
The entity type is simply the following:
```rust
pub struct Entity(uint, Uuid);
```
As mentioned before, no two valid entities will have the same index at the
same time. You can use this to store external data about the state at some
point in time. For data that may be gathered and operated on at different
times, you should be using the UUID to make sure you don't accidentally
associate an entity with data that belonged to the previous owner of its
index.

There are convenience methods to access the index or uuid
```rust
let index: uint = entity.get_index();
let uuid: Uuid = entity.get_id();
```
Or you can use tuple destructuring:
```rust
let Entity(index, uuid) = entity;
```
Finally, to obtain just the index, you can just dereference the entity:
```rust
let index: uint = *entity;
```
To check an entity is valid (ie: was created in the first place has not since
been deleted), call `World.is_valid()`
```
assert!(world.is_valid(&entity));
```

## Modification

Modification of an entity's existing components can and should be done with
systems, but sometimes that is impractical, or an entity needs a component
added or removed from it.

To modify an entity's components, call `World.modify_entity()` with
an implementation of `EntityModifier`. Once again, you don't need to worry
much about this until later. For consistency, the unit type also implements
`EntityModifier`, but using it is quite pointless.
```rust
world.modify_entity(entity, ());
```

## Removal

When you are finished with an entity (eg: when it has died), call
`World.delete_entity()` to free its index and remove it from any systems or
managers.
```rust
world.delete_entity(&entity);
```
You can check the entity was deleted with `!World.is_valid()`:
```rust
assert!(!world.is_valid(&entity));
```

<table style="width:100%">
<tr><td>[Previous Tutorial](../tutorial1.md)</td>
<td style="text-align:center">[Top](../tutorials.md)</td>
<td style="text-align:right">[Next Tutorial](../tutorial3.md)</td></tr>
</table>
