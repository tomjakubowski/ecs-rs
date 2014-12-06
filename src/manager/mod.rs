
#![unstable]

//! Traits to observe and manage entities as they are changed in the world.

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use Entity;
use World;

pub use self::event::{QueueManager, StateManager};
pub use self::group::GroupManager;
pub use self::player::PlayerManager;

pub mod event;
pub mod group;
pub mod player;

/// Mutable manager
pub trait Manager: Any
{
    fn as_any(&self) -> &Any
    {
        self as &Any
    }
    
    fn as_any_mut(&mut self) -> &mut Any
    {
        self as &mut Any
    }
    
    /// Called when an entity is added to the world.
    fn activated(&mut self, _: &Entity, _: &World)
    {
        
    }
    
    /// Called when an entity is modified in the world.
    fn reactivated(&mut self, _: &Entity, _: &World)
    {
        
    }
    
    /// Called when an entity is removed from the world.
    fn deactivated(&mut self, _: &Entity, _: &World)
    {
        
    }
}

impl<T: Manager> Manager for Rc<RefCell<T>>
{
    fn activated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().activated(e, w)
    }

    fn reactivated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().reactivated(e, w)
    }

    fn deactivated(&mut self, e: &Entity, w: &World)
    {
        self.borrow_mut().deactivated(e, w)
    }
}
