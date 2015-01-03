
use std::borrow::BorrowFrom;
use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use Entity;
use Manager;
use World;

pub trait GroupKey: Hash+Eq+'static {}
impl<T: Hash+Eq+'static> GroupKey for T {}

pub struct GroupManager<Key: GroupKey>
{
    groups: HashMap<Key, Vec<Entity>>,
}

impl<Key: GroupKey> GroupManager<Key>
{
    pub fn new() -> GroupManager<Key>
    {
        GroupManager
        {
            groups: HashMap::new(),
        }
    }

    pub fn create(&mut self, key: Key)
    {
        match self.groups.entry(key)
        {
            Entry::Vacant(entry) => {
                entry.set(Vec::new());
            },
            _ => (),
        }
    }

    pub fn get<Sized? Q>(&self, key: &Q) -> Option<&Vec<Entity>>
        where Q: Hash+Eq+BorrowFrom<Key>
    {
        self.groups.get(key)
    }

    pub fn delete<Sized? Q>(&mut self, key: &Q) -> Option<Vec<Entity>>
        where Q: Hash+Eq+BorrowFrom<Key>
    {
        self.groups.remove(key)
    }
}

impl<Key: GroupKey> Index<Key, Vec<Entity>> for GroupManager<Key>
{
    fn index(&self, i: &Key) -> &Vec<Entity>
    {
        &self.groups[*i]
    }
}

impl<Key: GroupKey> IndexMut<Key, Vec<Entity>> for GroupManager<Key>
{
    fn index_mut(&mut self, i: &Key) -> &mut Vec<Entity>
    {
        &mut self.groups[*i]
    }
}

impl<Key: GroupKey> Manager for GroupManager<Key>
{
    fn deactivated(&mut self, entity: &Entity, _: &World)
    {
        for (_, ref mut vec) in self.groups.iter_mut()
        {
            vec.retain(|e| *e != *entity);
        }
    }
}
