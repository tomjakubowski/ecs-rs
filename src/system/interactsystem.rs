
//! System to specifically deal with interactions between two types of entity.

use std::collections::HashSet;

use Aspect;
use EntityData;
use Entity;
use {Active, Passive, System};
use World;

pub trait InteractProcess: System
{
    fn process<'a, T: Iterator<&'a Entity>>(&self, T, T, &mut EntityData);
}

pub struct InteractSystem<T: InteractProcess>
{
    interested_a: HashSet<Entity>,
    interested_b: HashSet<Entity>,
    aspect_a: Aspect,
    aspect_b: Aspect,
    inner: T,
}

impl<T: InteractProcess> InteractSystem<T>
{
    pub fn new(inner: T, aspect_a: Aspect, aspect_b: Aspect) -> InteractSystem<T>
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

impl<T: InteractProcess> Active for InteractSystem<T>
{
    fn process(&mut self, c: &mut EntityData)
    {
        self.inner.process(self.interested_a.iter(), self.interested_b.iter(), c);
    }
}

impl<T: InteractProcess> System for InteractSystem<T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect_a.check(entity, world)
        {
            self.interested_a.insert(entity.clone());
            self.inner.activated(entity, world);
        }
        if self.aspect_b.check(entity, world)
        {
            self.interested_b.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested_a.contains(entity)
        {
            if self.aspect_a.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested_a.remove(entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect_a.check(entity, world)
        {
            self.interested_a.insert(entity.clone());
            self.inner.activated(entity, world);
        }
        if self.interested_b.contains(entity)
        {
            if self.aspect_b.check(entity, world)
            {
                self.inner.reactivated(entity, world);
            }
            else
            {
                self.interested_b.remove(entity);
                self.inner.deactivated(entity, world);
            }
        }
        else if self.aspect_b.check(entity, world)
        {
            self.interested_b.insert(entity.clone());
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested_a.remove(entity)
        {
            self.inner.deactivated(entity, world);
        }
        if self.interested_b.remove(entity)
        {
            self.inner.deactivated(entity, world);
        }
    }
}
