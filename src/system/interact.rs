
//! System to specifically deal with interactions between two types of entity.

use std::collections::HashSet;

use Aspect;
use ComponentManager;
use DataHelper;
use Entity;
use EntityData;
use EntityIter;
use {Process, System};

pub trait InteractProcess<T: ComponentManager>: System<T>
{
    fn process<'a>(&self, EntityIter<'a, T>, EntityIter<'a, T>, &mut DataHelper<T>);
}

pub struct InteractSystem<U: ComponentManager, T: InteractProcess<U>>
{
    interested_a: HashSet<Entity>,
    interested_b: HashSet<Entity>,
    aspect_a: Aspect<U>,
    aspect_b: Aspect<U>,
    inner: T,
}

impl<U: ComponentManager, T: InteractProcess<U>> InteractSystem<U, T>
{
    pub fn new(inner: T, aspect_a: Aspect<U>, aspect_b: Aspect<U>) -> InteractSystem<U, T>
    {
        InteractSystem
        {
            interested_a: HashSet::new(),
            interested_b: HashSet::new(),
            aspect_a: aspect_a,
            aspect_b: aspect_b,
            inner: inner,
        }
    }
}

impl<U: ComponentManager, T: InteractProcess<U>> System<U> for InteractSystem<U, T>
{
    fn activated(&mut self, entity: &EntityData<U>, world: &U)
    {
        if self.aspect_a.check(entity, world)
        {
            self.interested_a.insert(**entity);
            self.inner.activated(entity, world);
        }
        if self.aspect_b.check(entity, world)
        {
            self.interested_b.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &EntityData<U>, world: &U)
    {
        if self.interested_a.contains(&**entity)
        {
            if self.aspect_a.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested_a.remove(&**entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect_a.check(entity, world)
        {
            self.interested_a.insert(**entity);
            self.inner.activated(entity, world);
        }
        if self.interested_b.contains(&**entity)
        {
            if self.aspect_b.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested_b.remove(&**entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect_b.check(entity, world)
        {
            self.interested_b.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &EntityData<U>, world: &U)
    {
        if self.interested_a.remove(&**entity)
        {
            self.inner.deactivated(entity, world);
        }
        if self.interested_b.remove(&**entity)
        {
            self.inner.deactivated(entity, world);
        }
    }

    fn is_active(&self) -> bool
    {
        self.inner.is_active()
    }
}

impl<U: ComponentManager, T: InteractProcess<U>> Process<U> for InteractSystem<U, T>
{
    fn process(&mut self, c: &mut DataHelper<U>)
    {
        self.inner.process(EntityIter::new(self.interested_a.iter()), EntityIter::new(self.interested_b.iter()), c);
    }
}
