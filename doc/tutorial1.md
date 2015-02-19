## ecs-rs has just gone through a big refactoring and docs will remain out of date until dust has settled.

Tutorial 1: The World
=====================
The `World` is the top level object in the entity-component system.
It sorts all the entities, stores the components, processes all the systems,
and coordinates any other managers.

The first step to creating a `World` object is to create a `WorldBuilder`.
```rust
let mut builder = WorldBuilder::new();
```
The WorldBuilder is needed so the compiler can enforce that no
components/systems/managers are added to the world while it is operation.
This however, will be covered in later tutorials.

To finish building the world and allow operation, call `WorldBuilder.build()`:
```rust
let mut world = builder.build();
```
This consumes the world builder and returns a `World` object.

Usage of the world is covered in later tutorials.

There is no code in the world that needs any special destructors, so there is
no function needed to call to cleanup. Simply dropping the world is enough.
If you need to run any functionality before the world is destroyed, you should
use a passive system (more on that later).


<table style="width:100%">
<tr>
<td style="text-align:center"><a href="tutorials.md">Top</a></td>
<td style="text-align:right"><a href="tutorial2.md">Next Tutorial</a></td>
</tr>
</table>
