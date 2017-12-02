template<typename T>
struct Storage{
    virtual std::optional<T> add(id_type id, T to_add) = 0;
    virtual std::optional<T> remove(id_type id) = 0;
    virtual std::optional<T>& get(id_type id) = 0;
    virtual void clear() = 0;

    std::vector<bool> mask;

    virtual bool contains(id_type id) const {
        if ( (size_t)id < mask.size() ){
            return mask[id];
        } else {
            return false;
        }
    }

    virtual size_t size() = 0;
};

template<typename T>
struct NullStorage: public Storage<T>{
    std::optional<T> data = std::nullopt;

    virtual std::optional<T> add(id_type id, T ) override {
        if ((size_t)id >= this->mask.size() ){
            this->mask.resize((size_t)id+1);
        }

        this->mask[id] = true;

        return data;
    }

    virtual std::optional<T> remove(id_type id) override {
        if((size_t)id >= this->mask.size() ){
            return std::nullopt;
        }

        this->mask[id] = false;
        return data;
    }


    virtual std::optional<T>& get(id_type ) override {
        throw std::runtime_error("Can not retrieve data from NullStorage");
    }

    virtual void clear() override {
        this->mask.clear();
    }

    virtual size_t size() override {
        return this->mask.size();
    }
};

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




template<typename T>
struct VectorStorage: public Storage<T>{
    vector<std::optional<T>> data;

    virtual std::optional<T> add(id_type id, T to_add) override {
        if ((size_t)id >= data.size() ){
            data.resize((size_t)id+1);
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
};

template<typename T>
auto begin(VectorStorage<T> &storage){
    return storage.data.begin();
}

template<typename T>
auto end(VectorStorage<T> &storage){
    return storage.data.end();
}


