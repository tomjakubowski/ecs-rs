
//! Systems to specifically deal with entities.

use std::collections::TrieMap;

use Aspect;
use EntityData;
use Entity;
use {Active, System};
use World;

pub trait EntityProcess: System
{
    fn process<'a, T: Iterator<&'a Entity>>(&self, T, &mut EntityData);
}

/// Entity System that operates on all interested entities at once.
pub struct EntitySystem<T: EntityProcess>
{
    interested: TrieMap<Entity>,
    aspect: Aspect,
    inner: T,
}

impl<T: EntityProcess> EntitySystem<T>
{
    /// Return a new entity system with the specified bulk process.
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
