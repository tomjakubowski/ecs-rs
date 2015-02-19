
//! Systems to specifically deal with entities.

use std::collections::HashSet;

use Aspect;
use ComponentManager;
use DataHelper;
use Entity;
use EntityData;
use EntityIter;
use {System, Process};

pub trait EntityProcess<T: ComponentManager>: System<T>
{
    fn process<'a>(&mut self, EntityIter<'a, T>, &mut DataHelper<T>);
}

pub struct EntitySystem<U: ComponentManager, T: EntityProcess<U>>
{
    interested: HashSet<Entity>,
    aspect: Aspect<U>,
    pub inner: T,
}

impl<U: ComponentManager, T: EntityProcess<U>> EntitySystem<U, T>
{
    pub fn new(inner: T, aspect: Aspect<U>) -> EntitySystem<U, T>
    {
        EntitySystem
        {
            interested: HashSet::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<U: ComponentManager, T: EntityProcess<U>> System<U> for EntitySystem<U, T>
{
    fn activated(&mut self, entity: &EntityData<U>, world: &U)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(**entity);
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &EntityData<U>, world: &U)
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

    fn deactivated(&mut self, entity: &EntityData<U>, world: &U)
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

impl<U: ComponentManager, T: EntityProcess<U>> Process<U> for EntitySystem<U, T>
{
    fn process(&mut self, c: &mut DataHelper<U>)
    {
        self.inner.process(EntityIter::new(self.interested.iter()), c);
    }
}
