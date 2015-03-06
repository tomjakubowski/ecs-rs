
use std::collections::{HashMap, VecMap};
use std::ops::{Index, IndexMut};

use self::InnerComponentList::{Hot, Cold};

use {BuildData, EditData, ModifyData};
use Entity;
use ComponentManager;

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

    pub fn add(&mut self, entity: &BuildData, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(**entity.0, component),
            Cold(ref mut c) => c.insert(**entity.0, component),
        }
    }

    pub fn insert(&mut self, entity: &ModifyData, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(**entity.entity(), component),
            Cold(ref mut c) => c.insert(**entity.entity(), component),
        }
    }

    pub fn remove(&mut self, entity: &ModifyData) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.remove(entity.entity()),
            Cold(ref mut c) => c.remove(entity.entity()),
        }
    }

    pub fn set<U: EditData>(&mut self, entity: &U, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(**entity.entity(), component),
            Cold(ref mut c) => c.insert(**entity.entity(), component),
        }
    }

    pub fn get<U: EditData>(&self, entity: &U) -> Option<T> where T: Clone
    {
        match self.0
        {
            Hot(ref c) => c.get(entity.entity()).cloned(),
            Cold(ref c) => c.get(entity.entity()).cloned(),
        }
    }

    pub fn has<U: EditData>(&self, entity: &U) -> bool
    {
        match self.0
        {
            Hot(ref c) => c.contains_key(entity.entity()),
            Cold(ref c) => c.contains_key(entity.entity()),
        }
    }

    pub fn borrow<U: EditData>(&mut self, entity: &U) -> Option<&mut T>
    {
        match self.0
        {
            Hot(ref mut c) => c.get_mut(entity.entity()),
            Cold(ref mut c) => c.get_mut(entity.entity()),
        }
    }

    pub unsafe fn clear(&mut self, entity: &Entity)
    {
        match self.0
        {
            Hot(ref mut c) => c.remove(entity),
            Cold(ref mut c) => c.remove(entity),
        };
    }
}

impl<T: Component, U: EditData> Index<U> for ComponentList<T>
{
    type Output = T;
    fn index(&self, en: &U) -> &T
    {
        match self.0
        {
            Hot(ref c) => &c[**en.entity()],
            Cold(ref c) => &c[**en.entity()],
        }
    }
}

impl<T: Component, U: EditData> IndexMut<U> for ComponentList<T>
{
    fn index_mut(&mut self, en: &U) -> &mut T
    {
        match self.0
        {
            Hot(ref mut c) => &mut c[**en.entity()],
            Cold(ref mut c) => &mut c[**en.entity()],
        }
    }
}

pub trait EntityBuilder<T: ComponentManager>
{
    fn build<'a>(&mut self, BuildData<'a>, &mut T);
}

impl<T: ComponentManager, F> EntityBuilder<T> for F where F: FnMut(BuildData, &mut T)
{
    fn build(&mut self, e: BuildData, c: &mut T)
    {
        (*self)(e, c);
    }
}

impl<T: ComponentManager> EntityBuilder<T> for () { fn build(&mut self, _: BuildData, _: &mut T) {} }

pub trait EntityModifier<T: ComponentManager>
{
    fn modify<'a>(&mut self, ModifyData<'a>, &mut T);
}

impl<T: ComponentManager, F> EntityModifier<T> for F where F: FnMut(ModifyData, &mut T)
{
    fn modify(&mut self, e: ModifyData, c: &mut T)
    {
        (*self)(e, c);
    }
}

impl<T: ComponentManager> EntityModifier<T> for () { fn modify(&mut self, _: ModifyData, _: &mut T) {} }
