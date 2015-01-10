
use std::borrow::BorrowFrom;
use std::collections::{RingBuf};
use std::collections::hash_map::{HashMap, Hasher};
use std::hash::Hash;

use Manager;

pub trait StateKey: Hash<Hasher>+Eq+'static {}
impl<T: Hash<Hasher>+Eq+'static> StateKey for T {}

pub struct StateManager<Event, State: 'static>
{
    states: HashMap<Event, State>,
}

impl<E: StateKey, S: 'static> StateManager<E, S>
{
    pub fn new() -> StateManager<E, S>
    {
        StateManager
        {
            states: HashMap::new(),
        }
    }

    pub fn set(&mut self, event: E, state: S) -> Option<S>
    {
        self.states.insert(event, state)
    }

    pub fn get<Q: ?Sized>(&self, event: &Q) -> Option<&S>
        where Q: StateKey+BorrowFrom<E>
    {
        self.states.get(event)
    }

    pub fn clear<Q: ?Sized>(&mut self, event: &Q) -> Option<S>
        where Q: StateKey+BorrowFrom<E>
    {
        self.states.remove(event)
    }

    pub fn clear_all(&mut self)
    {
        self.states.clear()
    }
}

impl<E: StateKey, S: 'static> Manager for StateManager<E, S>
{

}

pub struct QueueManager<Event: 'static>
{
    queue: RingBuf<Event>,
}

impl<E: 'static> QueueManager<E>
{
    pub fn new() -> QueueManager<E>
    {
        QueueManager
        {
            queue: RingBuf::new(),
        }
    }

    pub fn push(&mut self, event: E)
    {
        self.queue.push_back(event);
    }

    pub fn pop(&mut self) -> Option<E>
    {
        self.queue.pop_front()
    }

    pub fn peek(&self) -> Option<&E>
    {
        self.queue.front()
    }

    pub fn modify(&mut self) -> Option<&mut E>
    {
        self.queue.front_mut()
    }

    pub fn is_empty(&self) -> bool
    {
        self.queue.is_empty()
    }
}

impl<E: 'static> Manager for QueueManager<E>
{

}
