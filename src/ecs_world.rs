#[allow(unused)]
use *;


pub struct EcsWorld {
    pub drawable_list: VectorStorage<Drawable>,
    pub physical_list: VectorStorage<Physical>,
    pub collidable_list: VectorStorage<Collidable>,
    pub controllable_list: HashStorage<PlayerControl>,
    pub bullet_list: HashStorage<Bullet>,
    pub shield_list: HashStorage<Shield>,
    pub despawn_left: HashStorage<DespawnFarLeft>,
    pub despawn_right: HashStorage<DespawnFarRight>,
    pub powerup_list: HashStorage<Powerup>,
    pub player_stats_list: HashStorage<PlayerStats>,
    pub weapon_list: HashStorage<Weapon>,
    pub auto_fire_list: HashStorage<AutoFire>,
    pub sine_movement_list: HashStorage<SineMovement>,
    pub sine_movement_x_list: HashStorage<SineMovementX>,
    pub team_list: VectorStorage<Team>,
    pub install_list: HashStorage<Install>,
    pub death_event_list: HashStorage<DeathEvent>,
    pub stop_at_list: HashStorage<StopAt>,
    pub timeout_death_list: HashStorage<TimeoutDeath>,
    pub boss_health_draw_list: HashStorage<BossHealthDraw>,

    pub unused_ids: Vec<IDType>,
    pub max_id: IDType,

    pub to_destroy: Vec<IDType>,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            drawable_list: VectorStorage::<Drawable>::new(),
            physical_list: VectorStorage::<Physical>::new(),
            collidable_list: VectorStorage::<Collidable>::new(),
            controllable_list: HashStorage::<PlayerControl>::new(),
            bullet_list: HashStorage::<Bullet>::new(),
            shield_list: HashStorage::<Shield>::new(),
            despawn_left: HashStorage::<DespawnFarLeft>::new(),
            despawn_right: HashStorage::<DespawnFarRight>::new(),
            powerup_list: HashStorage::<Powerup>::new(),
            player_stats_list: HashStorage::<PlayerStats>::new(),
            weapon_list: HashStorage::<Weapon>::new(),
            auto_fire_list: HashStorage::<AutoFire>::new(),
            sine_movement_list: HashStorage::<SineMovement>::new(),
            sine_movement_x_list: HashStorage::<SineMovementX>::new(),
            team_list: VectorStorage::<Team>::new(),
            install_list: HashStorage::<Install>::new(),
            death_event_list: HashStorage::<DeathEvent>::new(),
            stop_at_list: HashStorage::<StopAt>::new(),
            timeout_death_list: HashStorage::<TimeoutDeath>::new(),
            boss_health_draw_list: HashStorage::<BossHealthDraw>::new(),

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
        self.drawable_list.remove(id);
        self.physical_list.remove(id);
        self.collidable_list.remove(id);
        self.controllable_list.remove(id);
        self.bullet_list.remove(id);
        self.shield_list.remove(id);
        self.despawn_left.remove(id);
        self.despawn_right.remove(id);
        self.powerup_list.remove(id);
        self.player_stats_list.remove(id);
        self.weapon_list.remove(id);
        self.auto_fire_list.remove(id);
        self.sine_movement_list.remove(id);
        self.sine_movement_x_list.remove(id);
        self.team_list.remove(id);
        self.install_list.remove(id);
        self.death_event_list.remove(id);
        self.stop_at_list.remove(id);
        self.timeout_death_list.remove(id);
        self.boss_health_draw_list.remove(id);

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
}

#[derive(Clone)]
pub struct Prefab {
    pub drawable: Option<Drawable>,
    pub physical: Option<Physical>,
    pub collidable: Option<Collidable>,
    pub controllable: Option<PlayerControl>,
    pub bullet: Option<Bullet>,
    pub shield: Option<Shield>,
    pub despawn_left: Option<DespawnFarLeft>,
    pub despawn_right: Option<DespawnFarRight>,
    pub powerup: Option<Powerup>,
    pub player: Option<PlayerStats>,
    pub weapon: Option<Weapon>,
    pub auto_fire: Option<AutoFire>,
    pub sine: Option<SineMovement>,
    pub sine_x: Option<SineMovementX>,
    pub team: Option<Team>,
    pub install: Option<Install>,
    pub death_event: Option<DeathEvent>,
    pub stop_at: Option<StopAt>,
    pub timeout_death: Option<TimeoutDeath>,
    pub boss_health_draw: Option<BossHealthDraw>,
}

impl Prefab {
    pub fn spawn(&self, gd: &mut EcsWorld) -> IDType{
        let id = gd.alloc_id();

        if let Some(val) = self.drawable.clone() {
            gd.drawable_list.add(id, val);
        }
        if let Some(val) = self.physical.clone() {
            gd.physical_list.add(id,val);
        }
        if let Some(val) = self.collidable.clone() {
            gd.collidable_list.add(id,val);
        }
        if let Some(val) = self.controllable.clone() {
            gd.controllable_list.add(id,val);
        }
        if let Some(val) = self.bullet.clone() {
            gd.bullet_list.add(id,val);
        }
        if let Some(val) = self.shield.clone() {
            gd.shield_list.add(id,val);
        }
        if let Some(val) = self.despawn_left.clone() {
            gd.despawn_left.add(id,val);
        }
        if let Some(val) = self.despawn_right.clone() {
            gd.despawn_right.add(id,val);
        }
        if let Some(val) = self.powerup.clone() {
            gd.powerup_list.add(id,val);
        }
        if let Some(val) = self.player.clone() {
            gd.player_stats_list.add(id,val);
        }
        if let Some(val) = self.weapon.clone() {
            gd.weapon_list.add(id,val);
        }
        if let Some(val) = self.auto_fire.clone() {
            gd.auto_fire_list.add(id,val);
        }
        if let Some(val) = self.sine.clone() {
            gd.sine_movement_list.add(id,val);
        }
        if let Some(val) = self.sine_x.clone() {
            gd.sine_movement_x_list.add(id,val);
        }
        if let Some(val) = self.team.clone() {
            gd.team_list.add(id,val);
        }
        if let Some(val) = self.install.clone() {
            gd.install_list.add(id,val);
        }
        if let Some(val) = self.death_event.clone() {
            gd.death_event_list.add(id,val);
        }
        if let Some(val) = self.stop_at.clone() {
            gd.stop_at_list.add(id,val);
        }
        if let Some(val) = self.timeout_death.clone() {
            gd.timeout_death_list.add(id,val);
        }
        if let Some(val) = self.boss_health_draw.clone() {
            gd.boss_health_draw_list.add(id,val);
        }

        return id;
    }
}

#[derive(Clone)]
pub struct PrefabBuilder {
    thing: Prefab,
}

impl PrefabBuilder{
    pub fn new() -> Self {
        Self{ thing: Prefab{
            drawable: None,
            physical: None,
            collidable: None,
            controllable: None,
            bullet: None,
            shield: None,
            despawn_left: None,
            despawn_right: None,
            powerup: None,
            player: None,
            weapon: None,
            auto_fire: None,
            sine: None,
            sine_x: None,
            team: None,
            install: None,
            death_event: None,
            stop_at: None,
            timeout_death: None,
            boss_health_draw: None, 
        }}
    }

    pub fn drawable(mut self, val:Drawable) -> Self {
        self.thing.drawable = Some(val);
        self
    }
    pub fn physical(mut self, val:Physical) -> Self {
        self.thing.physical = Some(val);
        self
    }
    pub fn collidable(mut self, val:Collidable) -> Self {
        self.thing.collidable = Some(val);
        self
    }
    pub fn controllable(mut self, val:PlayerControl) -> Self {
        self.thing.controllable = Some(val);
        self
    }
    pub fn bullet(mut self, val:Bullet) -> Self {
        self.thing.bullet = Some(val);
        self
    }
    pub fn shield(mut self, val:Shield) -> Self {
        self.thing.shield = Some(val);
        self
    }
    pub fn despawn_left(mut self, val:DespawnFarLeft) -> Self {
        self.thing.despawn_left = Some(val);
        self
    }
    pub fn despawn_right(mut self, val:DespawnFarRight) -> Self {
        self.thing.despawn_right = Some(val);
        self
    }
    pub fn powerup(mut self, val:Powerup) -> Self {
        self.thing.powerup = Some(val);
        self
    }
    pub fn player_stats(mut self, val:PlayerStats) -> Self {
        self.thing.player = Some(val);
        self
    }
    pub fn weapon(mut self, val:Weapon) -> Self {
        self.thing.weapon = Some(val);
        self
    }
    pub fn auto_fire(mut self, val:AutoFire) -> Self {
        self.thing.auto_fire = Some(val);
        self
    }
    pub fn sine_movement(mut self, val:SineMovement) -> Self {
        self.thing.sine = Some(val);
        self
    }
    pub fn sine_movement_x(mut self, val:SineMovementX) -> Self {
        self.thing.sine_x = Some(val);
        self
    }
    pub fn team(mut self, val:Team) -> Self {
        self.thing.team = Some(val);
        self
    }
    pub fn install(mut self, val:Install) -> Self {
        self.thing.install = Some(val);
        self
    }
    pub fn death_event(mut self, val:DeathEvent) -> Self {
        self.thing.death_event = Some(val);
        self
    }
    pub fn stop_at(mut self, val:StopAt) -> Self {
        self.thing.stop_at = Some(val);
        self
    }
    pub fn timeout_death(mut self, val:TimeoutDeath) -> Self {
        self.thing.timeout_death = Some(val);
        self
    }

    pub fn boss_health_draw(mut self, val:BossHealthDraw) -> Self {
        self.thing.boss_health_draw = Some(val);
        self
    }

    pub fn build(self) -> Prefab {
        return self.thing
    }
}
