
use std::borrow::BorrowFrom;
use std::collections::HashMap;
use std::hash::Hash;

use Entity;
use Manager;
use World;

pub struct GroupManager<Key: Hash+Eq+'static>
{
    groups: HashMap<Key, Vec<Entity>>,
}

impl<Key: Hash+Eq+'static> GroupManager<Key>
{
    pub fn new() -> GroupManager<Key>
    {
        GroupManager
        {
            groups: HashMap::new(),
        }
    }
    
    pub fn get<Sized? Q>(&self, key: &Q) -> Option<&Vec<Entity>>
        where Q: Hash+Eq+BorrowFrom<Key>
    {
        self.groups.get(key)
    }
}

impl<Key: Hash+Eq+'static> Index<Key, Vec<Entity>> for GroupManager<Key>
{
    fn index(&self, i: &Key) -> &Vec<Entity>
    {
        &self.groups[*i]
    }
}

impl<Key: Hash+Eq+'static> IndexMut<Key, Vec<Entity>> for GroupManager<Key>
{
    fn index_mut(&mut self, i: &Key) -> &mut Vec<Entity>
    {
        &mut self.groups[*i]
    }
}

impl<Key: Hash+Eq+'static> Manager for GroupManager<Key>
{
    fn activated(&mut self, _: &Entity, _: &World) {}
    
    fn reactivated(&mut self, _: &Entity, _: &World) {}

    fn deactivated(&mut self, entity: &Entity, _: &World)
    {
        for (_, ref mut vec) in self.groups.iter_mut()
        {
            vec.retain(|e| *e != *entity);
        }
    }
}
