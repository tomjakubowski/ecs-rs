
use std::collections::{HashMap};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use self::InnerComponentList::{Hot, Cold};

use {BuildData, EditData, ModifyData};
use {IndexedEntity};
use ComponentManager;

pub trait Component: 'static {}

impl<T:'static> Component for T {}

pub struct ComponentList<C: ComponentManager, T: Component>(InnerComponentList<T>, PhantomData<fn(C)>);

enum InnerComponentList<T: Component>
{
    Hot(HashMap<usize, T>),
    Cold(HashMap<usize, T>),
}

impl<C: ComponentManager, T: Component> ComponentList<C, T>
{
    pub fn hot() -> ComponentList<C, T>
    {
        ComponentList(Hot(HashMap::new()), PhantomData)
    }

    pub fn cold() -> ComponentList<C, T>
    {
        ComponentList(Cold(HashMap::new()), PhantomData)
    }

    pub fn add(&mut self, entity: &BuildData<C>, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(entity.0.index(), component),
            Cold(ref mut c) => c.insert(entity.0.index(), component),
        }
    }

    pub fn insert(&mut self, entity: &ModifyData<C>, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(entity.entity().index(), component),
            Cold(ref mut c) => c.insert(entity.entity().index(), component),
        }
    }

    pub fn remove(&mut self, entity: &ModifyData<C>) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.remove(&entity.entity().index()),
            Cold(ref mut c) => c.remove(&entity.entity().index()),
        }
    }

    pub fn set<U: EditData<C>>(&mut self, entity: &U, component: T) -> Option<T>
    {
        match self.0
        {
            Hot(ref mut c) => c.insert(entity.entity().index(), component),
            Cold(ref mut c) => c.insert(entity.entity().index(), component),
        }
    }

    pub fn get<U: EditData<C>>(&self, entity: &U) -> Option<T> where T: Clone
    {
        match self.0
        {
            Hot(ref c) => c.get(&entity.entity().index()).cloned(),
            Cold(ref c) => c.get(&entity.entity().index()).cloned(),
        }
    }

    pub fn has<U: EditData<C>>(&self, entity: &U) -> bool
    {
        match self.0
        {
            Hot(ref c) => c.contains_key(&entity.entity().index()),
            Cold(ref c) => c.contains_key(&entity.entity().index()),
        }
    }

    pub fn borrow<U: EditData<C>>(&mut self, entity: &U) -> Option<&mut T>
    {
        match self.0
        {
            Hot(ref mut c) => c.get_mut(&entity.entity().index()),
            Cold(ref mut c) => c.get_mut(&entity.entity().index()),
        }
    }

    pub unsafe fn clear(&mut self, entity: &IndexedEntity<C>)
    {
        match self.0
        {
            Hot(ref mut c) => c.remove(&entity.index()),
            Cold(ref mut c) => c.remove(&entity.index()),
        };
    }
}

impl<C: ComponentManager, T: Component, U: EditData<C>> Index<U> for ComponentList<C, T>
{
    type Output = T;
    fn index(&self, en: U) -> &T
    {
        match self.0
        {
            Hot(ref c) => &c[&en.entity().index()],
            Cold(ref c) => &c[&en.entity().index()],
        }
    }
}

impl<C: ComponentManager, T: Component, U: EditData<C>> IndexMut<U> for ComponentList<C, T>
{
    fn index_mut(&mut self, en: U) -> &mut T
    {
        match self.0
        {
            Hot(ref mut c) => c.get_mut(&en.entity().index()),
            Cold(ref mut c) => c.get_mut(&en.entity().index()),
        }.expect(&format!("Could not find entry for {:?}", **en.entity()))
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
