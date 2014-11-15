
//! Systems to specifically deal with entities.

use std::collections::TrieMap;

use Aspect;
use EntityData;
use Entity;
use System;
use World;

pub trait EntityProcess: 'static
{
    fn process(&self, &Entity, &mut EntityData);

    fn preprocess(&mut self, _: &World)
    {

    }

    fn postprocess(&mut self, _: &World)
    {

    }

    fn activated(&mut self, _: &Entity, _: &World)
    {

    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.deactivated(e, w);
        self.activated(e, w);
    }

    fn deactivated(&mut self, _: &Entity, _: &World)
    {

    }
}

pub trait BulkEntityProcess: 'static
{
    fn process(&self, Vec<&Entity>, &mut EntityData);

    fn preprocess(&mut self, _: &World)
    {

    }

    fn postprocess(&mut self, _: &World)
    {

    }

    fn activated(&mut self, _: &Entity, _: &World)
    {

    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.deactivated(e, w);
        self.activated(e, w);
    }

    fn deactivated(&mut self, _: &Entity, _: &World)
    {

    }
}

/// Entity System that operates on all interested entities at once.
pub struct BulkEntitySystem
{
    interested: TrieMap<Entity>,
    aspect: Aspect,
    inner: Box<BulkEntityProcess>,
}

impl BulkEntitySystem
{
    /// Return a new entity system with the specified bulk process.
    pub fn new(inner: Box<BulkEntityProcess>, aspect: Aspect) -> BulkEntitySystem
    {
        BulkEntitySystem
        {
            interested: TrieMap::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl System for BulkEntitySystem
{
    fn process(&self, c: &mut EntityData)
    {
        self.inner.process(FromIterator::from_iter(self.interested.values()), c);
    }

    fn preprocess(&mut self, w: &World)
    {
        self.inner.preprocess(w);
    }

    fn postprocess(&mut self, w: &World)
    {
        self.inner.postprocess(w);
    }

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

/// Entity system that processes one entity at a time.
pub struct EntitySystem
{
    interested: TrieMap<Entity>,
    aspect: Aspect,
    inner: Box<EntityProcess>,
}

impl EntitySystem
{
    /// Return a new entity system with the specified process.
    pub fn new(inner: Box<EntityProcess>, aspect: Aspect) -> EntitySystem
    {
        EntitySystem
        {
            interested: TrieMap::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl System for EntitySystem
{
    fn process(&self, c: &mut EntityData)
    {
        for e in self.interested.values()
        {
            self.inner.process(e, c);
        }
    }

    fn preprocess(&mut self, w: &World)
    {
        self.inner.preprocess(w);
    }

    fn postprocess(&mut self, w: &World)
    {
        self.inner.postprocess(w);
    }

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
