
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
    fn process<'a>(&mut self, EntityIter<'a, <Self as System>::Components>, &mut DataHelper<<Self as System>::Components>);
}

pub struct EntitySystem<T: EntityProcess>
{
    interested: HashSet<Entity>,
    aspect: Aspect<<T as System>::Components>,
    pub inner: T,
}

impl<T: EntityProcess> EntitySystem<T>
{
    pub fn new(inner: T, aspect: Aspect<<T as System>::Components>) -> EntitySystem<T>
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
    type Components = <T as System>::Components;
    fn activated(&mut self, entity: &EntityData, world: &<T as System>::Components)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &EntityData, world: &<T as System>::Components)
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

    fn deactivated(&mut self, entity: &EntityData, world: &<T as System>::Components)
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
    fn process(&mut self, c: &mut DataHelper<<T as System>::Components>)
    {
        self.inner.process(EntityIter::new(self.interested.iter()), c);
    }
}
