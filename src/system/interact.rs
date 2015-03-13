
//! System to specifically deal with interactions between two types of entity.

use std::collections::HashSet;

use Aspect;
use DataHelper;
use Entity;
use EntityData;
use EntityIter;
use {Process, System};

pub trait InteractProcess: System
{
    fn process<'a>(&self, EntityIter<'a, Self::Components>, EntityIter<'a, Self::Components>, &mut DataHelper<Self::Components, Self::Services>);
}

pub struct InteractSystem<T: InteractProcess>
{
    interested_a: HashSet<Entity>,
    interested_b: HashSet<Entity>,
    aspect_a: Aspect<T::Components>,
    aspect_b: Aspect<T::Components>,
    inner: T,
}

impl<T: InteractProcess> InteractSystem<T>
{
    pub fn new(inner: T, aspect_a: Aspect<T::Components>, aspect_b: Aspect<T::Components>) -> InteractSystem<T>
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

impl<T: InteractProcess> System for InteractSystem<T>
{
    type Components = T::Components;
    type Services = T::Services;
    fn activated(&mut self, entity: &EntityData, world: &T::Components)
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

    fn reactivated(&mut self, entity: &EntityData, world: &T::Components)
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

    fn deactivated(&mut self, entity: &EntityData, world: &T::Components)
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

impl<T: InteractProcess> Process for InteractSystem<T>
{
    fn process(&mut self, c: &mut DataHelper<T::Components, T::Services>)
    {
        self.inner.process(EntityIter::new(self.interested_a.iter()), EntityIter::new(self.interested_b.iter()), c);
    }
}
