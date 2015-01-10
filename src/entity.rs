
//! Entity identifier and manager types.

use std::collections::{VecMap};
use std::default::Default;
use std::ops::Deref;

use Components;

pub type Id = u64;

/// Dual identifier for an entity.
///
/// The first element (usize) is the entity's index, used to locate components.
/// This value can be recycled, so the second element (Uuid) is used as an identifier.
#[stable]
#[derive(Copy, Clone, Eq, Hash, PartialEq, Show)]
pub struct Entity(usize, Id);

#[stable]
impl Entity
{
    #[stable]
    pub fn nil() -> Entity
    {
        Entity(0, 0)
    }

    /// Returns the entity's index.
    #[stable]
    #[inline]
    pub fn get_index(&self) -> usize
    {
        self.0.clone()
    }

    /// Returns the entity's unique identifier.
    #[stable]
    #[inline]
    pub fn get_id(&self) -> Id
    {
        self.1.clone()
    }
}

#[stable]
impl Default for Entity
{
    #[stable]
    fn default() -> Entity
    {
        Entity::nil()
    }
}

#[unstable="Used internally and subject to change"]
impl Deref for Entity
{
    type Target = usize;

    #[inline]
    fn deref(&self) -> &usize
    {
        &self.0
    }
}

pub trait EntityBuilder: 'static
{
    fn build(&mut self, &mut Components, Entity);
}

impl<F: 'static> EntityBuilder for F where F: FnMut(&mut Components, Entity)
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

impl<F: 'static> EntityModifier for F where F: FnMut(&mut Components, Entity)
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
    entities: VecMap<Entity>,
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
            entities: VecMap::new(),
            next_id: 0,
        }
    }

    pub fn clear(&mut self) -> Vec<Entity>
    {
        self.entities.into_iter().map(|(_, val)| val).collect()
    }

    pub fn count(&self) -> usize
    {
        self.indexes.count()
    }

    /// Creates a new `Entity`, assigning it the first available identifier.
    pub fn create_entity(&mut self) -> Entity
    {
        self.next_id += 1;
        let ret = Entity(self.indexes.get_id(), self.next_id);
        self.entities.insert(*ret, ret.clone());
        ret
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        self.entities.contains_key(&**entity)
    }

    /// Deletes an entity from the manager.
    pub fn delete_entity(&mut self, entity: &Entity)
    {
        self.entities.remove(&**entity);
        self.indexes.return_id(**entity);
    }
}

struct IndexPool
{
    recycled: Vec<usize>,
    next_index: usize,
}

impl IndexPool
{
    pub fn new() -> IndexPool
    {
        IndexPool
        {
            recycled: Vec::new(),
            next_index: 1us,
        }
    }

    pub fn count(&self) -> usize
    {
        self.next_index - self.recycled.len()
    }

    pub fn get_id(&mut self) -> usize
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

    pub fn return_id(&mut self, id: usize)
    {
        self.recycled.push(id);
    }
}
