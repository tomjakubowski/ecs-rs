Tutorial 4: Basic Systems
=========================
The final important part of an entity component system is the systems.
Systems are what controls entities and manipulate their components.

Systems are usually only interested in entities with a certain selection of
components available. This means that even for very different entities, you
can avoid writing duplicate code for the few features they may have in common.

The role of systems is quite broad, and so is spread over a few tutorials.
Rather than explain the basics of a system, we'll start off using helper types
for the most common use case: Entity Logic.

The library includes a helper type for this: `EntitySystem`.

## EntitySystem

This is the most basic entity helper system. All you need is to specify which
components you require entities to have/not have, and what to do with the
entities that satisfy those requirements.

The constructor for `EntitySystem` looks like this:
```rust
pub fn new(T, Aspect) -> EntitySystem<T> // T: EntityProcess
```
Those are two types that we haven't come across yet. The `T: EntityProcess`
is an implementation of the `EntityProcess` trait, which defines what we do
with the entities that meet the requirements. The second argument, `Aspect`,
defines those requirements, and acts like a filter to pick out which activated
entities the system wants to process.

(The next tutorial has a closer look at `Aspect`s)

## EntityProcess

The `EntityProcess` trait looks like the following:
```rust
pub trait EntityProcess: System
{
    // This method is called each update cycle with an iterator of entities that fulfill the requirements.
    fn process(&self, EntityIter, &mut EntityData);
}
```
The only method you need to implement is `process()`, but if you need some
special behaviour on certain events, the other methods are available to be
overriden.

Here's a basic implementation that just prints the entity IDs.
```rust
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

impl System for PrintEntityID {} // Empty implementation, but there are some methods that may be overriden.
```
Of course, this is not a system by itself (well, it is a `System`, but it does not implement `Active` or `Passive`), and so cannot be registered to the world. We need to use this `EntityProcess` to construct an `EntitySystem`.
```rust
let process = PrintEntityID;
let system = EntitySystem::new(process, Aspect::nil());
```
`Aspect::nil()` returns an empty aspect. Or in other words, all entities meet
the requirements and will be accepted by the `EntitySystem`.

Now we can test that everything works:
```rust
let mut builder = WorldBuilder::new();
builder.register_system(box system);
let mut world = builder.build();

for _ in range(0i, 3)
{
    world.build_entity(());
}

world.update();
```
This code should print out all the entity IDs.

<table style="width:100%">
<tr>
<td style="text-align:left"><a href="tutorial3.md">Previous Tutorial</a></td>
<td style="text-align:center"><a href="tutorials.md">Top</a></td>
<td style="text-align:right"><a href="tutorial5.md">Next Tutorial</a></td>
</tr>
</table>
