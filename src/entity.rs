
//! Entity identifier and manager types.

use std::collections::Bitv;
use std::default::Default;
use uuid::Uuid;

use Components;

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
    pub fn nil() -> Entity
    {
        Entity(0, Uuid::nil())
    }

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
        let &Entity(ref i, _) = self;
        i
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
    ids: IdPool,
    entities: Vec<Entity>,
    enabled: Bitv,
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
        }
    }

    pub fn count(&self) -> uint
    {
        self.ids.count()
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
        self.entities[*ret] = ret;

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
        self.entities[**entity] = Entity(0, Uuid::nil());
        self.enabled.set(**entity, false);
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

    pub fn count(&self) -> uint
    {
        self.next_id - self.recycled.len()
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
