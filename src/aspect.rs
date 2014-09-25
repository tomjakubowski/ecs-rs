
//! A filter for entities based on their components.

use ComponentId;
use Entity;
use World;

/// A filter for entities base on their components.
#[deriving(Clone)]
pub struct Aspect
{
    all: Vec<ComponentId>,
    any: Vec<ComponentId>,
    none: Vec<ComponentId>,
}

impl Aspect
{
    /// Create an empty `Aspect` that accepts all entities.
    pub fn nil() -> Aspect
    {
        Aspect
        {
            all: Vec::new(),
            any: Vec::new(),
            none: Vec::new(),
        }
    }

    /// Create an `Aspect` requiring that all the specifed `Component`s are found.
    pub fn for_all(vec: Vec<ComponentId>) -> Aspect
    {
        Aspect
        {
            all: vec,
            any: Vec::new(),
            none: Vec::new(),
        }
    }

    /// Create an `Aspect` requiring that at least one of the specified `Component`s is found.
    pub fn for_any(vec: Vec<ComponentId>) -> Aspect
    {
        Aspect
        {
            all: Vec::new(),
            any: vec,
            none: Vec::new(),
        }
    }

    /// Create an `Aspect` requiring that none the specified `Component`s are found.
    pub fn for_none(vec: Vec<ComponentId>) -> Aspect
    {
        Aspect
        {
            all: Vec::new(),
            any: Vec::new(),
            none: vec,
        }
    }

    /// Add a requirement for all the specified `Component`s to be found.
    pub fn with_all(mut self, vec: Vec<ComponentId>) -> Aspect
    {
        self.all.extend(vec.into_iter());
        self
    }

    /// Add a requirement for at least one of the specified `Component`s to be found.
    pub fn with_any(mut self, vec: Vec<ComponentId>) -> Aspect
    {
        self.any.extend(vec.into_iter());
        self
    }

    /// Add a requirement for none of the specified `Component`s to be found.
    pub fn with_none(mut self, vec: Vec<ComponentId>) -> Aspect
    {
        self.none.extend(vec.into_iter());
        self
    }

    /// Test if an `Entity` fulfills the requirements of this `Aspect`.
    pub fn check(&self, entity: &Entity, world: &World) -> bool
    {
        (self.all.is_empty() || self.all.iter().all(|id| world.has_component(entity, *id)))
        && !self.none.iter().any(|id| world.has_component(entity, *id))
        && (self.any.is_empty() || self.any.iter().any(|id| world.has_component(entity, *id)))
    }
}
