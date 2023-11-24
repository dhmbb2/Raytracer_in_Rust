use crate::hittable::Hittable;


pub struct World {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add_hittable<T:>(&mut self, obj: T)
    where
        T: Hittable + 'static
    {
        self.hittables.push(Box::<T>::new(obj));
    }
}