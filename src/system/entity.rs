
//! Systems to specifically deal with entities.

use std::collections::hash_set::{HashSet, Iter};

use Aspect;
use EntityData;
use Entity;
use {Active, Passive, System};
use World;

pub struct EntityIter<'a>
{
    inner: Iter<'a, Entity>,
}

impl<'a> Deref<Iter<'a, Entity>> for EntityIter<'a>
{
    fn deref(&self) -> &Iter<'a, Entity>
    {
        &self.inner
    }
}

impl<'a> Iterator<&'a Entity> for EntityIter<'a>
{
    fn next(&mut self) -> Option<&'a Entity>
    {
        self.inner.next()
    }
}

pub trait EntityProcess: System
{
    fn process<'a>(&self, EntityIter<'a>, &mut EntityData);
}

pub trait PassiveEntityProcess: System
{
    fn process<'a>(&mut self, EntityIter<'a>, &World);
}

pub struct EntitySystem<T: EntityProcess>
{
    interested: HashSet<Entity>,
    aspect: Aspect,
    inner: T,
}

pub struct PassiveEntitySystem<T: PassiveEntityProcess>
{
    interested: HashSet<Entity>,
    aspect: Aspect,
    inner: T,
}

impl<T: EntityProcess> EntitySystem<T>
{
    pub fn new(inner: T, aspect: Aspect) -> EntitySystem<T>
    {
        EntitySystem
        {
            interested: HashSet::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<T: EntityProcess> Active for EntitySystem<T>
{
    fn process(&mut self, c: &mut EntityData)
    {
        self.inner.process(EntityIter { inner: self.interested.iter() }, c);
    }
}

impl<T: EntityProcess> System for EntitySystem<T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.contains(entity)
        {
            if self.aspect.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested.remove(entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect.check(entity, world)
        {
            self.interested.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.remove(entity)
        {
            self.inner.deactivated(entity, world);
        }
    }
}

impl<T: PassiveEntityProcess> PassiveEntitySystem<T>
{
    pub fn new(inner: T, aspect: Aspect) -> PassiveEntitySystem<T>
    {
        PassiveEntitySystem
        {
            interested: HashSet::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<T: PassiveEntityProcess> Passive for PassiveEntitySystem<T>
{
    fn process(&mut self, c: &World)
    {
        self.inner.process(EntityIter { inner: self.interested.iter() }, c);
    }
}

impl<T: PassiveEntityProcess> System for PassiveEntitySystem<T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.contains(entity)
        {
            if self.aspect.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested.remove(entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect.check(entity, world)
        {
            self.interested.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.remove(entity)
        {
            self.inner.deactivated(entity, world);
        }
    }
}
