
#![experimental]

//! Traits to observe and manage entities as they are changed in the world.

use std::cell::RefCell;
use std::rc::Rc;

use Entity;
use World;

/// Mutable manager
pub trait MutableManager
{
    /// Called when an entity is added to the world.
    fn added(&mut self, &Entity, &World);
    /// Called when an entity is removed from the world.
    fn removed(&mut self, &Entity, &World);
    /// Called when an entity is activated in the world.
    fn activated(&mut self, &Entity, &World);
    /// Called when an entity is deactivated in the world.
    fn deactivated(&mut self, &Entity, &World);
}

/// Immutable manager
pub trait Manager
{
    /// Called when an entity is added to the world.
    fn added(&self, &Entity, &World);
    /// Called when an entity is removed from the world.
    fn removed(&self, &Entity, &World);
    /// Called when an entity is activated in the world.
    fn activated(&self, &Entity, &World);
    /// Called when an entity is deactivated in the world.
    fn deactivated(&self, &Entity, &World);
}

impl<T: MutableManager> MutableManager for Rc<RefCell<T>>
{
    fn added(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().added(e, w)
    }

    fn removed(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().removed(e, w)
    }

    fn activated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().activated(e, w)
    }

    fn deactivated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().deactivated(e, w)
    }
}
