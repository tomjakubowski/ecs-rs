
//! Management of entities, components, systems, and managers

use std::any::TypeId;
use std::cell::{RefCell};
use std::collections::HashMap;
use std::mem;

use {Component, ComponentId};
use {Entity, EntityBuilder, EntityModifier};
use {Manager};
use {Active, Passive, System};
use component::ComponentList;
use entity::EntityManager;
use error;

pub struct World
{
    build_queue: RefCell<Vec<(Entity, Box<EntityBuilder+'static>)>>,
    modify_queue: RefCell<Vec<(Entity, Box<EntityModifier+'static>)>>,
    remove_queue: RefCell<Vec<Entity>>,
    entities: RefCell<EntityManager>,
    components: ComponentManager,
    systems: RefCell<SystemManager>,
    managers: HashMap<&'static str, RefCell<Box<Manager>>>,
}

#[stable]
pub struct WorldBuilder
{
    world: World,
}

pub struct Components<'a>
{
    inner: &'a ComponentManager,
    world: &'a World,
}

pub struct EntityData<'a>
{
    inner: &'a ComponentManager,
    world: &'a World,
}

struct ComponentManager
{
    components: HashMap<ComponentId, RefCell<ComponentList>>,
}

struct SystemManager
{
    systems: Vec<Box<Active>>,
    passive: HashMap<&'static str, Box<Passive>>,
}

impl World
{
    /// Returns if an entity is valid (registered with the entity manager).
    #[stable]
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        self.entities.borrow().is_valid(entity)
    }

    #[stable]
    pub fn entity_count(&self) -> usize
    {
        self.entities.borrow().count()
    }

    pub fn with_entities<F>(&self, call: F) where F: FnMut(&Entity)
    {
        self.entities.borrow().with_entities(call)
    }

    pub fn clear(&mut self)
    {
        let entities = self.entities.borrow_mut().clear();
        for entity in entities.into_iter()
        {
            self.systems.borrow_mut().deactivated(&entity, self);
            for ref manager in self.managers.values()
            {
                manager.borrow_mut().deactivated(&entity, self);
            }
        }
        self.components.clear();
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
                builder.build(&mut self.components(), entity.clone());
                self.activate_entity(&entity);
            }
            else
            {
                error("Couldn't build invalid entity");
            }
        }
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut *self.modify_queue.borrow_mut());
        for (entity, mut modifier) in queue.into_iter()
        {
            if self.is_valid(&entity)
            {
                modifier.modify(&mut self.components(), entity.clone());
                self.reactivate_entity(&entity);
            }
            else
            {
                error("Couldn't modify invalid entity");
            }
        }
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut *self.remove_queue.borrow_mut());
        for entity in queue.into_iter()
        {
            self.remove_entity(&entity);
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
        builder.build(&mut self.components(), entity.clone());
        self.activate_entity(&entity);
        entity
    }

    /// Modifies an entity with the given modifier.
    pub fn modify_entity<T: EntityModifier>(&mut self, entity: Entity, mut modifier: T)
    {
        modifier.modify(&mut self.components(), entity.clone());
        self.reactivate_entity(&entity);
    }

    /// Removes an entity, deactivating it if it is activated
    ///
    /// If the entity is invalid a warning is issued and this method does nothing.
    pub fn remove_entity(&mut self, entity: &Entity)
    {
        if self.is_valid(entity)
        {
            self.systems.borrow_mut().deactivated(entity, self);
            for ref manager in self.managers.values()
            {
                manager.borrow_mut().deactivated(entity, self);
            }
            self.components.delete_entity(entity);
            self.entities.borrow_mut().delete_entity(entity);
        }
        else
        {
            error("Cannot remove invalid entity")
        }
    }

    /// Queues a entity to be built at the end of the next update cycle.
    pub fn queue_builder<T: EntityBuilder>(&self, builder: T) -> Entity
    {
        let entity = self.entities.borrow_mut().create_entity();
        self.build_queue.borrow_mut().push((entity.clone(), box builder));
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

    /// Calls a function with an immutable reference to the requested manager
    pub fn with_manager<T: Manager, U, F>(&self, key: &'static str, mut call: F) -> U
        where F: FnMut(&T) -> U
    {
        match self.managers.get(&key)
        {
            Some(any) => match any.borrow().downcast_ref::<T>() {
                Some(manager) => call(manager),
                None => error("Tried to downcast manager to wrong type")
            },
            None => error(&*format!("Could not find any manager for key '{}'", key))
        }
    }

    /// Calls a function with a mutable reference to the requested manager
    pub fn with_manager_mut<T: Manager, U, F>(&self, key: &'static str, mut call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        match self.managers.get(&key)
        {
            Some(any) => match any.borrow_mut().downcast_mut::<T>() {
                Some(manager) => call(manager),
                None => error("Tried to downcast manager to wrong type")
            },
            None => error(&*format!("Could not find any manager for key '{}'", key))
        }
    }

    /// Sets the value of a component for an entity
    ///
    /// Fails if the component does not exist or the entity is invalid
    pub fn set_component<T: Component>(&self, entity: &Entity, val: T)
    {
        if self.is_valid(entity)
        {
            self.components.set::<T>(entity, val)
        }
        else
        {
            error("Cannot set component for invalid entity")
        }
    }

    /// Returns the value of a component for an entity
    ///
    /// Fails if the component does not exist or the entity is invalid
    pub fn get_component<T: Component>(&self, entity: &Entity) -> T
    {
        if self.is_valid(entity)
        {
            self.components.get::<T>(entity)
        }
        else
        {
            error("Cannot get component for invalid entity")
        }
    }

    /// Returns the value of a component for an entity (or None)
    ///
    /// Fails if the entity is invalid
    pub fn try_component<T: Component>(&self, entity: &Entity) -> Option<T>
    {
        if self.is_valid(entity)
        {
            self.components.try_get::<T>(entity)
        }
        else
        {
            error("Cannot get component for invalid entity")
        }
    }

    /// Calls a function with a mutable reference to a component and returns the result or None
    /// if the component does not exist.
    ///
    /// Panics if the entity is invalid.
    pub fn try_with_component<T:Component, U, F>(&self, entity: &Entity, call: F) -> Option<U>
        where F: FnMut(&mut T) -> U
    {
        if self.is_valid(entity)
        {
            self.components.try_with(entity, call)
        }
        else
        {
            error("Cannot modify component for invalid entity")
        }
    }

    /// Calls a function with a mutable reference to a component and returns the result.
    ///
    /// Panics if the component does not exist or the entity is invalid.
    pub fn with_component<T:Component, U, F>(&self, entity: &Entity, call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        if self.is_valid(entity)
        {
            self.components.with(entity, call)
        }
        else
        {
            error("Cannot modify component for invalid entity")
        }
    }


    /// Returns whether an entity has a component.
    pub fn has_component(&self, entity: &Entity, id: ComponentId) -> bool
    {
        self.components.has(entity, id)
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
            inner: &self.components,
            world: self,
        }
    }

    fn components(&self) -> Components
    {
        Components
        {
            inner: &self.components,
            world: self,
        }
    }
}

impl SystemManager
{
    fn new() -> SystemManager
    {
        SystemManager
        {
            systems: Vec::new(),
            passive: HashMap::new(),
        }
    }

    fn register(&mut self, sys: Box<Active>)
    {
        self.systems.push(sys);
    }

    fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        self.passive.insert(key, sys);
    }

    fn process(&mut self, mut components: EntityData)
    {
        for sys in self.systems.iter_mut()
        {
            sys.process(&mut components);
        }
    }

    fn update_passive(&mut self, key: &'static str, world: &World)
    {
        if self.passive.contains_key(&key)
        {
            let passive: &mut Box<Passive> = &mut self.passive[key];
            passive.process(world);
        }
        else
        {
            error(&*format!("No passive system registered for key '{}'", key));
        }
    }

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

    fn clear(&mut self)
    {
        for list in self.components.values()
        {
            list.borrow_mut().clear();
        }
    }

    fn register(&mut self, list: ComponentList)
    {
        self.components.insert(list.get_cid(), RefCell::new(list));
    }

    fn delete_entity(&mut self, entity: &Entity)
    {
        for (_, list) in self.components.iter()
        {
            list.borrow_mut().remove(entity);
        }
    }

    fn add<T:Component>(&self, entity: &Entity, component: T)
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow_mut().add::<T>(entity, &component),
            None => error("Attempted to add an unregistered component"),
        }
    }

    fn set<T:Component>(&self, entity: &Entity, component: T)
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow_mut().set::<T>(entity, &component),
            None => error("Attempted to set an unregistered component"),
        }
    }

    fn get<T:Component>(&self, entity: &Entity) -> T
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow().get::<T>(entity),
            None => error("Attempted to access an unregistered component"),
        }
    }

    fn try_get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow().try_get::<T>(entity),
            None => error("Attempted to access an unregistered component"),
        }
    }

    fn has(&self, entity: &Entity, id: ComponentId) -> bool
    {
        match self.components.get(&id)
        {
            Some(entry) => entry.borrow().has(entity),
            None => error("Attempted to access an unregistered component"),
        }
    }

    fn try_with<T:Component, U, F>(&self, entity: &Entity, mut call: F) -> Option<U>
        where F: FnMut(&mut T) -> U
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow_mut().try_borrow::<T>(entity).map(|c| call(c)),
            None => error("Attempted to access an unregistered component"),
        }
    }

    fn with<T:Component, U, F>(&self, entity: &Entity, mut call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => call(entry.borrow_mut().borrow::<T>(entity)),
            None => error("Attempted to access an unregistered component"),
        }
    }

    fn remove<T:Component>(&self, entity: &Entity)
    {
        match self.components.get(&TypeId::of::<T>())
        {
            Some(entry) => entry.borrow_mut().remove(entity),
            None => error("Attempted to remove an unregistered component"),
        }
    }
}

impl<'a> Components<'a>
{
    /// Adds a component to an entity.
    pub fn add<T:Component>(&mut self, entity: &Entity, component: T)
    {
        self.inner.add(entity, component)
    }

    /// Sets an entity's component.
    pub fn set<T:Component>(&mut self, entity: &Entity, component: T)
    {
        self.inner.set(entity, component)
    }

    /// Returns an entity's component.
    ///
    /// Fails if the component doesn't exist.
    pub fn get<T:Component>(&self, entity: &Entity) -> T
    {
        self.inner.get(entity)
    }

    /// Returns an entity's component or None if it can't be found.
    pub fn try_get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        self.inner.try_get(entity)
    }

    /// Check if an entity has a component
    pub fn has(&self, entity: &Entity, id: ComponentId) -> bool
    {
        self.inner.has(entity, id)
    }

    /// Remove a component from an entity
    pub fn remove<T:Component>(&mut self, entity: &Entity)
    {
        self.inner.remove::<T>(entity)
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

    /// Calls a function with an immutable reference to the requested manager
    pub fn with_manager<T: Manager, U, F>(&self, key: &'static str, call: F) -> U
        where F: FnMut(&T) -> U
    {
        self.world.with_manager(key, call)
    }

    /// Calls a function with an mutable reference to the requested manager
    pub fn with_manager_mut<T: Manager, U, F>(&self, key: &'static str, call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        self.world.with_manager_mut(key, call)
    }
}

impl<'a> EntityData<'a>
{
    /// Calls a function with a mutable reference to a component and returns the result or None
    /// if the component does not exist.
    pub fn try_with<T:Component, U, F>(&self, entity: &Entity, call: F) -> Option<U>
        where F: FnMut(&mut T) -> U
    {
        self.inner.try_with(entity, call)
    }

    /// Calls a function with a mutable reference to a component and returns the result.
    ///
    /// Panics if the component does not exist.
    pub fn with<T:Component, U, F>(&self, entity: &Entity, call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        self.inner.with(entity, call)
    }

    /// Sets an entity's component.
    pub fn set<T:Component>(&self, entity: &Entity, component: T)
    {
        self.inner.set(entity, component)
    }

    /// Returns an entity's component.
    ///
    /// Fails if the component doesn't exist.
    pub fn get<T:Component>(&self, entity: &Entity) -> T
    {
        self.inner.get(entity)
    }

    /// Returns an entity's component or None if it can't be found.
    pub fn try_get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        self.inner.try_get(entity)
    }

    /// Check if an entity has a component
    pub fn has(&self, entity: &Entity, id: ComponentId) -> bool
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

    /// Calls a function with an immutable reference to the requested manager
    pub fn with_manager<T: Manager, U, F>(&self, key: &'static str, call: F) -> U
        where F: FnMut(&T) -> U
    {
        self.world.with_manager(key, call)
    }

    /// Calls a function with an mutable reference to the requested manager
    pub fn with_manager_mut<T: Manager, U, F>(&self, key: &'static str, call: F) -> U
        where F: FnMut(&mut T) -> U
    {
        self.world.with_manager_mut(key, call)
    }
}

#[stable]
impl WorldBuilder
{
    /// Create a new world builder.
    #[stable]
    pub fn new() -> WorldBuilder
    {
        WorldBuilder {
            world: World {
                build_queue: RefCell::new(Vec::new()),
                modify_queue: RefCell::new(Vec::new()),
                remove_queue: RefCell::new(Vec::new()),
                entities: RefCell::new(EntityManager::new()),
                components: ComponentManager::new(),
                systems: RefCell::new(SystemManager::new()),
                managers: HashMap::new(),
            }
        }
    }

    /// Completes the world setup and return the World object for use.
    #[stable]
    pub fn build(self) -> World
    {
        self.world
    }

    /// Registers a manager.
    #[stable]
    pub fn register_manager(&mut self, key: &'static str, manager: Box<Manager>)
    {
        self.world.managers.insert(key, RefCell::new(manager));
    }

    /// Registers a component.
    #[stable]
    pub fn register_component<T: Component>(&mut self)
    {
        self.world.components.register(ComponentList::new::<T>());
    }

    /// Registers a system.
    #[stable]
    pub fn register_system(&mut self, sys: Box<Active>)
    {
        self.world.systems.borrow_mut().register(sys);
    }

    /// Registers a passive system.
    #[stable]
    pub fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        self.world.systems.borrow_mut().register_passive(key, sys);
    }
}
