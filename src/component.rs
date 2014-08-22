
//! Store data in parts to allow defining different entities through composition.

use std::collections::Bitv;
use std::default::Default;

use buffer::{Buffer, Buffered};
use Entity;
use Phantom;

pub trait Component: Copy+Clone+Default
{
    /// Return the Component ID of a component type.
    fn cid(Phantom<Self>) -> ComponentId;

    // The user doesn't need to know about the Buffered trait.
    #[inline]
    #[doc(hidden)]
    fn stride(a: Phantom<Self>) -> uint
    {
        Buffered::stride(a)
    }
}

impl<T:Component> Buffered for T {}

pub type ComponentId = u64;

pub struct ComponentList
{
    buffer: Buffer,
    enabled: Bitv,
    id: ComponentId,
}

impl ComponentList
{
    pub fn new<T:Component>(a: Phantom<T>) -> ComponentList
    {
        ComponentList
        {
            buffer: Buffer::new(Component::stride(a)),
            enabled: Bitv::new(),
            id: Component::cid(a),
        }
    }

    pub fn new_stride(stride: uint, id: ComponentId) -> ComponentList
    {
        ComponentList
        {
            buffer: Buffer::new(stride),
            enabled: Bitv::new(),
            id: id,
        }
    }

    pub fn add<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            false
        }
        else if Component::cid(Phantom::<T>) != self.id
        {
            false
        }
        else
        {
            unsafe { self.buffer.set(**entity, component); }
            if **entity >= self.enabled.len()
            {
                let diff = **entity - self.enabled.len();
                self.enabled.grow(diff+1, false);
            }
            self.enabled.set(**entity, true);
            true
        }
    }

    pub fn set<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if **entity >= self.enabled.len() || !self.enabled.get(**entity)
        {
            false
        }
        else if Component::cid(Phantom::<T>) != self.id
        {
            false
        }
        else
        {
            unsafe { self.buffer.set(**entity, component); }
            true
        }
    }

    pub fn add_or_set<T:Component>(&mut self, entity: &Entity, component: &T) -> bool
    {
        if Component::cid(Phantom::<T>) != self.id
        {
            false
        }
        else
        {
            unsafe { self.buffer.set(**entity, component); }
            if **entity >= self.enabled.len()
            {
                let diff = **entity - self.enabled.len();
                self.enabled.grow(diff+1, false);
            }
            self.enabled.set(**entity, true);
            true
        }
    }

    pub fn has(&self, entity: &Entity) -> bool
    {
        **entity < self.enabled.len() && self.enabled.get(**entity)
    }

    pub fn get<T:Component>(&self, entity: &Entity) -> Option<T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            unsafe { self.buffer.get::<T>(**entity) }
        }
        else
        {
            None
        }
    }

    pub fn borrow<T:Component>(&self, entity: &Entity) -> Option<&T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            unsafe { self.buffer.borrow::<T>(**entity) }
        }
        else
        {
            None
        }
    }

    pub fn borrow_mut<T:Component>(&mut self, entity: &Entity) -> Option<&mut T>
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            unsafe { self.buffer.borrow_mut::<T>(**entity) }
        }
        else
        {
            None
        }
    }

    pub fn rm(&mut self, entity: &Entity) -> bool
    {
        if **entity < self.enabled.len() && self.enabled.get(**entity)
        {
            self.enabled.set(**entity, false);
            true
        }
        else
        {
            false
        }
    }

    pub fn get_cid(&self) -> ComponentId
    {
        self.id
    }
}
