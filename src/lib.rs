//! Entity Component System Library (ECS)
//!
//! For info about why an ECS may be beneficial, see some of these articles:
//!
//! - http://gameprogrammingpatterns.com/component.html
//! - http://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/
//! - http://www.gamedev.net/page/resources/_/technical/game-programming/understanding-component-entity-systems-r3013
//! - http://cowboyprogramming.com/2007/01/05/evolve-your-heirachy/
//!
//! There is a large variety of ways an ECS may work. This particular one is similar to
//! [Artemis](http://gamadu.com/artemis/).
//! Although this isn't a port to Rust, most functionality should be similar, and the
//! tutorials/manual there should be able to make up for the current lack of documentation [FIXME]
//!
//! Here's the basic structure:
//!
//! - An `Entity` is just an identifier. It contains no data or logic whatsoever.
//! - A `Component` is a piece of data (eg: Position, Velocity, Colour). While containing logic can
//! sometimes be useful, it's best practice to avoid it wherever possible.
//! - A `System` runs all the logic. Most of the time, it filters out entities based on their
//! components, and only runs it's logic on the entities it's interested in. These filters are
//! called `Aspect`s. Some systems ignore entities, and just apply logic to the world itself.
//! - An `Aspect` is a simple helper to filter entities based on their components.
//! - A `Manager` is simply an object that observes when an entity is
//! added/activated/deactivated/removed. They are used to help 'manage' the entities, rather than
//! define data or logic.
//! - The `World` organises all the above items together to make sure everything runs as it should.

#![crate_name = "ecs"]
#![comment = "Entity Component System Library"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(macro_rules, phase)]
#![unstable]

#[phase(plugin)]
extern crate lazy_static;
extern crate uuid;

pub use aspect::Aspect;
pub use component::{Component, ComponentId};
pub use entity::Entity;
pub use manager::{Manager, MutableManager};
pub use system::System;
pub use world::{Components, World};

pub mod buffer;

pub mod aspect;
pub mod component;
pub mod entity;
pub mod manager;
pub mod system;
pub mod world;

/// A silly type needed to call static trait functions.
#[experimental="This should not be needed when static methods and generics work properly"]
pub struct Phantom<T>;

#[macro_escape]
mod macros
{
    /// Macro used to create a component.
    ///
    /// Manually created components will probably not work.
    ///
    /// # Example
    ///
    /// ```ignore
    /// component!(
    ///     ID_Pos: Position {
    ///         x: uint,
    ///         y: uint
    ///     }
    /// )
    /// ```
    /// Expands to a component struct:
    ///
    /// ```ignore
    /// pub struct Position {
    ///     x: uint,
    ///     y: uint
    /// }
    /// ```
    /// And an identifier (lazy-evaluated static):
    ///
    /// ```ignore
    /// static ref ID_Pos: ComponentId = ...;
    /// ```
    ///
    /// `Component` is automatically implemented for the struct.
    #[macro_export]
    macro_rules! component(
        ($ID:ident : $Name:ident { $($field:ident : $ty:ty),+ }) =>
        (
            lazy_static! { // No point in recalculating this all the time.
                static ref $ID: ::ecs::ComponentId = {
                    ::std::hash::hash(&stringify!($ID $Name))
                };
            }

            #[deriving(Clone, Default, PartialEq, Show)]
            pub struct $Name
            {
                $(pub $field : $ty),+
            }

            impl Component for $Name
            {
                fn cid(_: ::ecs::Phantom<$Name>) -> ::ecs::ComponentId
                {
                    *$ID
                }
            }
        );
        ($ID: ident : $Name: ident) =>
        (
            lazy_static! { // No point in recalculating this all the time.
                static ref $ID: ::ecs::ComponentId = {
                    ::std::hash::hash(&stringify!($ID $Name))
                };
            }

            #[deriving(Clone, Default, PartialEq, Show)]
            pub struct $Name;

            impl Component for $Name
            {
                fn cid(_: ::ecs::Phantom<$Name>) -> ::ecs::ComponentId
                {
                    *$ID
                }
            }
        )
    )
}
