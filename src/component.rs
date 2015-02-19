
use std::collections::{HashMap, VecMap};
use std::ops::{Index, IndexMut};

use self::InnerComponentList::{Hot, Cold};

use {BuildData, ModifyData};
use ComponentManager;
use Entity;

pub trait Component: 'static {}

impl<T:'static> Component for T {}

pub struct ComponentList<T: Component>(InnerComponentList<T>);

enum InnerComponentList<T: Component>
{
    Hot(VecMap<T>),
    Cold(HashMap<usize, T>),
}

impl<T: Component> ComponentList<T>
{
    pub fn hot() -> ComponentList<T>
    {
        ComponentList(Hot(VecMap::new()))
    }

    pub fn cold() -> ComponentList<T>
    {
        ComponentList(Cold(HashMap::new()))
    }

    pub unsafe fn insert(&mut self, entity: &Entity, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(**entity, component),
            Cold(ref mut c) => c.insert(**entity, component),
        }
    }

    pub unsafe fn remove(&mut self, entity: &Entity) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.remove(&**entity),
            Cold(ref mut c) => c.remove(&**entity),
        }
    }

    pub unsafe fn get(&self, entity: &Entity) -> Option<T> where T: Clone
    {
        match self.0
        {
            Hot(ref c) => c.get(&**entity).cloned(),
            Cold(ref c) => c.get(&**entity).cloned(),
        }
    }

    pub unsafe fn has(&self, entity: &Entity) -> bool
    {
        match self.0
        {
            Hot(ref c) => c.contains_key(&**entity),
            Cold(ref c) => c.contains_key(&**entity),
        }
    }

    pub unsafe fn borrow(&mut self, entity: &Entity) -> Option<&mut T>
    {
        match self.0
        {
            Hot(ref mut c) => c.get_mut(&**entity),
            Cold(ref mut c) => c.get_mut(&**entity),
        }
    }
}

pub trait EntityBuilder<T: ComponentManager>
{
    fn build<'a>(&mut self, BuildData<'a, T>, &mut T);
}

impl<T: ComponentManager, F> EntityBuilder<T> for F where F: FnMut(BuildData<T>, &mut T)
{
    fn build(&mut self, e: BuildData<T>, c: &mut T)
    {
        (*self)(e, c);
    }
}

impl<T: ComponentManager> EntityBuilder<T> for () { fn build(&mut self, _: BuildData<T>, _: &mut T) {} }

pub trait EntityModifier<T: ComponentManager>
{
    fn modify<'a>(&mut self, ModifyData<'a, T>, &mut T);
}

impl<T: ComponentManager, F> EntityModifier<T> for F where F: FnMut(ModifyData<T>, &mut T)
{
    fn modify(&mut self, e: ModifyData<T>, c: &mut T)
    {
        (*self)(e, c);
    }
}

impl<T: ComponentManager> EntityModifier<T> for () { fn modify(&mut self, _: ModifyData<T>, _: &mut T) {} }

impl<T: Component> Index<usize> for ComponentList<T>
{
    type Output = T;
    fn index(&self, index: &usize) -> &T
    {
        match self.0
        {
            Hot(ref c) => &c[*index],
            Cold(ref c) => &c[*index],
        }
    }
}

impl<T: Component> IndexMut<usize> for ComponentList<T>
{
    fn index_mut(&mut self, index: &usize) -> &mut T
    {
        match self.0
        {
            Hot(ref mut c) => &mut c[*index],
            Cold(ref mut c) => &mut c[*index],
        }
    }
}

impl<T: Component> Index<Entity> for ComponentList<T>
{
    type Output = T;
    fn index(&self, index: &Entity) -> &T
    {
        match self.0
        {
            Hot(ref c) => &c[**index],
            Cold(ref c) => &c[**index],
        }
    }
}

impl<T: Component> IndexMut<Entity> for ComponentList<T>
{
    fn index_mut(&mut self, index: &Entity) -> &mut T
    {
        match self.0
        {
            Hot(ref mut c) => &mut c[**index],
            Cold(ref mut c) => &mut c[**index],
        }
    }
}
