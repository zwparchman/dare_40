#[allow(unused)]
use *;

use rand;
use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[derive(Clone, Default)]
pub struct Spawner {
    prefabs: Vec<Prefab>,
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

fn gen_player() -> Prefab{
    PrefabBuilder::new()
        .install(Install{})
        .drawable(DrawableBuilder::default()
                  .texture_by_name("player.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                           .x(50.0)
                           .y(200.0)
                           .build().unwrap())
        .player_control(PlayerControl{})
        .collidable(Collidable{ radius: 20.0})
        .player_stats( PlayerStats{
            movement_speed: 350.0,
            base_speed: 350.0,
            owned: vec![],
            install_progress: 0,
            install_finish_sound: load_sound("upgrade-finished.wav".to_string()).unwrap()
        })
        .shield( ShieldBuilder::default()
                 .regen(1.00)
                 .ammount(30.0)
                 .build().unwrap())
        .weapon( WeaponBuilder::default()
                 .fire_angle(60.0)
                 .fire_rate(0.3)
                 .fire_velocity(300.0)
                 .fire_sound("laser001.wav".to_string())
                 .prefab(PrefabBuilder::new()
                           .physical(PhysicalBuilder::default().build().unwrap())
                           .team(Team{team:0})
                           .bullet(Bullet{damage: 10.0})
                           .despawn_far_right(DespawnFarRight{})
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::default()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build().unwrap())
                           .build())
                 .offset(40.0)
                 .build().unwrap())
        .clamp_y(ClampYBuilder::default()
                 .low(0.0)
                 .high(768.0)
                 .build().unwrap())
        .team(Team{team:0})
        .build()
}

#[allow(unused)]
fn gen_shot_increase(x: f32, y: f32, mut rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    let mut base = gen_fire_damage_increase(x,y,&mut rng);

    base.powerup = Some(PowerupBuilder::default()
        .sound_by_name("item-pickup.wav".to_string())
        .shot_increase(1)
        .build().unwrap());
    base.drawable = Some(DrawableBuilder::default()
                         .texture_by_name("shot-number-increase.png".to_string())
                         .layer(1.0)
                         .build().unwrap());
    return base;
}

fn gen_fire_damage_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("damage-up.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build().unwrap())
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("item-pickup.wav".to_string())
                 .fire_damage_increase(1.05)
                 .build().unwrap())
        .build()
}

fn gen_regen_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("shield-regen.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build().unwrap())
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("item-pickup.wav".to_string())
                 .regen_increase(1.05)
                 .build().unwrap())
        .build()
}

fn gen_shield_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("shield-up.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build().unwrap())
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("item-pickup.wav".to_string())
                 .shield_increase(1.05)
                 .build().unwrap())
        .build()
}

fn gen_fire_rate_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("fire-rate-up.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build().unwrap())
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("item-pickup.wav".to_string())
                 .fire_rate_increase(0.95)
                 .build().unwrap())
        .build()
}

fn gen_enemy_2(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("enemy2.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                           .x(x)
                           .y(y)
                           .xvel(rng.gen_range(-200.0, -150.0))
                           .build().unwrap())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::default()
                     .score_add(20)
                     .sound_by_name("explosion001.wav".to_string())
                     .build().unwrap())
        .shield( ShieldBuilder::default()
                 .ammount(11.0)
                 .build().unwrap())
        .sine_movement( SineMovementBuilder::default()
                        .amplitude(rng.gen_range(20.0, 40.0))
                        .frequency(rng.gen_range(0.5, 2.0))
                        .build().unwrap())
        .weapon( WeaponBuilder::default()
                 .fire_rate(rng.gen_range(1.7, 2.0))
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_far_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 10.0})
                           .physical(PhysicalBuilder::default().build().unwrap())
                           .collidable(Collidable{radius: 4.0})
                           .sine_movement(SineMovementBuilder::default()
                                          .amplitude(30.0)
                                          .frequency(2.0)
                                          .build().unwrap())
                           .sine_movement_x(SineMovementXBuilder::default()
                                            .amplitude(30.0)
                                            .frequency(2.5)
                                            .build().unwrap())
                           .drawable(DrawableBuilder::default()
                                     .texture_by_name("green-ball.png".to_string())
                                     .layer(1.0)
                                     .build().unwrap())
                           .build())
                 .fire_velocity(rng.gen_range(-300.0, -280.0))
                 .offset(-10.0)
                 .gun_cooldown_frames(rng.gen_range(2.0,4.0))
                 .build().unwrap())
        .team(Team{team:1})
        .build()
}

fn gen_enemy_3(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("enemy3.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                           .x(x)
                           .y(y)
                           .xvel(-100.5+ rng.next_f32()*0.01)
                           .build().unwrap())
        .stop_at(StopAtBuilder::default()
                 .xloc(900.0+rng.next_f32()*100.0)
                 .build().unwrap())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::default()
                     .sound_by_name("explosion001.wav".to_string())
                     .score_add(50)
                     .build().unwrap())
        .shield( ShieldBuilder::default()
                 .ammount(30.0)
                 .build().unwrap())
        .sine_movement( SineMovementBuilder::default()
                        .amplitude(rng.gen_range(300.0,320.0))
                        .frequency(rng.gen_range(0.05,1.0))
                        .build().unwrap())
        .clamp_y(ClampYBuilder::default()
                 .low(0.0)
                 .high(768.0)
                 .build().unwrap())
        .sine_movement_x( SineMovementXBuilder::default()
                          .amplitude(rng.gen_range(50.0, 150.0))
                          .frequency(rng.gen_range(0.05, 0.30))
                          .build().unwrap())
        .weapon( WeaponBuilder::default()
                 .pattern(2)
                 .fire_angle(rng.gen_range(90.0, 120.0))
                 .fire_rate(rng.gen_range(1.0, 1.5))
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_far_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 2.0})
                           .sine_movement(SineMovementBuilder::default()
                                          .amplitude(20.0)
                                          .frequency(3.0)
                                          .build().unwrap())
                           .physical(PhysicalBuilder::default().build().unwrap())
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::default()
                                     .texture_by_name("orange-ball.png".to_string())
                                     .layer(1.0)
                                     .build().unwrap())
                           .build())
                 .fire_velocity(rng.gen_range(-300.0, -200.0))
                 .offset(-10.0)
                 .gun_cooldown_frames(rng.gen_range(1.0, 3.0))
                 .build().unwrap())
        .team(Team{team:1})
        .build()
}

fn gen_enemy_1(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("enemy1.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                           .x(x)
                           .y(y)
                           .xvel(rng.gen_range(-200.0,-150.0))
                           .build().unwrap())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::default()
                     .sound_by_name("explosion001.wav".to_string())
                     .score_add(10)
                     .build().unwrap())
        .shield( ShieldBuilder::default()
                 .ammount(11.0)
                 .build().unwrap())
        .follow_player_y( FollowPlayerYBuilder::default()
                          .speed(100.0)
                          .build().unwrap())
        .weapon( WeaponBuilder::default()
                 .fire_rate(rng.gen_range(3.0, 4.0))
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_far_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 10.0})
                           .physical(PhysicalBuilder::default().build().unwrap())
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::default()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build().unwrap())
                           .build())
                 .fire_velocity(-300.0 - rng.next_f32() * 0.2)
                 .offset(-10.0)
                 .gun_cooldown_frames(rng.gen_range(1.0, 3.0))
                 .build().unwrap())
        .team(Team{team:1})
        .build()
}

fn gen_bomb(rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    let sub_munition_prefab = PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("yellow-ball.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .timeout_death(TimeoutDeathBuilder::default()
                       .ticks(10000)
                       .build().unwrap())
        .collidable(Collidable{radius: 4.0})
        .despawn_far_left(DespawnFarLeft{})
        .despawn_far_right(DespawnFarRight{})
        .team(Team{team:1})
        .bullet(Bullet{damage: 3.0});

    let get_sub = |prefab: &PrefabBuilder, mut angle: f32, vel: f32| {
        angle *= DEG2RAD as f32;
        prefab.clone()
            .physical(PhysicalBuilder::default()
                      .xvel(angle.cos() * -1.0 * vel)
                      .yvel(angle.sin() * vel)
                      .build().unwrap())
            .build()
    };

    let mut spawner= Spawner::new();
    for angle in get_shot_angles( rng.gen_range(200.0, 360.0), rng.gen_range(5, 10)) {
        spawner.push( get_sub(&sub_munition_prefab, angle, rng.gen_range(100.0, 150.0)));
    }

    //the actuall bomb
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("bomb.png".to_string())
                  .layer(2.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default().build().unwrap())
        .drag(DragBuilder::default()
              .x(0.8)
              .build().unwrap())
        .timeout_death(TimeoutDeathBuilder::default()
                       .ticks( (4.0*FRAME_RATE) as i32)
                       .build().unwrap())
        .death_event(DeathEventBuilder::default()
                     .spawner(Arc::new(spawner))
                     .sound_by_name("bomb-explode.wav".to_string())
                     .build().unwrap())
        .build()
}

fn gen_enemy_5(x: f32, y: f32, mut rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    let bomb = gen_bomb(&mut rng);

    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("enemy5.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(x)
                  .y(y)
                  .xvel(-70.0)
                  .build().unwrap())
        .auto_fire(AutoFire{})
        .shield(ShieldBuilder::default()
                .ammount(50.0)
                .build().unwrap())
        .sine_movement(SineMovementBuilder::default()
                       .amplitude(5.0)
                       .frequency(1.5)
                       .build().unwrap())
        .weapon(WeaponBuilder::default()
                .prefab(bomb)
                .fire_rate(rng.gen_range(4.0, 5.0))
                .fire_sound("bomb-launch.wav".to_string())
                .fire_velocity(rng.gen_range(-300.0, -200.0))
                .gun_cooldown_frames( 5.0 )
                .offset(-40.0)
                .build().unwrap())
        .collidable(Collidable{radius: 30.0})
        .death_event(DeathEventBuilder::default()
                     .score_add(20)
                     .sound_by_name("explosion002.wav".to_string())
                     .build().unwrap())
        .despawn_far_left(DespawnFarLeft{})
        .team(Team{team:1})
        .build()
}

fn gen_enemy_4(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    let base_builder = PrefabBuilder::new()
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("bad-pickup.wav".to_string())
                 .build().unwrap())
        .drawable(DrawableBuilder::default()
                  .texture_by_name("null-powerup.png".to_string())
                  .build().unwrap());

    let phy_builder = PhysicalBuilder::default()
        .xvel(-100.0);

    let mut spawner = Spawner::new();
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(5.0)
                           .build().unwrap())
                 .build());
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(0.0)
                           .build().unwrap())
                 .build());
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(-5.0)
                           .build().unwrap())
                 .build());
    
    PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("enemy4.png".to_string())
                  .layer(1.0)
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                           .x(x)
                           .y(y)
                           .xvel(-150.5+ rng.next_f32()*15.0)
                           .build().unwrap())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 14.0})
        .despawn_far_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::default()
                     .sound_by_name("explosion-small.wav".to_string())
                     .spawner(Arc::new(spawner))
                     .score_add(5)
                     .build().unwrap())
        .shield( ShieldBuilder::default()
                 .ammount(1.0)
                 .build().unwrap())
        .sine_movement( SineMovementBuilder::default()
                        .amplitude(rng.gen_range(0.0,40.0))
                        .frequency(rng.gen_range(1.0, 3.0))
                        .build().unwrap())
        .team(Team{team:1})
        .build()
}

fn gen_random_upgrade(x: f32, y: f32, mut rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    let a = gen_fire_rate_increase(x,y,&mut rng).clone();
    let b = gen_fire_damage_increase(x,y,&mut rng).clone();
    let c = gen_shield_increase(x,y,&mut rng).clone();
    let d = gen_regen_increase(x,y,&mut rng).clone();
    return rng.choose(&[a,b,c,d]).unwrap().clone();
}

type GenThing = fn (x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab;
type GenLevel = fn (difficulty: f32, length: f32, start_frame: u64, rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan;


pub fn gen_level_from_weights(difficulty: f32,
                              length: f32,
                              start_frame: u64,
                              mut rng: &mut rand::isaac::Isaac64Rng,
                              mut weights: &mut Vec<Weighted<(GenThing,f32)>>) 
                -> SpawnPlan {

    let mut ret = SpawnPlan::new();
    let chooser = WeightedChoice::new(&mut weights);

    let mut len_left = length - rng.gen_range(0.0, 1.0 * FRAME_RATE);
    while len_left > 0.0 {
        let mut spawner = Spawner::new();
        let mut cur_diff = difficulty;
        while cur_diff > 0.0 {
            let (fun,dif) = chooser.ind_sample(&mut rng);
            cur_diff -= dif;
            spawner.push(fun(rng.gen_range(1400.0, 1500.0), rng.gen_range(0.0, 700.0), &mut rng));
        }

        let offset = FRAME_RATE * rng.gen_range(3.0, 5.0);
        let when: u64 = start_frame + len_left as u64;

        // print!("when {}\n", when);
        trace!("adding spawner for frame {}", when);
        ret.add( when, spawner );
        len_left -= offset;
    }
    return ret;
}


pub fn gen_level_bomber(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut weights = vec![ 
        Weighted{ weight: 5, item: (gen_enemy_4 as GenThing, 2.0) },
        Weighted{ weight: 1, item: (gen_enemy_1 as GenThing, 10.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 30.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 50.0) },
        Weighted{ weight: 100, item: (gen_enemy_5 as GenThing, 40.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    return gen_level_from_weights(difficulty, length, start_frame, &mut rng, &mut weights);
}

pub fn gen_level_bad_upgrade(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut weights = vec![ 
        Weighted{ weight: 100, item: (gen_enemy_4 as GenThing, 8.0) },
        Weighted{ weight: 1, item: (gen_enemy_1 as GenThing, 10.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 30.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 50.0) },
        Weighted{ weight: 1, item: (gen_enemy_5 as GenThing, 40.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    return gen_level_from_weights(difficulty, length, start_frame, &mut rng, &mut weights);
}

pub fn gen_level_simple(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut weights = vec![ 
        Weighted{ weight: 10, item: (gen_enemy_1 as GenThing, 10.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 30.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 50.0) },
        Weighted{ weight: 0, item: (gen_enemy_4 as GenThing, 2.0) },
        Weighted{ weight: 1, item: (gen_enemy_5 as GenThing, 40.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    return gen_level_from_weights(difficulty, length, start_frame, &mut rng, &mut weights);
}

pub fn gen_boss_2_level(_difficulty: f32, _length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut plan = SpawnPlan::new();
    let mut spawner = Spawner::new();

    let missile_builder = PrefabBuilder::new()
        .team(Team{team: 1})
        .follow_player_y(FollowPlayerYBuilder::default()
                         .speed(70.0)
                         .build().unwrap())
        .shield(ShieldBuilder::default()
                .ammount(30.0)
                .build().unwrap())
        .team(Team{team: 1})
        .collidable(Collidable{radius: 4.0})
        .drawable(DrawableBuilder::default()
                  .layer(1.0)
                  .texture_by_name("missile.png".to_string())
                  .build().unwrap())
        .despawn_far_left(DespawnFarLeft{})
        .bullet(Bullet{damage: 5.0})
        .drag(DragBuilder::default()
              .y(0.90)
              .build().unwrap());

    let mut missile_spawner = Spawner::new();
    for i in 0..4_i32 {
        let xoff = 10.0 * i as f32;
        let yoff = 10.0;

        let xvel = -10.0 * i as f32;
        let yvel = i as f32 *120.0;

        let xacc = -300.0;
        let p1 = PhysicalBuilder::default()
            .x(xoff)
            .y(yoff)
            .xvel(xvel)
            .yvel(yvel)
            .xacc(xacc)
            .build().unwrap();
        let p2 = PhysicalBuilder::default()
            .x(xoff)
            .y(-yoff)
            .xvel(xvel)
            .yvel(-yvel)
            .xacc(xacc)
            .build().unwrap();

        missile_spawner.push(missile_builder.clone()
                             .physical(p1)
                             .build());
        missile_spawner.push(missile_builder.clone()
                             .physical(p2)
                             .build());
    }

    let spawn_missile_prefab = PrefabBuilder::new()
        .physical(PhysicalBuilder::default().build().unwrap())
        .death_event(DeathEventBuilder::default()
                    .spawner(Arc::new(missile_spawner))
                    .build().unwrap())
        .timeout_death(TimeoutDeathBuilder::default()
                       .ticks(1)
                       .build().unwrap())
        .build();

    let shot_increase_powerup = gen_shot_increase(0.0, 0.0, &mut rng);
    let mut shot_increase_spawner = Spawner::new();
    shot_increase_spawner.push(shot_increase_powerup);

    let boss = PrefabBuilder::new()
        .drawable(DrawableBuilder::default()
                  .texture_by_name("boss002.png".to_string())
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(1400.0)
                  .y(400.0)
                  .xvel(-100.0)
                  .build().unwrap())
        .team(Team{team:1})
        .sine_movement(SineMovementBuilder::default()
                       .amplitude(50.0)
                       .frequency(1.0)
                       .build().unwrap())
        .follow_player_y(FollowPlayerYBuilder::default()
                         .speed(5.0)
                         .build().unwrap())
        .weapon(WeaponBuilder::default()
                .fire_rate(5.0)
                .fire_velocity(0.0)
                .fire_sound("missile-launch.wav".to_string())
                .prefab(spawn_missile_prefab)
                .build().unwrap())
        .auto_fire(AutoFire{})
        .stop_at(StopAtBuilder::default()
                .xloc(1100.0)
                .build().unwrap())
        .shield(ShieldBuilder::default()
                .ammount(800.0)
                .build().unwrap())
        .boss_health_draw(BossHealthDrawBuilder::default().build().unwrap())
        .collidable(Collidable{radius: 50.0})
        .avoid_player_y(AvoidPlayerYBuilder::default()
                        .speed(25.0)
                        .build().unwrap())
        .clamp_y(ClampYBuilder::default()
                 .low(50.0)
                 .high(700.0)
                 .build().unwrap())
        .death_event(DeathEventBuilder::default()
                     .sound_by_name("explosion001.wav".to_string())
                     .spawner(Arc::new(shot_increase_spawner))
                     .clear_spawn_plan(true)
                     .build().unwrap())
        .build();

    spawner.push( boss );
    plan.add( (start_frame as f32 + 1.0 * FRAME_RATE) as u64, spawner);

    spawner = Spawner::new();
    spawner.push( PrefabBuilder::new().build());
    plan.add((start_frame as f32 + 1000.0 * FRAME_RATE) as u64, spawner);
    return plan;
}

pub fn gen_boss_1_level(difficulty: f32, _length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    print!("boss level!\n");
    let base_builder = PrefabBuilder::new()
        .collidable(Collidable{radius: 20.0})
        .despawn_far_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::default()
                 .sound_by_name("bad-pickup.wav".to_string())
                 .build().unwrap())
        .drawable(DrawableBuilder::default()
                  .texture_by_name("null-powerup.png".to_string())
                  .build().unwrap());

    let phy_builder = PhysicalBuilder::default()
        .xvel(-100.0);

    let mut null_powerup_spawner = Spawner::new();
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(5.0)
                           .build().unwrap())
                 .build());
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(0.0)
                           .build().unwrap())
                 .build());
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(-5.0)
                           .build().unwrap())
                 .build());
 
    let shot_increase_powerup = gen_shot_increase(0.0, 0.0, &mut rng);
    let mut shot_increase_spawner = Spawner::new();
    shot_increase_spawner.push(shot_increase_powerup);

    let mut plan = SpawnPlan::new();
    let mut spawner = Spawner::new();
    let minion = PrefabBuilder::new()
        .physical(PhysicalBuilder::default()
                  .xvel(-0.0)
                  .build().unwrap())
        .team(Team{team:1})
        .despawn_far_left(DespawnFarLeft{})
        .despawn_y(DespawnY{})
        .drawable(DrawableBuilder::default()
                  .texture_by_name("boss001_minion.png".to_string())
                  .build().unwrap())
        .timeout_death(TimeoutDeathBuilder::default()
                       .ticks((FRAME_RATE * 100.0) as i32)
                       .build().unwrap())
        .collidable(Collidable{radius:20.0})
        .sine_movement(SineMovementBuilder::default()
                       .amplitude(10.0)
                       .frequency(1.0)
                       .build().unwrap())
        .sine_movement_x(SineMovementXBuilder::default()
                         .amplitude(20.0)
                         .frequency(0.9)
                         .build().unwrap())
        .shield(ShieldBuilder::default()
                .ammount(30.0)
                .build().unwrap())
        .bullet(Bullet{damage: 50.0})
        .death_event(DeathEventBuilder::default()
                     .score_add(10)
                     .sound_by_name("boss001_minion_death.wav".to_string())
                     .spawner(Arc::new(null_powerup_spawner))
                     .build().unwrap())
        .weapon(WeaponBuilder::default()
                .fire_angle(360.0)
                .fire_rate(3.0)
                .fire_velocity(-270.0)
                .offset(-00.0)
                .gun_cooldown_frames(3.0)
                .pattern(8)
                .prefab(PrefabBuilder::new()
                        .physical(PhysicalBuilder::default().build().unwrap())
                        .collidable(Collidable{radius:4.0})
                        .shield(ShieldBuilder::default()
                                .ammount(1.0)
                                .build().unwrap())
                        .timeout_death(TimeoutDeathBuilder::default()
                                       .ticks((FRAME_RATE*10.0) as i32)
                                       .build().unwrap())
                        .bullet(Bullet{damage:8.0})
                        .despawn_far_left(DespawnFarLeft{})
                        .despawn_far_right(DespawnFarRight{})
                        .despawn_y(DespawnY{})
                        .team(Team{team:1})
                        .drawable(DrawableBuilder::default()
                                  .layer(1.0)
                                  .texture_by_name("red_ball.png".to_string())
                                  .build().unwrap())
                        .build())
                .build().unwrap())
        .auto_fire(AutoFire{})
        .build();
    let boss = PrefabBuilder::new()
        .auto_fire(AutoFire{})
        .drawable(DrawableBuilder::default()
                  .layer(1.0)
                  .texture_by_name("boss001.png".to_string())
                  .build().unwrap())
        .physical(PhysicalBuilder::default()
                  .x(1400.0)
                  .y(400.0)
                  .xvel(-100.0)
                  .build().unwrap())
        .stop_at( StopAtBuilder::default()
                  .xloc(1000.0)
                  .build().unwrap())
        .sine_movement(SineMovementBuilder::default()
                       .amplitude(200.0)
                       .frequency(0.2)
                       .build().unwrap())
        .sine_movement_x(SineMovementXBuilder::default()
                       .amplitude(100.0)
                       .frequency(0.05)
                       .build().unwrap())
        .team(Team{team:1})
        .death_event(DeathEventBuilder::default()
                     .score_add(1000)
                     .spawner(Arc::new(shot_increase_spawner))
                     .clear_spawn_plan(true)
                     .build().unwrap())
        .weapon( WeaponBuilder::default()
                 .fire_rate(6.0)
                 .fire_angle(120.0)
                 .fire_velocity(-60.0)
                 .gun_cooldown_frames(4.0)
                 .fire_sound("boss001_shot.wav".to_string())
                 .prefab(minion.clone())
                 .pattern(3)
                 .offset(-0.0)
                 .build().unwrap())
        .collidable(Collidable{radius: 60.0})
        .boss_health_draw(BossHealthDrawBuilder::default().build().unwrap())
        .shield(ShieldBuilder::default()
                .ammount(500.0 + 10.0 * difficulty as f32)
                .build().unwrap())
        .build();

    spawner.push(boss);
    plan.add(start_frame + 1, spawner);
    plan.add(start_frame + 1000000000, Spawner::new());
    return plan;
}

pub fn gen_level_normal(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut weights = vec![ 
        Weighted{ weight: 5, item: (gen_enemy_4 as GenThing, 2.0) },
        Weighted{ weight: 1, item: (gen_enemy_1 as GenThing, 30.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 30.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 90.0) },
        Weighted{ weight: 1, item: (gen_enemy_5 as GenThing, 20.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    return gen_level_from_weights(difficulty, length, start_frame, &mut rng, &mut weights);
}



pub fn gen_level(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    if start_frame == 0 {
        return gen_first_level(difficulty, length, start_frame, &mut rng);
    }

    let mut weights = Vec::<Weighted<GenLevel>>::new();
    if start_frame as f32 > 0.0 {
        weights.push( Weighted{ weight: 2, item: gen_level_simple as GenLevel });
    }

    if start_frame as f32 > 30.0 * FRAME_RATE {
        weights.push( Weighted{ weight: 5, item: gen_level_normal as GenLevel } );
    }

    if start_frame as f32 > 60.0 * FRAME_RATE {
        weights.push( Weighted{ weight: 3, item: gen_level_bomber as GenLevel } );
    }

    if start_frame as f32 > 90.0 * FRAME_RATE {
        weights.push(Weighted{ weight: 1, item: gen_level_bad_upgrade as GenLevel });
    }

    let chooser = WeightedChoice::new(&mut weights);

    return chooser.ind_sample(&mut rng)(difficulty, length, start_frame, &mut rng);
}

pub fn gen_first_level(_difficulty: f32, _length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut ret = SpawnPlan::new();

    let mut spawner = Spawner::new();
    spawner.push(gen_player());
    if start_frame == 0 {
        ret.insert(0, vec![spawner.clone()]);
    }

    spawner = Spawner::new();
    spawner.push(gen_random_upgrade(1350.0, 400.0, &mut rng));

    ret.add( 1, spawner);


    for i in 0..3 {
        spawner = Spawner::new();
        for _ in 0..20 {
            spawner.push(
                gen_enemy_4( rng.gen_range(1350.0, 1450.0), rng.gen_range(0.0, 750.0), &mut rng));
        }
        ret.add((0.75*FRAME_RATE * i as f32) as u64,  spawner);
    }


    return ret;
}

pub fn gen_star_spawner(start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let fun = |dist: f32, rng: &mut rand::isaac::Isaac64Rng| {
        let color_base = 1.0 - 1.0 * (dist * dist);
        let color_byte = (color_base * 255.0) as u8;
        PrefabBuilder::new()
            .drawable(DrawableBuilder::default()
                      .layer(0.0)
                      .texture_by_name("background_star.png".to_string())
                      .tint(Color{r:color_byte, g: color_byte, b: color_byte, a: color_byte})
                      .build().unwrap())
            .despawn_far_left(DespawnFarLeft{})
            .physical(PhysicalBuilder::default()
                      .x(1300.0)
                      .y(rng.gen_range::<f32>(0.0,750.0))
                      .xvel(-250.0 * (color_base * color_base))
                      .build().unwrap())
            .build()
    };
    let density = 20;

    let mut to_spawn = 1000;
    let mut cur_frame = start_frame;

    let mut ret = SpawnPlan::new();

    if start_frame == 0 {
        to_spawn = 50;
    }

    while to_spawn != 0 && to_spawn < 10000 {
        let step = rng.gen_range(0,density);
        cur_frame += step;
        to_spawn -= 1;

        let mut spawner = Spawner::new();
        if start_frame == 0 {
            let mut star = fun(rng.gen_range(0.0, 1.0), &mut rng);
            if let Some(ref mut phy) = star.physical {
                phy.x = rng.gen_range(0.0, 1300.0);
            }
            spawner.push(star);
            ret.add(1, spawner);
        } else {
            spawner.push(fun(rng.gen_range(0.0, 1.0), &mut rng));
            ret.add(cur_frame as u64, spawner);
        }
    }
    return ret;
}
