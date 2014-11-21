
//! Systems to specifically deal with entities.

use std::collections::TrieMap;

use Aspect;
use EntityData;
use Entity;
use {Active, Passive, System};
use World;

pub trait EntityProcess: System
{
    fn process<'a, T: Iterator<&'a Entity>>(&self, T, &mut EntityData);
}

pub trait PassiveEntityProcess: System
{
    fn process<'a, T: Iterator<&'a Entity>>(&mut self, T, &World);
}

pub struct EntitySystem<T: EntityProcess>
{
    interested: TrieMap<Entity>,
    aspect: Aspect,
    inner: T,
}

pub struct PassiveEntitySystem<T: PassiveEntityProcess>
{
    interested: TrieMap<Entity>,
    aspect: Aspect,
    inner: T,
}

impl<T: EntityProcess> EntitySystem<T>
{
    pub fn new(inner: T, aspect: Aspect) -> EntitySystem<T>
    {
        EntitySystem
        {
            interested: TrieMap::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<T: EntityProcess> Active for EntitySystem<T>
{
    fn process(&mut self, c: &mut EntityData)
    {
        self.inner.process(self.interested.values(), c);
    }
}

impl<T: EntityProcess> System for EntitySystem<T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity, *entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.contains_key(&**entity)
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
            self.interested.insert(**entity, *entity);
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.remove(&**entity).is_some()
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
            interested: TrieMap::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<T: PassiveEntityProcess> Passive for PassiveEntitySystem<T>
{
    fn process(&mut self, c: &World)
    {
        self.inner.process(self.interested.values(), c);
    }
}

impl<T: PassiveEntityProcess> System for PassiveEntitySystem<T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity, *entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.contains_key(&**entity)
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
            self.interested.insert(**entity, *entity);
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.remove(&**entity).is_some()
        {
            self.inner.deactivated(entity, world);
        }
    }
}
