
//! System to specifically deal with interactions between two types of entity.

use std::collections::HashMap;

use Aspect;
use EntityData;
use Entity;
use {Active, Passive, System};
use World;

pub type DataMap<'a, Data> = (&'a Entity, &'a mut Data);

pub trait DataProcess<Data: 'static>: System
{
    fn init(&self, e: &Entity, w: &World) -> Data;

    fn process<'a, T: Iterator<DataMap<'a, Data>>>(&self, T, &mut EntityData);
}

pub struct DataSystem<Data: 'static, T: DataProcess<Data>>
{
    interested: HashMap<Entity, Data>,
    aspect: Aspect,
    inner: T,
}

impl<Data: 'static, T: DataProcess<Data>> DataSystem<Data, T>
{
    pub fn new(inner: T, aspect: Aspect) -> DataSystem<Data, T>
    {
        DataSystem
        {
            interested: HashMap::new(),
            aspect: aspect,
            inner: inner,
        }
    }
}

impl<Data: 'static, T: DataProcess<Data>> Active for DataSystem<Data, T>
{
    fn process(&mut self, c: &mut EntityData)
    {
        self.inner.process(self.interested.iter_mut(), c);
    }
}

impl<Data: 'static, T: DataProcess<Data>> System for DataSystem<Data, T>
{
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        if self.aspect.check(entity, world)
        {
            self.interested.insert(*entity, self.inner.init(entity, world));
            self.inner.activated(entity, world);
        }
    }

    fn reactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.contains_key(entity)
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
            self.interested.insert(*entity, self.inner.init(entity, world));
            self.inner.activated(entity, world);
        }
    }

    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        if self.interested.remove(entity).is_some()
        {
            self.inner.deactivated(entity, world);
        }
    }
}
