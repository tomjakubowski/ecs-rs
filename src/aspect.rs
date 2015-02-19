
use {ComponentManager, EntityData};

pub struct Aspect<T: ComponentManager>(Box<Fn(&EntityData<T>, &T) -> bool + 'static>);

impl<T: ComponentManager> Aspect<T>
{
    pub fn all() -> Aspect<T>
    {
        Aspect(box |_, _| true)
    }

    pub fn none() -> Aspect<T>
    {
        Aspect(box |_, _| false)
    }

    pub unsafe fn new(inner: Box<Fn(&EntityData<T>, &T) -> bool + 'static>) -> Aspect<T>
    {
        Aspect(inner)
    }

    pub fn check<'a>(&self, entity: &EntityData<'a, T>, components: &T) -> bool
    {
        (self.0)(entity, components)
    }
}
