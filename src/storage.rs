#[allow(unused)]
use std::collections::HashMap;

use hibitset::BitSet;

use IDType;

pub trait Storage<T>  where T: Clone {
    fn new() -> Self;

    fn add(&mut self, id: IDType, to_add: T) -> Option<T>;
    fn remove(&mut self, id: IDType) -> Option<T>;
    fn get(&self, id: IDType) -> Option<T>;

    fn contains(&self, id: IDType) -> bool;
}

#[derive(Clone)]
pub struct VectorStorage<T> {
    pub data: Vec<Option<T>>,
    // pub mask: Vec<bool>,
    pub mask: BitSet,
}

impl<T> Storage<T> for VectorStorage<T> where T: Clone{
    fn new() -> Self {
        Self{
            data: Vec::<Option<T>>::new(),
            //mask: Vec::<bool>::new(),
            mask: BitSet::new(),
        }
    }

    fn add(&mut self, id: IDType, to_add: T) -> Option<T> {
        if id as usize >= self.data.len() {
            self.data.resize( id as usize + 1, None );
        }

        let ret = self.data[id as usize].take();
        self.data[id as usize] = Some(to_add);

        self.mask.add(id as u32);

        return ret;
    }

    fn remove(&mut self, id: IDType) -> Option<T> {
        self.mask.remove(id as u32);
        if id as usize >= self.data.len() {
            return None;
        }

        let ret = self.data[id as usize].take();
        return ret;
    }

    fn get(&self, id: IDType) -> Option<T> {
        if id as usize >= self.data.len() {
            return None;
        }

        match self.data[id as usize] {
            Some(ref val) => return Some(val.clone()),
            None => return None,
        }
    }

    fn contains(&self, id: IDType) -> bool {
        self.mask.contains(id as u32)
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct NullStorage {
    mask: BitSet,
}

#[derive(Clone)]
pub struct HashStorage<T> {
    pub data: HashMap<IDType, T>,
    // pub mask: Vec<bool>,
    pub mask: BitSet,
}

impl<T> Storage<T> for HashStorage<T> where T: Clone{
    fn new() -> Self {
        Self{
            data: HashMap::<IDType,T>::new(),
            //mask: Vec::<bool>::new(),
            mask: BitSet::new(),
        }
    }

    fn add(&mut self, id: IDType, to_add: T) -> Option<T> {
        let ret = self.data.remove(&id);
        self.data.insert(id,to_add);
        self.mask.add(id as u32);

        return ret;
    }

    fn remove(&mut self, id: IDType) -> Option<T> {
        self.mask.remove(id as u32);
        self.data.remove(&id)
    }

    fn get(&self, id: IDType) -> Option<T> {
        if ! self.contains(id) {
            return None;
        }

        if let Some(val) = self.data.get(&id) {
            return Some(val.clone());
        } else {
            return None;
        }
    }

    fn contains(&self, id: IDType) -> bool {
        self.mask.contains(id as u32)
    }
}
