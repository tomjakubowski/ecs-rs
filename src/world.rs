
//! Management of entities, components, systems, and managers

use std::cell::{RefMut, RefCell};
use std::collections::HashMap;
use std::intrinsics::TypeId;
use std::mem;

use {Component, ComponentId};
use {Entity, EntityBuilder, EntityModifier};
use {Manager, MutableManager};
use {Active, Passive, System};
use component::ComponentList;
use entity::EntityManager;

pub struct World
{
    build_queue: RefCell<Vec<(Entity, Box<EntityBuilder+'static>)>>,
    modify_queue: RefCell<Vec<(Entity, Box<EntityModifier+'static>)>>,
    entities: RefCell<EntityManager>,
    components: RefCell<ComponentManager>,
    systems: RefCell<SystemManager>,
    mut_managers: Vec<RefCell<Box<MutableManager>>>,
    managers: Vec<Box<Manager>>,
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

    /// Deletes an entity, deactivating it if it is activated
    ///
    /// If the entity is invalid a warning is issued and this method does nothing.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        if self.is_valid(entity)
        {
            self.systems.borrow_mut().deactivated(entity, self);
            for ref manager in self.mut_managers.iter()
            {
                manager.borrow_mut().deactivated(entity, self);
            }
            for ref manager in self.managers.iter()
            {
                manager.deactivated(entity, self);
            }
            self.entities.borrow_mut().delete_entity(entity);
            self.components.borrow_mut().delete_entity(entity);
        }
        else
        {
            println!("[ecs] WARNING: Cannot delete invalid entity")
        }
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
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().activated(entity, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.activated(entity, self);
        }
    }

    fn reactivate_entity(&mut self, entity: &Entity)
    {
        self.systems.borrow_mut().reactivated(entity, self);
        for ref manager in self.mut_managers.iter()
        {
            manager.borrow_mut().reactivated(entity, self);
        }
        for ref manager in self.managers.iter()
        {
            manager.reactivated(entity, self);
        }
    }

    fn entity_data(&self) -> EntityData
    {
        EntityData
        {
            inner: self.components.borrow_mut()
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
            self.passive[key].process(world);
        }
        else
        {
            println!("[ecs] WARNING: No passive system registered for key '{}'", key);
        }
    }
}

impl MutableManager for SystemManager
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
        self.components[TypeId::of::<T>().hash()].add::<T>(entity, &component)
    }

    fn set<T:Component>(&mut self, entity: &Entity, component: T) -> bool
    {
        self.components[TypeId::of::<T>().hash()].set::<T>(entity, &component)
    }

    fn get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        self.components[TypeId::of::<T>().hash()].get::<T>(entity)
    }

    pub fn has(&self, entity: &Entity, id: ComponentId) -> bool
    {
        self.components[id].has(entity)
    }

    fn borrow_mut<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        self.components[TypeId::of::<T>().hash()].borrow_mut::<T>(entity)
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
                entities: RefCell::new(EntityManager::new()),
                components: RefCell::new(ComponentManager::new()),
                systems: RefCell::new(SystemManager::new()),
                mut_managers: Vec::new(),
                managers: Vec::new(),
            }
        }
    }

    /// Completes the world setup and return the World object for use.
    pub fn build(self) -> World
    {
        self.world
    }

    /// Registers a mutable manager.
    #[experimental]
    pub fn register_manager_mut(&mut self, manager: Box<MutableManager>)
    {
        self.world.mut_managers.push(RefCell::new(manager));
    }

    /// Registers an immutable manager.
    #[experimental]
    pub fn register_manager(&mut self, manager: Box<Manager>)
    {
        self.world.managers.push(manager);
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
