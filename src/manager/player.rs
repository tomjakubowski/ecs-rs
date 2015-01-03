
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

    pub fn add(&mut self, player: uint, entity: Entity) -> Option<Entity>
    {
        self.players.insert(player, entity)
    }

    pub fn get(&self, player: uint) -> Option<&Entity>
    {
        self.players.get(&player)
    }

    pub fn remove(&mut self, player: uint) -> Option<Entity>
    {
        self.players.remove(&player)
    }
}

impl Index<uint, Entity> for PlayerManager
{
    fn index(&self, i: &uint) -> &Entity
    {
        &self.players[*i]
    }
}

impl IndexMut<uint, Entity> for PlayerManager
{
    fn index_mut(&mut self, i: &uint) -> &mut Entity
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
