
//! Types to process the world and entities.

pub use self::entitysystem::{EntitySystem, EntityProcess};
pub use self::entitysystem::{PassiveEntitySystem, PassiveEntityProcess};
pub use self::interactsystem::{InteractSystem, InteractProcess};

use EntityData;
use Entity;
use World;

pub mod entitysystem;
pub mod interactsystem;

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

/// System which operates every certain number of updates.
pub struct IntervalSystem<T: Active>
{
    interval: u8,
    ticker: u8,
    inner: T,
}

impl<T: Active> IntervalSystem<T>
{
    /// Create a new interval system with the specified number of updates between processes.
    pub fn new(system: T, interval: u8) -> IntervalSystem<T>
    {
        IntervalSystem
        {
            interval: interval,
            ticker: 0,
            inner: system,
        }
    }
}

impl<T: Active> Active for IntervalSystem<T>
{
    fn process(&mut self, c: &mut EntityData)
    {
        self.ticker += 1;
        if self.ticker == self.interval
        {
            self.ticker = 0;
            self.inner.process(c);
        }
    }
}

impl<T: Active> System for IntervalSystem<T>
{
    fn activated(&mut self, e: &Entity, w: &World)
    {
        self.inner.activated(e, w);
    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.inner.reactivated(e, w);
    }

    fn deactivated(&mut self, e: &Entity, w: &World)
    {
        self.inner.deactivated(e, w);
    }
}
