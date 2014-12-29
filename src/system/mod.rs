
//! Types to process the world and entities.

pub use self::data::{DataIter, DataSystem, DataProcess};
pub use self::entity::{EntityIter, EntitySystem, EntityProcess};
pub use self::entity::{PassiveEntitySystem, PassiveEntityProcess};
pub use self::interact::{InteractSystem, InteractProcess};
pub use self::interval::{IntervalSystem};

use EntityData;
use Entity;
use World;

pub mod data;
pub mod entity;
pub mod interact;
pub mod interval;

/// Generic base system type.
pub trait System: 'static
{
    /// Optional method called when an entity is activated.
    fn activated(&mut self, _: &Entity, _: &World)
    {

    }

    /// Optional method called when an entity is reactivated.
    ///
    /// By default it calls deactivated() followed by activated()
    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.deactivated(e, w);
        self.activated(e, w);
    }

    /// Optional method called when an entity is deactivated.
    fn deactivated(&mut self, _: &Entity, _: &World)
    {

    }
}

/// Generic active system type.
pub trait Active: System
{
    /// Process the world.
    fn process(&mut self, &mut EntityData);
}

/// Generic passive system type.
pub trait Passive: System
{
    /// Process the world.
    fn process(&mut self, &World);
}
