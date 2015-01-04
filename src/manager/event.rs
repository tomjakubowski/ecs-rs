
use std::borrow::BorrowFrom;
use std::collections::{HashMap, RingBuf};
use std::hash::Hash;

use Manager;

pub trait Key: Hash+Eq+'static {}
impl<T: Hash+Eq+'static> Key for T {}

pub struct StateManager<Event, State: 'static>
{
    states: HashMap<Event, State>,
}

impl<E: Key, S: 'static> StateManager<E, S>
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

    pub fn get<Sized? Q>(&self, event: &Q) -> Option<&S>
        where Q: Hash+Eq+BorrowFrom<E>
    {
        self.states.get(event)
    }

    pub fn clear<Sized? Q>(&mut self, event: &Q) -> Option<S>
        where Q: Hash+Eq+BorrowFrom<E>
    {
        self.states.remove(event)
    }

    pub fn clear_all(&mut self)
    {
        self.states.clear()
    }
}

impl<E: Key, S: 'static> Manager for StateManager<E, S>
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
