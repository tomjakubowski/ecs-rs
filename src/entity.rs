
//! Entity identifier and manager types.

use std::collections::hash_set::{HashSet, Iter, Drain};
use std::default::Default;
use std::ops::Deref;

use Aspect;
use ComponentManager;
use EntityData;

pub type Id = u64;

/// Dual identifier for an entity.
///
/// The first element (usize) is the entity's index, used to locate components.
/// This value can be recycled, so the second element (u64) is used as an identifier.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity(usize, Id);

impl Entity
{
    pub fn nil() -> Entity
    {
        Entity(0, 0)
    }

    /// Returns the entity's index.
    #[inline]
    pub fn get_index(&self) -> usize
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

impl Deref for Entity
{
    type Target = usize;

    #[inline]
    fn deref(&self) -> &usize
    {
        &self.0
    }
}

pub struct EntityIter<'a, T: ComponentManager>
{
    inner: Iter<'a, Entity>,
}

pub struct FilteredEntityIter<'a, T: ComponentManager>
{
    inner: EntityIter<'a, T>,
    aspect: Aspect<T>,
    components: &'a T,
}

impl<'a, T: ComponentManager> EntityIter<'a, T>
{
    pub fn new(iter: Iter<'a, Entity>) -> EntityIter<'a, T>
    {
        EntityIter
        {
            inner: iter,
        }
    }

    pub fn filter(self, aspect: Aspect<T>, components: &'a T) -> FilteredEntityIter<'a, T>
    {
        FilteredEntityIter
        {
            inner: self,
            aspect: aspect,
            components: components,
        }
    }
}

impl<'a, T: ComponentManager> Iterator for EntityIter<'a, T>
{
    type Item = EntityData<'a, T>;
    fn next(&mut self) -> Option<EntityData<'a, T>>
    {
        self.inner.next().map(|x| EntityData(x))
    }
}

impl<'a, T: ComponentManager> Iterator for FilteredEntityIter<'a, T>
{
    type Item = EntityData<'a, T>;
    fn next(&mut self) -> Option<EntityData<'a, T>>
    {
        for x in self.inner.by_ref()
        {
            if self.aspect.check(&x, self.components)
            {
                return Some(x);
            }
            else
            {
                continue
            }
        }
        None
    }
}

/// Handles creation, activation, and validating of entities.
#[doc(hidden)]
pub struct EntityManager
{
    indices: IndexPool,
    entities: HashSet<Entity>,
    next_id: Id,
}

impl EntityManager
{
    /// Returns a new `EntityManager`
    pub fn new() -> EntityManager
    {
        EntityManager
        {
            indices: IndexPool::new(),
            entities: HashSet::new(),
            next_id: 0,
        }
    }

    pub fn iter<T: ComponentManager>(&self) -> EntityIter<T>
    {
        EntityIter::new(self.entities.iter())
    }

    pub fn drain(&mut self) -> Drain<Entity>
    {
        self.entities.drain()
    }

    pub fn count(&self) -> usize
    {
        self.indices.count()
    }

    /// Creates a new `Entity`, assigning it the first available index.
    pub fn create(&mut self) -> Entity
    {
        self.next_id += 1;
        let ret = Entity(self.indices.get_index(), self.next_id);
        self.entities.insert(ret.clone());
        ret
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: &Entity) -> bool
    {
        self.entities.contains(entity)
    }

    /// Deletes an entity from the manager.
    pub fn remove(&mut self, entity: &Entity)
    {
        self.entities.remove(entity);
        self.indices.return_id(**entity);
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

    pub fn get_index(&mut self) -> usize
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
