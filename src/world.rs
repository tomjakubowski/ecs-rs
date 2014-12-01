
//! Management of entities, components, systems, and managers

use std::any::{Any, AnyRefExt, AnyMutRefExt};
use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use std::intrinsics::TypeId;
use std::mem;

use {Component, ComponentId};
use {Entity, EntityBuilder, EntityModifier};
use {Manager};
use {Active, Passive, System};
use component::ComponentList;
use entity::EntityManager;

pub struct World
{
    build_queue: RefCell<Vec<(Entity, Box<EntityBuilder+'static>)>>,
    modify_queue: RefCell<Vec<(Entity, Box<EntityModifier+'static>)>>,
    remove_queue: RefCell<Vec<Entity>>,
    entities: RefCell<EntityManager>,
    components: RefCell<ComponentManager>,
    systems: RefCell<SystemManager>,
    managers: HashMap<&'static str, RefCell<Box<Manager>>>,
}

#[experimental]
pub struct WorldBuilder
{
    world: World,
}

#[experimental]
pub struct Components<'a>
{
    inner: RefMut<'a, ComponentManager>,
}

#[experimental]
pub struct EntityData<'a>
{
    inner: RefMut<'a, ComponentManager>,
    world: &'a World,
}

struct ComponentManager
{
    components: HashMap<ComponentId, ComponentList>,
}

struct SystemManager
{
    systems: Vec<Box<Active>>,
    passive: HashMap<&'static str, Box<Passive>>,
}

impl World
{
    /// Returns if an entity is valid (registered with the entity manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        self.entities.borrow().is_valid(entity)
    }

    pub fn entity_count(&self) -> uint
    {
        self.entities.borrow().count()
    }

    /// Updates the world by processing all systems.
    pub fn update(&mut self)
    {
        self.systems.borrow_mut().process(self.entity_data());
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut *self.build_queue.borrow_mut());
        for (entity, mut builder) in queue.into_iter()
        {
            if self.is_valid(&entity)
            {
                builder.build(&mut self.components(), entity);
                self.activate_entity(&entity);
            }
            else
            {
                println!("[ecs] WARNING: Couldn't build invalid entity");
            }
        }
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut *self.modify_queue.borrow_mut());
        for (entity, mut modifier) in queue.into_iter()
        {
            if self.is_valid(&entity)
            {
                modifier.modify(&mut self.components(), entity);
                self.reactivate_entity(&entity);
            }
            else
            {
                println!("[ecs] WARNING: Couldn't modify invalid entity");
            }
        }
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut *self.remove_queue.borrow_mut());
        for entity in queue.into_iter()
        {
            self.delete_entity(&entity);
        }
    }

    /// Updates the passive system corresponding to `key`
    pub fn update_passive(&mut self, key: &'static str)
    {
        self.systems.borrow_mut().update_passive(key, self);
    }

    /// Create an entity with the given builder.
    pub fn build_entity<T: EntityBuilder>(&mut self, mut builder: T) -> Entity
    {
        let entity = self.entities.borrow_mut().create_entity();
        builder.build(&mut self.components(), entity);
        self.activate_entity(&entity);
        entity
    }

    /// Modifies an entity with the given modifier.
    pub fn modify_entity<T: EntityModifier>(&mut self, entity: Entity, mut modifier: T)
    {
        modifier.modify(&mut self.components(), entity);
        self.reactivate_entity(&entity);
    }

    /// Queues a entity to be built at the end of the next update cycle.
    pub fn queue_builder<T: EntityBuilder>(&self, builder: T) -> Entity
    {
        let entity = self.entities.borrow_mut().create_entity();
        self.build_queue.borrow_mut().push((entity, box builder));
        entity
    }

    /// Queues a entity to be modified at the end of the next update cycle.
    pub fn queue_modifier<T: EntityModifier>(&self, entity: Entity, modifier: T)
    {
        self.modify_queue.borrow_mut().push((entity, box modifier));
    }

    /// Queues a entity to be removed at the end of the next update cycle.
    pub fn queue_removal(&self, entity: Entity)
    {
        self.remove_queue.borrow_mut().push(entity);
    }

    /// Deletes an entity, deactivating it if it is activated
    ///
    /// If the entity is invalid a warning is issued and this method does nothing.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        if self.is_valid(entity)
        {
            self.systems.borrow_mut().deactivated(entity, self);
            for ref manager in self.managers.values()
            {
                manager.borrow_mut().deactivated(entity, self);
            }
            self.components.borrow_mut().delete_entity(entity);
            self.entities.borrow_mut().delete_entity(entity);
        }
        else
        {
            println!("[ecs] WARNING: Cannot delete invalid entity")
        }
    }

    pub fn with_manager<T: Manager, U>(&self, key: &'static str, call: |&T| -> U) -> Option<U>
    {
        self.managers.get(&key).and_then(|a|
            (&*a.borrow() as &Any).downcast_ref().map(
                |m| call(m)
            )
        )
    }
    
    pub fn with_manager_mut<T: Manager, U>(&self, key: &'static str, call: |&mut T| -> U) -> Option<U>
    {
        self.managers.get(&key).and_then(|a|
            (&mut *a.borrow_mut() as &mut Any).downcast_mut().map(
                |m| call(m)
            )
        )
    }

    /// Returns the value of a component for an entity (or None)
    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<T>
    {
        if self.is_valid(entity)
        {
            self.components.borrow().get::<T>(entity)
        }
        else
        {
            None
        }
    }

    /// Returns if an entity contains a component.
    pub fn has_component(&self, entity: &Entity, id: ComponentId) -> bool
    {
        self.components.borrow().has(entity, id)
    }

    fn activate_entity(&mut self, entity: &Entity)
    {
        self.systems.borrow_mut().activated(entity, self);
        for ref manager in self.managers.values()
        {
            manager.borrow_mut().activated(entity, self);
        }
    }

    fn reactivate_entity(&mut self, entity: &Entity)
    {
        self.systems.borrow_mut().reactivated(entity, self);
        for ref manager in self.managers.values()
        {
            manager.borrow_mut().reactivated(entity, self);
        }
    }

    fn entity_data(&self) -> EntityData
    {
        EntityData
        {
            inner: self.components.borrow_mut(),
            world: self,
        }
    }

    fn components(&self) -> Components
    {
        Components
        {
            inner: self.components.borrow_mut()
        }
    }
}

impl SystemManager
{
    pub fn new() -> SystemManager
    {
        SystemManager
        {
            systems: Vec::new(),
            passive: HashMap::new(),
        }
    }

    pub fn register(&mut self, sys: Box<Active>)
    {
        self.systems.push(sys);
    }

    pub fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        self.passive.insert(key, sys);
    }

    pub fn process(&mut self, mut components: EntityData)
    {
        for sys in self.systems.iter_mut()
        {
            sys.process(&mut components);
        }
    }

    pub fn update_passive(&mut self, key: &'static str, world: &World)
    {
        if self.passive.contains_key(&key)
        {
            let passive: &mut Box<Passive> = &mut self.passive[key];
            passive.process(world);
        }
        else
        {
            println!("[ecs] WARNING: No passive system registered for key '{}'", key);
        }
    }
}

impl Manager for SystemManager
{
    fn activated(&mut self, e: &Entity, w: &World)
    {
        for sys in self.systems.iter_mut()
        {
            sys.activated(e, w);
        }
        for (_, sys) in self.passive.iter_mut()
        {
            sys.activated(e, w);
        }
    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        for sys in self.systems.iter_mut()
        {
            sys.reactivated(e, w);
        }
        for (_, sys) in self.passive.iter_mut()
        {
            sys.reactivated(e, w);
        }
    }

    fn deactivated(&mut self, e: &Entity, w: &World)
    {
        for sys in self.systems.iter_mut()
        {
            sys.deactivated(e, w);
        }
        for (_, sys) in self.passive.iter_mut()
        {
            sys.deactivated(e, w);
        }
    }
}

impl ComponentManager
{
    pub fn new() -> ComponentManager
    {
        ComponentManager
        {
            components: HashMap::new(),
        }
    }

    pub fn register(&mut self, list: ComponentList)
    {
        self.components.insert(list.get_cid(), list);
    }

    fn delete_entity(&mut self, entity: &Entity)
    {
        for (_, list) in self.components.iter_mut()
        {
            list.rm(entity);
        }
    }

    fn add<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        match self.components.get_mut(&TypeId::of::<T>().hash())
        {
            Some(entry) => entry.add::<T>(entity, &component),
            None => {
                println!("[ecs] WARNING: Attempted to add an unregistered component");
                false
            }
        }
    }

    fn set<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        match self.components.get_mut(&TypeId::of::<T>().hash())
        {
            Some(entry) => entry.set::<T>(entity, &component),
            None => {
                println!("[ecs] WARNING: Attempted to modify an unregistered component");
                false
            }
        }
    }

    fn get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        match self.components.get(&TypeId::of::<T>().hash())
        {
            Some(entry) => entry.get::<T>(entity),
            None => {
                println!("[ecs] WARNING: Attempted to access an unregistered component");
                None
            }
        }
    }

    pub fn has(&self, entity: &Entity, id: ComponentId) -> bool
    {
        match self.components.get(&id)
        {
            Some(entry) => entry.has(entity),
            None => {
                println!("[ecs] WARNING: Attempted to access an unregistered component");
                false
            }
        }
    }

    fn borrow_mut<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        self.components.get_mut(&TypeId::of::<T>().hash()).and_then(|entry| entry.borrow_mut::<T>(entity))
    }

    fn remove<T:Component>(&mut self, entity: &Entity) -> bool
    {
        self.components[TypeId::of::<T>().hash()].rm(entity)
    }
}

#[experimental]
impl<'a> Components<'a>
{
    pub fn add<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.inner.add::<T>(entity, component)
    }

    pub fn set<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.inner.set::<T>(entity, component)
    }

    pub fn get<T:Component>(&mut self, entity: &Entity) -> Option<T>
    {
        self.inner.get::<T>(entity)
    }

    pub fn has(&mut self, entity: &Entity, id: ComponentId) -> bool
    {
        self.inner.has(entity, id)
    }

    pub fn remove<T:Component>(&mut self, entity: &Entity) -> bool
    {
        self.inner.remove::<T>(entity)
    }
}

#[experimental]
impl<'a> EntityData<'a>
{
    pub fn borrow<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        self.inner.borrow_mut::<T>(entity)
    }

    pub fn set<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.inner.set::<T>(entity, component)
    }

    pub fn get<T:Component>(&mut self, entity: &Entity) -> Option<T>
    {
        self.inner.get::<T>(entity)
    }

    pub fn has(&mut self, entity: &Entity, id: ComponentId) -> bool
    {
        self.inner.has(entity, id)
    }

    /// Queues a entity to be built at the end of the next update cycle.
    pub fn build_entity<T: EntityBuilder>(&self, builder: T) -> Entity
    {
        self.world.queue_builder(builder)
    }

    /// Queues a entity to be modified at the end of the next update cycle.
    pub fn modify_entity<T: EntityModifier>(&self, entity: Entity, modifier: T)
    {
        self.world.queue_modifier(entity, modifier)
    }

    /// Queues a entity to be removed at the end of the next update cycle.
    pub fn remove_entity(&self, entity: Entity)
    {
        self.world.queue_removal(entity)
    }
}

impl WorldBuilder
{
    /// Create a new world builder.
    pub fn new() -> WorldBuilder
    {
        WorldBuilder {
            world: World {
                build_queue: RefCell::new(Vec::new()),
                modify_queue: RefCell::new(Vec::new()),
                remove_queue: RefCell::new(Vec::new()),
                entities: RefCell::new(EntityManager::new()),
                components: RefCell::new(ComponentManager::new()),
                systems: RefCell::new(SystemManager::new()),
                managers: HashMap::new(),
            }
        }
    }

    /// Completes the world setup and return the World object for use.
    pub fn build(self) -> World
    {
        self.world
    }

    /// Registers a manager.
    #[experimental]
    pub fn register_manager(&mut self, key: &'static str, manager: Box<Manager>)
    {
        self.world.managers.insert(key, RefCell::new(manager));
    }

    /// Registers a component.
    pub fn register_component<T: Component>(&mut self)
    {
        self.world.components.borrow_mut().register(ComponentList::new::<T>());
    }

    /// Registers a system.
    pub fn register_system(&mut self, sys: Box<Active>)
    {
        self.world.systems.borrow_mut().register(sys);
    }

    /// Registers a passive system.
    pub fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        self.world.systems.borrow_mut().register_passive(key, sys);
    }
}
