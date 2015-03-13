
//! Systems to specifically deal with entities.

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use Aspect;
use DataHelper;
use Entity;
use EntityData;
use EntityIter;
use {System, Process};

pub trait EntityProcess: System
{
    fn process<'a>(&mut self, EntityIter<'a, Self::Components>, &mut DataHelper<Self::Components, Self::Services>);
}

pub struct EntitySystem<T: EntityProcess>
{
    interested: HashSet<Entity>,
    aspect: Aspect<T::Components>,
    pub inner: T,
}

impl<T: EntityProcess> EntitySystem<T>
{
    pub fn new(inner: T, aspect: Aspect<T::Components>) -> EntitySystem<T>
    {
        EntitySystem
        {
            interested: HashSet::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<T: EntityProcess> Deref for EntitySystem<T>
{
    type Target = T;
    fn deref(&self) -> &T
    {
        &self.inner
    }
}

impl<T: EntityProcess> DerefMut for EntitySystem<T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        &mut self.inner
    }
}

impl<T: EntityProcess> System for EntitySystem<T>
{
    type Components = T::Components;
    type Services = T::Services;
    fn activated(&mut self, entity: &EntityData, world: &T::Components)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &EntityData, world: &T::Components)
    {
        if self.interested.contains(&**entity)
        {
            if self.aspect.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested.remove(&**entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &EntityData, world: &T::Components)
    {
        if self.interested.remove(&**entity)
        {
            self.inner.deactivated(entity, world);
        }
    }

    fn is_active(&self) -> bool
    {
        self.inner.is_active()
    }
}

impl<T: EntityProcess> Process for EntitySystem<T>
{
    fn process(&mut self, c: &mut DataHelper<T::Components, T::Services>)
    {
        self.inner.process(EntityIter::new(self.interested.iter()), c);
    }
}
