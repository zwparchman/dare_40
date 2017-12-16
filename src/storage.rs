#[allow(unused)]
use std::collections::HashMap;

use hibitset::BitSet;

use id_type;

pub trait Storage<T>  where T: Clone {
    fn new() -> Self;

    fn add(&mut self, id: id_type, to_add: T) -> Option<T>;
    fn remove(&mut self, id: id_type) -> Option<T>;
    fn get(&self, id: id_type) -> Option<T>;

    fn contains(&self, id: id_type) -> bool;
}

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

    fn add(&mut self, id: id_type, to_add: T) -> Option<T> {
        if id as usize >= self.data.len() {
            self.data.resize( id as usize + 1, None );
        }

        let ret = self.data[id as usize].take();
        self.data[id as usize] = Some(to_add);

        self.mask.add(id as u32);

        return ret;
    }

    fn remove(&mut self, id: id_type) -> Option<T> {
        self.mask.remove(id as u32);
        if id as usize >= self.data.len() {
            return None;
        }

        let ret = self.data[id as usize].take();
        return ret;
    }

    fn get(&self, id: id_type) -> Option<T> {
        if id as usize >= self.data.len() {
            return None;
        }

        match self.data[id as usize] {
            Some(ref val) => return Some(val.clone()),
            None => return None,
        }
    }

    fn contains(&self, id: id_type) -> bool {
        self.mask.contains(id as u32)
    }
}

    /*
    virtual std::optional<T> remove(id_type id) override {
        if((size_t)id >= data.size() ){
            return std::nullopt;
        }

        std::optional<T> ret = data[(size_t)id];
        data[(size_t)id] = std::nullopt;
        this->mask[id] = false;
        return ret;
    }


    virtual std::optional<T>& get(id_type id) override {
        if ((size_t)id >= data.size() ){
            data.resize((size_t)id+1, std::nullopt);
            this->mask.resize((size_t)id+1);
        }

        std::optional<T>& ret = data[(size_t)id];
        return ret;
    }

    virtual void clear() override {
        data.clear();
        this->mask.clear();
    }

    virtual size_t size() override {
        return data.size();
    }
}
*/

#[allow(unused)]
pub struct NullStorage {
    mask: Vec<bool>,
}

/*
impl<T> Storage<T> for NullStorage {
    fn new() -> Self {
        Self{
            mask: Vec::<bool>::new(),
        }
    }

    fn add(&mut self, id: id_type, to_add: T) -> Option<T> {
        self.mask[id] = true;
        return None;
    }

    fn remove(&mut self, id: id_type) -> Option<T> {
        if id >= self.data.len() {
            return None;
        }

        let ret = self.data[id];

        self.mask[id] = false;

        return None;
    }

    fn get(&self, id: id_type) -> Option<&mut T> {
        if id >= self.data.len() {
            self.data.resize( id + 1 );
            self.mask.resize( id + 1 );
        }

        let ret = self.data[id];
        self.mask[id] = true;

        return None;
    }
}



pub struct HashStorage<T> {
    data: HashMap<id_type, T>,
    mask: Vec<bool>,
}

impl<T> HashStorage<T> {
    pub fn new() -> Self {
        Self{
            data: HashMap::<id_type, T>::new(),
            mask: Vec::<bool>::new(),
        }
    }
}

*/

/*
impl<T> Storage<T> for HashStorage<T> {
    fn new() -> Self {
        Self{
            data: HashStorage::<T>::new(),
            mask: Vec::<bool>::new(),
        }
    }

    fn add(&mut self, id: id_type, to_add: T) -> Option<T> {
        if id >= self.data.len() {
            self.data.resize( id + 1 );
            self.mask.resize( id + 1 );
        }

        let ret = self.data[id];
        self.data[id] = Some(to_add);

        self.mask[id] = true;

        return ret;
    }

    fn remove(&mut self, id: id_type) -> Option<T> {
        if id >= self.data.len() {
            return None;
        }

        let ret = self.data[id];

        self.mask[id] = false;

        return ret;
    }

    fn get(&mut self, id: id_type) -> &Option<T> {
        if id >= self.data.len() {
            self.data.resize( id + 1 );
            self.mask.resize( id + 1 );
        }

        let ret = self.data[id];
        self.mask[id] = true;

        return ret;
    }
}

/*
template<typename T>
struct HashStorage: public Storage<T>{
    unordered_map<id_type,std::optional<T>> data;

    virtual std::optional<T> add(id_type id, T to_add) override {
        if ((size_t)id >= data.size() ){
            this->mask.resize((size_t)id+1);
        }

        std::optional<T> ret = data[(size_t)id];
        data[(size_t)id] = std::optional<T>{to_add};

        this->mask[id] = true;
        return ret;
    }

    virtual std::optional<T> remove(id_type id) override {
        if((size_t)id >= data.size() ){
            return std::nullopt;
        }

        std::optional<T> ret = data[(size_t)id];
        data[(size_t)id] = std::nullopt;
        this->mask[id] = false;
        return ret;
    }


    virtual std::optional<T>& get(id_type id) override {
        if ((size_t)id >= data.size() ){
            this->mask.resize((size_t)id+1);
        }

        std::optional<T>& ret = data[(size_t)id];
        return ret;
    }

    virtual void clear() override {
        data.clear();
        this->mask.clear();
    }

    virtual size_t size() override {
        return data.size();
    }
};
*/
*/
