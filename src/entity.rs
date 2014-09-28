
//! Entity identifier and manager types.

use std::collections::Bitv;
use uuid::Uuid;

use world::World;

/// Dual identifier for an entity.
///
/// The first element (uint) is the entity's index, used to locate components.
/// This value can be recycled, so the second element (Uuid) is used as an identifier.
#[stable]
#[deriving(Clone, Eq, PartialEq, Show)]
pub struct Entity(uint, Uuid);

#[stable]
impl Entity
{
    /// Returns the entity's index.
    #[inline]
    pub fn get_index(&self) -> uint
    {
        let &Entity(i, _) = self;
        i
    }

    /// Returns the entity's unique identifier.
    #[inline]
    pub fn get_id(&self) -> Uuid
    {
        let &Entity(_, id) = self;
        id
    }
}

impl Deref<uint> for Entity
{
    #[inline]
    fn deref(&self) -> &uint
    {
        let &Entity(ref i, _) = self;
        i
    }
}

pub trait EntityBuilder
{
    fn build(&mut self, &mut World, Entity);
}

impl<'a> EntityBuilder for |&mut World, Entity|: 'a
{
    fn build(&mut self, w: &mut World, e: Entity)
    {
        (*self)(w, e);
    }
}

/// Handles creation, activation, and validating of entities.
#[doc(hidden)]
pub struct EntityManager
{
    ids: IdPool,
    entities: Vec<Entity>,
    enabled: Bitv,
    activated: Bitv,
}

impl EntityManager
{
    /// Returns a new `EntityManager`
    pub fn new() -> EntityManager
    {
        EntityManager
        {
            ids: IdPool::new(),
            entities: Vec::new(),
            enabled: Bitv::new(),
            activated: Bitv::new(),
        }
    }

    /// Creates a new `Entity`, assigning it the first available identifier.
    pub fn create_entity(&mut self) -> Entity
    {
        let ret = Entity(self.ids.get_id(), Uuid::new_v4());
        if *ret >= self.entities.len()
        {
            let diff = *ret - self.entities.len();
            self.entities.grow(diff+1, Entity(0, Uuid::nil()));
        }
        self.entities[mut][*ret] = ret;

        if *ret >= self.enabled.len()
        {
            let diff = *ret - self.enabled.len();
            self.enabled.grow(diff+1, false);
            self.activated.grow(diff+1, false);
        }
        self.enabled.set(*ret, true);
        ret
    }

    /// Marks an entity as activated, locking its components and allowing it to be used by systems.
    pub fn activate_entity(&mut self, entity: &Entity)
    {
        if self.is_valid(entity)
        {
            if self.activated[**entity]
            {
                fail!("Entity already activated")
            }
            self.activated.set(**entity, true);
        }
        else
        {
            fail!("Tried to activate invalid entity")
        }
    }

    /// Marks an entity as deactivated, unlocking its components and disabling it from any systems.
    pub fn deactivate_entity(&mut self, entity: &Entity)
    {
        if self.is_valid(entity)
        {
            if !self.activated[**entity]
            {
                fail!("Entity already deactivated")
            }
            self.activated.set(**entity, false);
        }
        else
        {
            fail!("Tried to deactivate invalid entity")
        }
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        &self.entities[**entity] == entity && self.enabled[**entity]
    }

    /// Returns true if an entity is activated.
    #[inline]
    pub fn is_activated(&self, entity: &Entity) -> bool
    {
        self.is_valid(entity) && self.activated[**entity]
    }

    /// Deletes an entity from the manager.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        self.entities[mut][**entity] = Entity(0, Uuid::nil());
        self.enabled.set(**entity, false);
        self.activated.set(**entity, false);
        self.ids.return_id(**entity);
    }
}

struct IdPool
{
    recycled: Vec<uint>,
    next_id: uint,
}

impl IdPool
{
    pub fn new() -> IdPool
    {
        IdPool
        {
            recycled: Vec::new(),
            next_id: 1u,
        }
    }

    pub fn get_id(&mut self) -> uint
    {
        match self.recycled.pop()
        {
            Some(id) => id,
            None => {
                self.next_id += 1;
                self.next_id - 1
            }
        }
    }

    pub fn return_id(&mut self, id: uint)
    {
        self.recycled.push(id);
    }
}
