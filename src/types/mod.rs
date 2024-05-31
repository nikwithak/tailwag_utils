// Arc<HashMap<TypeId, Arc<Box<dyn Any + Send + Sync>>>>
// WIP

pub mod generic_type_map {
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    #[derive(Default)]
    pub struct TypeInstanceMap {
        data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    }

    impl TypeInstanceMap {
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.data.get(&TypeId::of::<T>()).map(|t| {
                t.downcast_ref::<T>().unwrap_or_else(|| panic!("Found TypeId {:?} in map, but it did not downcast to a object with the same TypeId",&TypeId::of::<T>()))
            })
        }
        pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
            self.data.get_mut(&TypeId::of::<T>()).map(|t| {
                t.downcast_mut::<T>().unwrap_or_else(|| panic!("Found TypeId {:?} in map, but it did not downcast to a object with the same TypeId",&TypeId::of::<T>()))
            })
        }

        pub fn insert<T: 'static + Send + Sync>(
            &mut self,
            t: T,
        ) {
            self.data.insert(TypeId::of::<T>(), Box::new(t));
        }
    }

    pub type CloneableTypeInstanceMap = TypeInstanceMap;
}
