
//! Types to process the world and entities.

pub use self::entitysystem::{BulkEntitySystem, BulkEntityProcess};
pub use self::entitysystem::{EntitySystem, EntityProcess};

use EntityData;
use Entity;
use World;

pub mod entitysystem;

/// Generic system type.
pub trait System: 'static
{
    /// Process the world.
    fn process(&self, &mut EntityData);

    /// Optional method called before processing.
    fn preprocess(&mut self, _: &World)
    {

    }

    /// Optional method called after proceessing.
    fn postprocess(&mut self, _: &World)
    {

    }

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

/// Generic passive system type.
pub trait Passive: 'static
{
    /// Process the world.
    fn process(&mut self, &World);

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

/// System which operates every certain number of updates.
pub struct IntervalSystem
{
    interval: u8,
    ticker: u8,
    inner: Box<System>,
}

impl IntervalSystem
{
    /// Create a new interval system with the specified number of updates between processes.
    pub fn new(system: Box<System>, interval: u8) -> IntervalSystem
    {
        IntervalSystem
        {
            interval: interval,
            ticker: 0,
            inner: system,
        }
    }
}

impl System for IntervalSystem
{
    fn process(&self, c: &mut EntityData)
    {
        if self.ticker == self.interval
        {
            self.inner.process(c);
        }
    }

    fn preprocess(&mut self, w: &World)
    {
        if self.ticker < self.interval
        {
            self.ticker += 1;
        }
        if self.ticker == self.interval
        {
            self.inner.preprocess(w);
        }
    }

    fn postprocess(&mut self, w: &World)
    {
        if self.ticker == self.interval
        {
            self.inner.postprocess(w);
            self.ticker = 0;
        }
    }

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
