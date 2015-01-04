
#![unstable]

//! Traits to observe and manage entities as they are changed in the world.

use std::any::{Any, AnyRefExt, AnyMutRefExt};
use std::cell::RefCell;
use std::intrinsics::TypeId;
use std::mem;
use std::raw::TraitObject;
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

impl<'a> AnyRefExt<'a> for &'a Manager {
    #[inline]
    fn is<T: 'static>(self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let boxed = self.get_type_id();

        // Compare both TypeIds on equality
        t == boxed
    }

    #[inline]
    fn downcast_ref<T: 'static>(self) -> Option<&'a T> {
        if self.is::<T>() {
            unsafe {
                // Get the raw representation of the trait object
                let to: TraitObject = mem::transmute(self);

                // Extract the data pointer
                Some(mem::transmute(to.data))
            }
        } else {
            None
        }
    }
}

impl<'a> AnyMutRefExt<'a> for &'a mut Manager {
    #[inline]
    fn downcast_mut<T: 'static>(self) -> Option<&'a mut T> {
        if self.is::<T>() {
            unsafe {
                // Get the raw representation of the trait object
                let to: TraitObject = mem::transmute(self);

                // Extract the data pointer
                Some(mem::transmute(to.data))
            }
        } else {
            None
        }
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
