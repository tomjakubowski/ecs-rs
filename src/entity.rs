
//! Entity identifier and manager types.

use std::collections::Bitv;
use std::default::Default;

use Components;

pub type Id = u64;

/// Dual identifier for an entity.
///
/// The first element (uint) is the entity's index, used to locate components.
/// This value can be recycled, so the second element (Uuid) is used as an identifier.
#[stable]
#[deriving(Copy, Clone, Eq, Hash, PartialEq, Show)]
pub struct Entity(uint, Id);

#[stable]
impl Entity
{
    pub fn nil() -> Entity
    {
        Entity(0, 0)
    }

    /// Returns the entity's index.
    #[inline]
    pub fn get_index(&self) -> uint
    {
        self.0.clone()
    }

    /// Returns the entity's unique identifier.
    #[inline]
    pub fn get_id(&self) -> Id
    {
        self.1.clone()
    }
}

impl Default for Entity
{
    fn default() -> Entity
    {
        Entity::nil()
    }
}

impl Deref<uint> for Entity
{
    #[inline]
    fn deref(&self) -> &uint
    {
        &self.0
    }
}

pub trait EntityBuilder: 'static
{
    fn build(&mut self, &mut Components, Entity);
}

impl EntityBuilder for |&mut Components, Entity|: 'static
{
    fn build(&mut self, c: &mut Components, e: Entity)
    {
        (*self)(c, e);
    }
}

impl EntityBuilder for () { fn build(&mut self, _: &mut Components, _: Entity) {} }

pub trait EntityModifier: 'static
{
    fn modify(&mut self, &mut Components, Entity);
}

impl EntityModifier for |&mut Components, Entity|: 'static
{
    fn modify(&mut self, c: &mut Components, e: Entity)
    {
        (*self)(c, e);
    }
}

impl EntityModifier for () { fn modify(&mut self, _: &mut Components, _: Entity) {} }

/// Handles creation, activation, and validating of entities.
#[doc(hidden)]
pub struct EntityManager
{
    indexes: IndexPool,
    entities: Vec<Entity>,
    enabled: Bitv,
    next_id: Id,
}

impl EntityManager
{
    /// Returns a new `EntityManager`
    pub fn new() -> EntityManager
    {
        EntityManager
        {
            indexes: IndexPool::new(),
            entities: Vec::new(),
            enabled: Bitv::new(),
            next_id: 0,
        }
    }

    pub fn clear(&mut self) -> Vec<Entity>
    {
        let mut vec = Vec::new();
        ::std::mem::swap(&mut vec, &mut self.entities);
        vec.retain(|e| self.enabled[**e]);
        self.enabled = Bitv::new();
        self.indexes = IndexPool::new();
        vec
    }

    pub fn count(&self) -> uint
    {
        self.indexes.count()
    }

    /// Creates a new `Entity`, assigning it the first available identifier.
    pub fn create_entity(&mut self) -> Entity
    {
        self.next_id += 1;
        let ret = Entity(self.indexes.get_id(), self.next_id);
        if *ret >= self.entities.len()
        {
            let diff = *ret - self.entities.len();
            self.entities.grow(diff+1, Entity::nil());
        }
        self.entities[*ret] = ret.clone();

        if *ret >= self.enabled.len()
        {
            let diff = *ret - self.enabled.len();
            self.enabled.grow(diff+1, false);
        }
        self.enabled.set(*ret, true);
        ret
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        &self.entities[**entity] == entity && self.enabled[**entity]
    }

    /// Deletes an entity from the manager.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        self.entities[**entity] = Entity::nil();
        self.enabled.set(**entity, false);
        self.indexes.return_id(**entity);
    }
}

struct IndexPool
{
    recycled: Vec<uint>,
    next_index: uint,
}

impl IndexPool
{
    pub fn new() -> IndexPool
    {
        IndexPool
        {
            recycled: Vec::new(),
            next_index: 1u,
        }
    }

    pub fn count(&self) -> uint
    {
        self.next_index - self.recycled.len()
    }

    pub fn get_id(&mut self) -> uint
    {
        match self.recycled.pop()
        {
            Some(id) => id,
            None => {
                self.next_index += 1;
                self.next_index - 1
            }
        }
    }

    pub fn return_id(&mut self, id: uint)
    {
        self.recycled.push(id);
    }
}
