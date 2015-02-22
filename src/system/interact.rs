
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
    fn process<'a>(&self, EntityIter<'a, <Self as System>::Components>, EntityIter<'a, <Self as System>::Components>, &mut DataHelper<<Self as System>::Components>);
}

pub struct InteractSystem<T: InteractProcess>
{
    interested_a: HashSet<Entity>,
    interested_b: HashSet<Entity>,
    aspect_a: Aspect<<T as System>::Components>,
    aspect_b: Aspect<<T as System>::Components>,
    inner: T,
}

impl<T: InteractProcess> InteractSystem<T>
{
    pub fn new(inner: T, aspect_a: Aspect<<T as System>::Components>, aspect_b: Aspect<<T as System>::Components>) -> InteractSystem<T>
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
    type Components = <T as System>::Components;
    fn activated(&mut self, entity: &EntityData<<T as System>::Components>, world: &<T as System>::Components)
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

    fn reactivated(&mut self, entity: &EntityData<<T as System>::Components>, world: &<T as System>::Components)
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

    fn deactivated(&mut self, entity: &EntityData<<T as System>::Components>, world: &<T as System>::Components)
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
    fn process(&mut self, c: &mut DataHelper<<T as System>::Components>)
    {
        self.inner.process(EntityIter::new(self.interested_a.iter()), EntityIter::new(self.interested_b.iter()), c);
    }
}
