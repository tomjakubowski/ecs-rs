
use std::ops::{Deref, DerefMut};

use {BuildData, EntityData, ModifyData};
use {Entity, IndexedEntity, EntityIter};
use {EntityBuilder, EntityModifier};
use {System};
use entity::EntityManager;

#[derive(Clone)]
enum Event
{
    BuildEntity(Entity),
    RemoveEntity(Entity),
}

pub struct World<S> where S: SystemManager
{
    pub systems: S,
    pub data: DataHelper<S::Components, S::Services>,
}

pub struct DataHelper<C, M> where C: ComponentManager, M: ServiceManager
{
    pub components: C,
    pub services: M,
    entities: EntityManager<C>,
    event_queue: Vec<Event>,
}

pub unsafe trait ComponentManager: 'static
{
    unsafe fn new() -> Self;
    unsafe fn remove_all(&mut self, en: &IndexedEntity<Self>);
}

pub trait ServiceManager: 'static
{
    fn new() -> Self;
}

impl ServiceManager for () { fn new(){} }

pub unsafe trait SystemManager
{
    type Components: ComponentManager;
    type Services: ServiceManager;
    unsafe fn new() -> Self;
    unsafe fn activated(&mut self, en: EntityData<Self::Components>, co: &Self::Components);
    unsafe fn reactivated(&mut self, en: EntityData<Self::Components>, co: &Self::Components);
    unsafe fn deactivated(&mut self, en: EntityData<Self::Components>, co: &Self::Components);
    unsafe fn update(&mut self, co: &mut DataHelper<Self::Components, Self::Services>);
}

impl<S: SystemManager> Deref for World<S>
{
    type Target = DataHelper<S::Components, S::Services>;
    fn deref(&self) -> &DataHelper<S::Components, S::Services>
    {
        &self.data
    }
}

impl<S: SystemManager> DerefMut for World<S>
{
    fn deref_mut(&mut self) -> &mut DataHelper<S::Components, S::Services>
    {
        &mut self.data
    }
}

impl<C: ComponentManager, M: ServiceManager> Deref for DataHelper<C, M>
{
    type Target = C;
    fn deref(&self) -> &C
    {
        &self.components
    }
}

impl<C: ComponentManager, M: ServiceManager> DerefMut for DataHelper<C, M>
{
    fn deref_mut(&mut self) -> &mut C
    {
        &mut self.components
    }
}

impl<C: ComponentManager, M: ServiceManager> DataHelper<C, M>
{
    pub fn with_entity_data<F, R>(&mut self, entity: &Entity, mut call: F) -> Option<R>
        where F: FnMut(EntityData<C>, &mut C) -> R
    {
        // TODO cleanup
        if self.entities.is_valid(entity) {
            Some(call(EntityData(unsafe { &self.entities.indexed(&entity).clone() }), self))
        } else {
            None
        }
    }

    pub fn create_entity<B>(&mut self, mut builder: B) -> Entity where B: EntityBuilder<C>
    {
        let entity = self.entities.create();
        builder.build(BuildData(self.entities.indexed(&entity)), &mut self.components);
        self.event_queue.push(Event::BuildEntity(entity));
        entity
    }

    pub fn remove_entity(&mut self, entity: Entity)
    {
        self.event_queue.push(Event::RemoveEntity(entity));
    }
}

impl<S: SystemManager> World<S>
{
    pub fn new() -> World<S>
    {
        World {
            systems: unsafe { S::new() },
            data: DataHelper {
                components: unsafe { S::Components::new() },
                services: S::Services::new(),
                entities: EntityManager::new(),
                event_queue: Vec::new(),
            },
        }
    }

    pub fn entities(&self) -> EntityIter<S::Components>
    {
        self.data.entities.iter()
    }

    pub fn modify_entity<M>(&mut self, entity: Entity, mut modifier: M) where M: EntityModifier<S::Components>
    {
        let indexed = self.data.entities.indexed(&entity);
        modifier.modify(ModifyData(indexed), &mut self.data.components);
        unsafe { self.systems.reactivated(EntityData(indexed), &mut self.data.components); }
    }

    fn flush_queue(&mut self)
    {
        for e in self.data.event_queue.iter().map(|x| x.clone()) {
            match e {
                Event::BuildEntity(entity) => {
                    unsafe { self.systems.activated(EntityData(self.data.entities.indexed(&entity)), &mut self.data.components); }
                },
                Event::RemoveEntity(entity) => {
                    unsafe {
                        let indexed = self.data.entities.indexed(&entity);
                        self.systems.deactivated(EntityData(indexed), &mut self.data.components);
                        self.data.components.remove_all(indexed);
                    }
                    self.data.entities.remove(&entity);
                }
            }
        }
    }

    pub fn update(&mut self)
    {
        self.flush_queue();
        unsafe { self.systems.update(&mut self.data); }
        self.flush_queue();
    }
}
