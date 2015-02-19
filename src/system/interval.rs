
use ComponentManager;
use DataHelper;
use EntityData;
use {Process, System};

/// System which operates every certain number of updates.
pub struct IntervalSystem<U: ComponentManager, T: Process<U>>
{
    interval: u8,
    ticker: u8,
    inner: T,
}

impl<U: ComponentManager, T: Process<U>> IntervalSystem<U, T>
{
    /// Create a new interval system with the specified number of updates between processes.
    pub fn new(system: T, interval: u8) -> IntervalSystem<U, T>
    {
        IntervalSystem
        {
            interval: interval,
            ticker: 0,
            inner: system,
        }
    }
}

impl<U: ComponentManager, T: Process<U>> Process<U> for IntervalSystem<U, T>
{
    fn process(&mut self, c: &mut DataHelper<U>)
    {
        self.ticker += 1;
        if self.ticker == self.interval
        {
            self.ticker = 0;
            self.inner.process(c);
        }
    }
}

impl<U: ComponentManager, T: Process<U>> System<U> for IntervalSystem<U, T>
{
    fn activated(&mut self, e: &EntityData<U>, w: &U)
    {
        self.inner.activated(e, w);
    }

    fn reactivated(&mut self, e: &EntityData<U>, w: &U)
    {
        self.inner.reactivated(e, w);
    }

    fn deactivated(&mut self, e: &EntityData<U>, w: &U)
    {
        self.inner.deactivated(e, w);
    }

    fn is_active(&self) -> bool
    {
        self.inner.is_active()
    }
}
