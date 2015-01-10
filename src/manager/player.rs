
use std::collections::VecMap;
use std::ops::{Index, IndexMut};

use Aspect;
use Entity;
use Manager;
use World;

pub struct PlayerManager
{
    players: VecMap<Entity>,
    aspect: Aspect,
}

impl PlayerManager
{
    pub fn new() -> PlayerManager
    {
        PlayerManager
        {
            players: VecMap::new(),
            aspect: Aspect::nil(),
        }
    }

    pub fn with_aspect(aspect: Aspect) -> PlayerManager
    {
        PlayerManager
        {
            players: VecMap::new(),
            aspect: aspect,
        }
    }

    pub fn add(&mut self, player: usize, entity: Entity) -> Option<Entity>
    {
        self.players.insert(player, entity)
    }

    pub fn get(&self, player: usize) -> Option<&Entity>
    {
        self.players.get(&player)
    }

    pub fn remove(&mut self, player: usize) -> Option<Entity>
    {
        self.players.remove(&player)
    }
}

impl Index<usize> for PlayerManager
{
    type Output = Entity;
    fn index(&self, i: &usize) -> &Entity
    {
        &self.players[*i]
    }
}

impl IndexMut<usize> for PlayerManager
{
    type Output = Entity;
    fn index_mut(&mut self, i: &usize) -> &mut Entity
    {
        &mut self.players[*i]
    }
}

impl Manager for PlayerManager
{
    fn reactivated(&mut self, entity: &Entity, w: &World)
    {
        let mut r = Vec::new();
        for (i, e) in self.players.iter()
        {
            if *e == *entity && !self.aspect.check(entity, w)
            {
                r.push(i);
            }
        }
        for i in r.iter()
        {
            self.players.remove(i);
        }
    }

    fn deactivated(&mut self, entity: &Entity, _: &World)
    {
        let mut r = Vec::new();
        for (i, e) in self.players.iter()
        {
            if *e == *entity
            {
                r.push(i);
            }
        }
        for i in r.iter()
        {
            self.players.remove(i);
        }
    }
}
