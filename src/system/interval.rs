
use Entity;
use EntityData;
use {Active, Passive, System};
use World;

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
