#[allow(unused)]
use *;

use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Spawner {
    prefabs: Vec<Prefab>,
}

#[derive(Clone)]
pub struct TextureHandle{
    pub val: Arc<Texture2D>,
}

impl TextureHandle {
    pub fn from_file_str(fname: &str) -> Self {
        Self {
            val: load_texture(fname.to_string()).unwrap(),
        }
    }
}

impl rlua::UserData for TextureHandle {
    fn add_methods(_methods: &mut rlua::UserDataMethods<Self>) {
    }
}



impl Spawner {
    pub fn new() -> Self {
        Self {prefabs: vec![]}
    }

    pub fn push(&mut self, val: Prefab) {
        self.prefabs.push(val);
    }

    pub fn spawn(&self , mut gd: &mut EcsWorld){
        for fab in self.prefabs.clone() {
            fab.spawn(&mut gd);
        }
    }

    pub fn spawn_at_pos(&self , mut gd: &mut EcsWorld, phy: &Physical){
        for fab in self.prefabs.clone() {
            let mut ffab = fab.clone();
            if let Some(ref mut pos) = ffab.physical {
                pos.x += phy.x;
                pos.y += phy.y;
            }
            ffab.spawn(&mut gd);
        }
    }
}

#[derive(Clone)]
pub struct SpawnPlan {
    plan: HashMap<u64, Vec<Spawner>>,
}

impl SpawnPlan {
    pub fn new() -> Self {
        Self {
            plan: HashMap::<u64,Vec<Spawner>>::new(),
        }
    }

    pub fn insert(&mut self, key: u64, val: Vec<Spawner>) -> Option<Vec<Spawner>>{
        self.plan.insert(key, val)
    }

    pub fn contains_key(&mut self, key: &u64) -> bool {
        self.plan.contains_key(&key)
    }
    pub fn is_empty(&mut self) -> bool {
        self.plan.is_empty()
    }

    pub fn remove(&mut self, key: &u64) -> Option<Vec<Spawner>>{
        self.plan.remove(key)
    }

    pub fn clear(&mut self) {
        self.plan.clear()
    }

    pub fn add(&mut self, key: u64, val: Spawner) {
        if self.contains_key(&key) {
            self.plan.get_mut(&key).unwrap().push(val);
        } else {
            self.insert( key, vec![val]);
        }
    }

    pub fn execute(&mut self, frame: u64, mut world: &mut EcsWorld) -> bool {
        let did_spawn;
        if let Some(lst) = self.remove(&frame) {
            for spawner in lst {
                spawner.spawn(&mut world);
            }
            did_spawn = true;
        } else {
            did_spawn = false;
        }

        return did_spawn;
    }
}

impl rlua::UserData for Spawner {
    fn add_methods(methods: &mut rlua::UserDataMethods<Self>) {
        methods.add_method_mut("push", 
           |_, this, args: Prefab| -> _ {
               this.push(args);
               Ok(())
           });

    }
}

pub fn register_level_gen(lua: &rlua::Lua) -> Result<(), rlua::Error >{
    {
        let fun = lua.create_function::<_, SpawnPlan, _>(|_,_:()| -> std::result::Result<_,_> { 
            Ok(SpawnPlan::new())
        } )?;
        lua.globals().set("SpawnPlan", fun).unwrap();
    }

    {
        let fun = lua.create_function::<_, Spawner, _>(|_,_:()| -> std::result::Result<_,_> { 
            Ok(Spawner::new())
        } )?;
        lua.globals().set("Spawner", fun).unwrap();
    }

    {
        let fun = lua.create_function::<_, Color, _>(
            |_,table: rlua::Table| -> _  { 
                let rr: u8 = table.get::<&str, f64>("r")? as u8;
                let gg: u8 = table.get::<&str, f64>("g")? as u8;
                let bb: u8 = table.get::<&str, f64>("b")? as u8;
                let aa: u8 = table.get::<&str, f64>("a")? as u8;
                let color =  Color::new(rr,gg,bb,aa);
                Ok(color)
            })?;
        lua.globals().set("Color", fun).unwrap();
    }

    {
        let fun = lua.create_function::<_, TextureHandle, _>(
            |_, table: rlua::Table| -> _ {
                let name: String = table.get("file")?;

                match load_texture(name) {
                    Some(val) => return Ok(TextureHandle{val:val}),
                    None => return Err(rlua::Error::RuntimeError("could not load texture".to_string())),
                }
            })?;
        lua.globals().set("Texture", fun).unwrap();
    }

    Ok(())
}

impl rlua::UserData for SpawnPlan {
    fn add_methods(methods: &mut rlua::UserDataMethods<Self>) {
        methods.add_method_mut("add", |_, this, (key, spawner): (i64, Spawner) | -> std::result::Result<(), rlua::Error> {
            this.add(key as u64 ,spawner);
            Ok(())
        });

        methods.add_method_mut("add_prefab", |_, this, (key, prefab): (i64, Prefab) | -> std::result::Result<(), rlua::Error> {
            let mut spawner = Spawner::new();
            spawner.push(prefab);
            this.add(key as u64 ,spawner);
            Ok(())
        });
    }
}

pub fn gen_level_spawner_from_lua(start_frame: u64,
                                  difficulty: f32,
                                  length: f32,
                                  fun_name: &str ,
                                  lua: &rlua::Lua) -> SpawnPlan {
    let fun: rlua::Function = lua.globals().get(fun_name).unwrap();
    match fun.call::<_,SpawnPlan>( (start_frame as i64,
                                    difficulty,
                                    length)) {
        Ok( ref val) => {
            return val.clone();
        },
        Err(e) => {
            print!("error {}\n", e);
            return SpawnPlan::new();
        }
    }

}

pub fn gen_star_spawner2(start_frame: u64, lua: &rlua::Lua) -> SpawnPlan {
    let fun : rlua::Function= lua.globals().get("gen_star_spawner").unwrap();

    let ret: SpawnPlan;
    match fun.call::<(i64), (SpawnPlan)>(start_frame as i64) {
        Ok(ref val) => {
            let sp = val;
            ret = sp.clone();
            return ret;
        }
        Err(e) => {
            print!("error {}\n",e);
        }
    }
    return SpawnPlan::new();
}
