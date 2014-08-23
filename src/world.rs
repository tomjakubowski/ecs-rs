
//! Management of entities, components, systems, and managers

use std::cell::{RefMut, RefCell};
use std::collections::HashMap;

use {Component, ComponentId};
use Entity;
use {Manager, MutableManager};
use Phantom;
use {Passive, System};
use component::ComponentList;
use entity::EntityManager;

pub struct World
{
    entities: RefCell<EntityManager>,
    components: RefCell<ComponentManager>,
    systems: RefCell<SystemManager>,
    mut_managers: Vec<RefCell<Box<MutableManager>>>,
    managers: Vec<Box<Manager>>,
    locked: bool,
}

pub struct Components<'a>
{
    inner: RefMut<'a, ComponentManager>,
}

struct ComponentManager
{
    components: HashMap<ComponentId, ComponentList>,
}

struct SystemManager
{
    systems: Vec<Box<System>>,
    passive: HashMap<&'static str, Box<Passive>>,
}

impl World
{
    pub fn new() -> World
    {
        World
        {
            entities: RefCell::new(EntityManager::new()),
            components: RefCell::new(ComponentManager::new()),
            systems: RefCell::new(SystemManager::new()),
            mut_managers: Vec::new(),
            managers: Vec::new(),
            locked: false,
        }
    }

    /// Lock down the managers and systems, and components of a world.
    ///
    /// This must be called before entities can be created and modified.
    /// After it is called, no systems/managers/components may be registered.
    pub fn finalise(&mut self)
    {
        if self.locked
        {
            fail!("World is already finalised")
        }
        self.locked = true;
    }

    /// Returns if an entity has been activated.
    #[inline]
    pub fn is_active(&self, entity: &Entity) -> bool
    {
        self.entities.borrow().is_activated(entity)
    }

    /// Registers a mutable manager.
    ///
    /// # Failure
    ///
    /// If the world has been finalised
    #[experimental]
    pub fn register_manager_mut(&mut self, manager: Box<MutableManager>)
    {
        if self.locked
        {
            fail!("World is locked. Managers may not be registered")
        }
        self.mut_managers.push(RefCell::new(manager));
    }

    /// Registers an immutable manager.
    ///
    /// # Failure
    ///
    /// If the world has been finalised
    #[experimental]
    pub fn register_manager(&mut self, manager: Box<Manager>)
    {
        if self.locked
        {
            fail!("World is locked. Managers may not be registered")
        }
        self.managers.push(manager);
    }

    /// Registers a component.
    ///
    /// # Failure
    ///
    /// If the world has been finalised
    pub fn register_component<T: Component>(&mut self)
    {
        if self.locked
        {
            fail!("World is locked. Components may not be registered")
        }
        self.components.borrow_mut().register(ComponentList::new(Phantom::<T>));
    }

    /// Registers a system.
    ///
    /// # Failure
    ///
    /// If the world has been finalised
    pub fn register_system(&mut self, sys: Box<System>)
    {
        if self.locked
        {
            fail!("World is locked. Systems may not be registered")
        }
        self.systems.borrow_mut().register(sys);
    }

    /// Registers a passive system.
    ///
    /// # Failure
    ///
    /// If the world has been finalised
    pub fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        if self.locked
        {
            fail!("World is locked. Systems may not be registered")
        }
        self.systems.borrow_mut().register_passive(key, sys);
    }

    /// Updates the world by processing all systems.
    ///
    /// # Failure
    ///
    /// If the world has not been finalised
    pub fn update(&mut self)
    {
        if !self.locked
        {
            fail!("World must be locked before updating")
        }
        self.systems.borrow_mut().update(self, self.components());
    }

    /// Updates the passive system corresponding to `key`
    ///
    /// # Failure
    ///
    /// If the world has not been finalised
    pub fn update_passive(&mut self, key: &'static str)
    {
        if !self.locked
        {
            fail!("World must be locked before updating")
        }
        self.systems.borrow_mut().update_passive(key, self);
    }

    fn components(&self) -> Components
    {
        Components
        {
            inner: self.components.borrow_mut()
        }
    }

    /// Creates an entity
    ///
    /// # Failure
    ///
    /// If the world has not been finalised
    pub fn create_entity(&mut self) -> Entity
    {
        if !self.locked
        {
            fail!("World must be locked before creating entities")
        }
        let ret = self.entities.borrow_mut().create_entity();
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().added(&ret, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.added(&ret, self);
        }
        ret
    }

    /// Activates an entity
    ///
    /// Once activated, components cannot be added to the entity,
    /// and systems will be able to process it.
    ///
    /// # Failure
    ///
    /// If the entity is invalid or already activated
    pub fn activate_entity(&mut self, entity: &Entity)
    {
        if self.is_active(entity)
        {
            fail!("Entity is already activated")
        }
        if !self.entities.borrow().is_valid(entity)
        {
            fail!("Cannot activate invalid entity")
        }
        self.entities.borrow_mut().activate_entity(entity);
        self.systems.borrow_mut().activated(entity, self);
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().activated(entity, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.activated(entity, self);
        }
    }

    /// Deactivates an entity
    ///
    /// It will then be ignored by all systems.
    /// Components can only be added/removed from an entity while deactivated.
    ///
    /// # Failure
    ///
    /// If the entity is not already activated
    pub fn deactivate_entity(&mut self, entity: &Entity)
    {
        if !self.is_active(entity)
        {
            fail!("Cannot deactivate unactivated entity")
        }
        self.entities.borrow_mut().deactivate_entity(entity);
        self.systems.borrow_mut().deactivated(entity, self);
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().deactivated(entity, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.deactivated(entity, self);
        }
    }

    /// Deletes an entity, deactivating it if it is activated
    ///
    /// # Failure
    ///
    /// If an entity is invalid
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        if !self.entities.borrow().is_valid(entity)
        {
            fail!("Cannot delete invalid entity")
        }
        if self.is_active(entity)
        {
            self.deactivate_entity(entity);
        }
        self.entities.borrow_mut().delete_entity(entity);
        self.components.borrow_mut().delete_entity(entity);
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().removed(entity, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.removed(entity, self);
        }
    }

    /// Add a component to an entity
    ///
    /// Returns false if the component could not be added (entity invalid or activated).
    pub fn add_component<T: Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.entities.borrow().is_valid(entity)
        && !self.is_active(entity)
        && self.components.borrow_mut().add::<T>(entity, component)
    }

    /// Removes a component from an entity.
    ///
    /// Returns false if the component could not be removed (entity invalid or activated).
    pub fn remove_component<T: Component>(&mut self, entity: &Entity) -> bool
    {
        self.entities.borrow().is_valid(entity)
        && !self.is_active(entity)
        && self.components.borrow_mut().remove::<T>(entity)
    }

    /// Set the value of a component for an entity
    ///
    /// Returns false if the entity does not contain that component.
    pub fn set_component<T: Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.entities.borrow().is_valid(entity)
        && self.components.borrow_mut().set::<T>(entity, component)
    }

    /// Returns the value of a component for an entity (or None)
    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<T>
    {
        if self.entities.borrow().is_valid(entity)
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

    pub fn register(&mut self, sys: Box<System>)
    {
        self.systems.push(sys);
    }

    pub fn register_passive(&mut self, key: &'static str, sys: Box<Passive>)
    {
        self.passive.insert(key, sys);
    }

    pub fn update(&mut self, world: &World, mut components: Components)
    {
        for sys in self.systems.mut_iter()
        {
            sys.preprocess(world);
        }
        for sys in self.systems.iter()
        {
            sys.process(world, &mut components);
        }
        for sys in self.systems.mut_iter()
        {
            sys.postprocess(world);
        }
    }

    pub fn update_passive(&mut self, key: &'static str, world: &World)
    {
        self.passive.get_mut(&key).process(world);
    }
}

impl MutableManager for SystemManager
{
    fn added(&mut self, _: &Entity, _: &World)
    {

    }

    fn removed(&mut self, _: &Entity, _: &World)
    {

    }

    fn activated(&mut self, e: &Entity, w: &World)
    {
        for sys in self.systems.mut_iter()
        {
            sys.activated(e, w);
        }
        for (_, sys) in self.passive.mut_iter()
        {
            sys.activated(e, w);
        }
    }

    fn deactivated(&mut self, e: &Entity, _: &World)
    {
        for sys in self.systems.mut_iter()
        {
            sys.deactivated(e);
        }
        for (_, sys) in self.passive.mut_iter()
        {
            sys.deactivated(e);
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
        for (_, list) in self.components.mut_iter()
        {
            list.rm(entity);
        }
    }

    fn add<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.components.get_mut(&Component::cid(Phantom::<T>)).add::<T>(entity, &component)
    }

    fn set<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.components.get_mut(&Component::cid(Phantom::<T>)).set::<T>(entity, &component)
    }

    fn get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        self.components[Component::cid(Phantom::<T>)].get::<T>(entity)
    }

    pub fn has(&self, entity: &Entity, id: ComponentId) -> bool
    {
        self.components[id].has(entity)
    }

    fn borrow_mut<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        self.components.get_mut(&Component::cid(Phantom::<T>)).borrow_mut::<T>(entity)
    }

    fn remove<T:Component>(&mut self, entity: &Entity) -> bool
    {
        self.components.get_mut(&Component::cid(Phantom::<T>)).rm(entity)
    }
}

impl<'a> Components<'a>
{
    pub fn borrow<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        self.inner.borrow_mut::<T>(entity)
    }
}
