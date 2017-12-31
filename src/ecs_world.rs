#[allow(unused)]
use *;

macro_rules! declare_ecs_world_full {
    ($(($name: ident, $list: ident, $type: ty, $storage: ty),)+) => {

pub struct EcsWorld {
    $(
        pub $list: $storage,
    )+

    pub unused_ids: Vec<IDType>,
    pub max_id: IDType,

    pub to_destroy: Vec<IDType>,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            $(
                $list: <$storage>::new(),
            )+


            unused_ids: Vec::<IDType>::new(),
            max_id: 0,
            to_destroy: Vec::<IDType>::new(),
        }
    }

    #[allow(unused)]
    pub fn get_max_id(&self) -> IDType {
        self.max_id
    }

    pub fn destroy(&mut self, id: IDType){
        $(
            self.$list.remove(id);
        )+

        self.free_id(id);
    }

    pub fn maintain(&mut self){
        self.to_destroy.sort();
        self.to_destroy.dedup_by(|a, b| { a == b } );

        for id in self.to_destroy.clone() {
            self.destroy(id);
        }
        self.to_destroy.clear();
    }

    pub fn alloc_id(&mut self) -> IDType {
        let out: IDType;
        if 0 != self.unused_ids.len() {
            let val = self.unused_ids.pop().unwrap();
            out = val;
        } else {
            out = self.max_id;
            self.max_id += 1;
        }

        //print!("alocated id {} on frame {}\n", out, self.frame_count);
        return out;
    }

    pub fn free_id(&mut self, id: IDType) {
        self.unused_ids.push(id);
    }

    pub fn destroy_later(&mut self, id: IDType) {
        self.to_destroy.push(id);
    }

    pub fn id_to_prefab(&self, id: IDType) -> Prefab {
        let mut ret = PrefabBuilder::new();
        $(
            if let Some(val) = self.$list.get(id) {
                ret = ret.$name(val);
            }
        )+
        return ret.build();
    }
}

#[derive(Clone)]
pub struct Prefab {
    $(
        pub $name: Option<$type>,
    )+
}

impl Prefab {
    pub fn spawn(&self, gd: &mut EcsWorld) -> IDType{
        let id = gd.alloc_id();

        $(
            if let Some(val) = self.$name.clone() {
                gd.$list.add(id, val);
            }
        )+

        return id;
    }
}


#[derive(Clone)]
/// Helper for building a prefab
pub struct PrefabBuilder {
    thing: Prefab,
}

impl PrefabBuilder{
    pub fn new() -> Self {
        Self{ thing: Prefab{
            $(
                $name: None,
            )+
        }}
    }

    $(
        pub fn $name(mut self, val: $type) -> Self {
            self.thing.$name = Some(val);
            self
        }
    )+

    pub fn build(self) -> Prefab {
        return self.thing
    }
}


impl rlua::UserData for Prefab{
    fn add_methods(_methods: &mut rlua::UserDataMethods<Self>){}
}

pub fn register_ecs(lua: &rlua::Lua) -> std::result::Result<(), rlua::Error>{
    let fun = lua.create_function::<_, Prefab, _>(
        |_, table: rlua::Table| -> _ {
            let mut builder = PrefabBuilder::new();
            $(
            match table.get::<&str, rlua::Value>(stringify!($name)) {
                Ok(val) => {
                    match val {
                        rlua::Value::Table(ref dat) => {
                            builder = builder.$name(<$type>::from_table(dat));
                        },
                        rlua::Value::Nil => {},
                        _ => print!("bad value when loading {}: {:?}\n", stringify!($name), val),
                    }
                }
                Err(e) => {
                    print!("error {:?}\n", e);
                }
            }
            )+
            Ok(builder.build())
        })?;
    lua.globals().set("Prefab", fun).unwrap();

    return Ok(());
}

}} //end of macro definition

declare_ecs_world_full!(
    (drawable, drawable_list, Drawable, VectorStorage<Drawable>),
    (physical, physical_list, Physical, VectorStorage<Physical>),
    (collidable, collidable_list, Collidable,  VectorStorage<Collidable>),
    (player_control, controllable_list, PlayerControl,  HashStorage<PlayerControl>),
    (bullet, bullet_list, Bullet, HashStorage<Bullet>),
    (shield, shield_list, Shield, HashStorage<Shield>),
    (despawn_far_left, despawn_far_left, DespawnFarLeft, HashStorage<DespawnFarLeft>),
    (despawn_far_right, despawn_far_right, DespawnFarRight, HashStorage<DespawnFarRight>),
    (powerup, powerup_list, Powerup, HashStorage<Powerup>),
    (player_stats, player_stats_list, PlayerStats, HashStorage<PlayerStats>),
    (weapon, weapon_list, Weapon, HashStorage<Weapon>),
    (auto_fire, auto_fire_list, AutoFire, HashStorage<AutoFire>),
    (sine_movement, sine_movement_list, SineMovement, HashStorage<SineMovement>),
    (sine_movement_x, sine_movement_x_list, SineMovementX, HashStorage<SineMovementX>),
    (team, team_list, Team, VectorStorage<Team>),
    (install, install_list, Install, HashStorage<Install>),
    (death_event, death_event_list, DeathEvent, HashStorage<DeathEvent>),
    (stop_at, stop_at_list, StopAt, HashStorage<StopAt>),
    (timeout_death, timeout_death_list, TimeoutDeath, HashStorage<TimeoutDeath>),
    (boss_health_draw, boss_health_draw_list, BossHealthDraw, HashStorage<BossHealthDraw>),
    (despawn_y, despawn_y_list, DespawnY, HashStorage<DespawnY>),
    (follow_player_y, follow_player_y_list, FollowPlayerY, HashStorage<FollowPlayerY>),
    (drag, drag_list, Drag, HashStorage<Drag>),
    (avoid_player_y, avoid_player_y_list, AvoidPlayerY, HashStorage<AvoidPlayerY>),
    (clamp_y, clamp_y_list, ClampY, HashStorage<ClampY>),
    (point_along_movement_vector, point_along_movement_vector_list, PointAlongMovementVector, HashStorage<PointAlongMovementVector>),
);
