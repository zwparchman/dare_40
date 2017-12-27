#[allow(unused)]
use *;

use rand;
use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[derive(Clone)]
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
        .drawable(DrawableBuilder::new()
                  .texture_by_name("player.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(50.0)
                           .y(200.0)
                           .build())
        .controllable(PlayerControl{})
        .collidable(Collidable{ radius: 20.0})
        .player_stats( PlayerStats{
            movement_speed: 350.0,
            base_speed: 350.0,
            owned: vec![],
            install_progress: 0,
            install_finish_sound: load_sound("upgrade-finished.wav".to_string()).unwrap()
        })
        .shield( ShieldBuilder::new()
                 .regen(1.00)
                 .ammount(30.0)
                 .build())
        .weapon( WeaponBuilder::new()
                 .fire_angle(60.0)
                 .fire_rate(0.2*FRAME_RATE)
                 .fire_velocity(300.0)
                 .fire_sound("laser001.wav".to_string())
                 .prefab(PrefabBuilder::new()
                           .physical(PhysicalBuilder::new().build())
                           .team(Team{team:0})
                           .bullet(Bullet{damage: 10.0})
                           .despawn_right(DespawnFarRight{})
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .offset(40.0)
                 .gun_cooldown_frames(1)
                 .build())
        .team(Team{team:0})
        .build()
}

#[allow(unused)]
fn gen_shot_increase(x: f32, y: f32, mut rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    let mut base = gen_fire_damage_increase(x,y,&mut rng);

    base.powerup = Some(PowerupBuilder::new()
        .sound_by_name("item-pickup.wav".to_string())
        .shot_increase(1)
        .build());
    base.drawable = Some(DrawableBuilder::new()
                         .texture_by_name("shot-number-increase.png".to_string())
                         .layer(1.0)
                         .build());
    return base;
}

fn gen_fire_damage_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("damage-up.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build())
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("item-pickup.wav".to_string())
                 .fire_damage_increase(1.05)
                 .build())
        .build()
}

fn gen_regen_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("shield-regen.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build())
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("item-pickup.wav".to_string())
                 .regen_increase(1.05)
                 .build())
        .build()
}

fn gen_shield_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("shield-up.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build())
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("item-pickup.wav".to_string())
                 .shield_increase(1.05)
                 .build())
        .build()
}

fn gen_fire_rate_increase(x: f32, y: f32, _rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("fire-rate-up.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-200.0)
                  .build())
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("item-pickup.wav".to_string())
                 .fire_rate_increase(0.95)
                 .build())
        .build()
}

fn gen_enemy_2(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy2.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(x)
                           .y(y)
                           .xvel(rng.gen_range(-200.0, -150.0))
                           .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::new()
                     .score_add(20)
                     .sound_by_name("explosion001.wav".to_string())
                     .build())
        .shield( ShieldBuilder::new()
                 .ammount(11.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(rng.gen_range(20.0, 40.0))
                        .frequency(rng.gen_range(0.5, 2.0))
                        .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(3.0*FRAME_RATE+rng.next_f32()* 0.5)
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 10.0})
                           .physical(PhysicalBuilder::new().build())
                           .collidable(Collidable{radius: 4.0})
                           .sine_movement(SineMovementBuilder::new()
                                          .amplitude(30.0)
                                          .frequency(2.0)
                                          .build())
                           .sine_movement_x(SineMovementXBuilder::new()
                                            .amplitude(30.0)
                                            .frequency(2.5)
                                            .build())
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("green-ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .fire_velocity(rng.gen_range(-300.0, -280.0))
                 .offset(-10.0)
                 .gun_cooldown_frames((FRAME_RATE * rng.gen_range(2.0,4.0)) as i32)
                 .build())
        .team(Team{team:1})
        .build()
}

fn gen_enemy_3(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy3.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(x)
                           .y(y)
                           .xvel(-100.5+ rng.next_f32()*0.01)
                           .build())
        .stop_at(StopAtBuilder::new()
                 .xloc(900.0+rng.next_f32()*100.0)
                 .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::new()
                     .sound_by_name("explosion001.wav".to_string())
                     .score_add(50)
                     .build())
        .shield( ShieldBuilder::new()
                 .ammount(30.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(rng.gen_range(300.0,320.0))
                        .frequency(rng.gen_range(0.05,1.0))
                        .build())
        .sine_movement_x( SineMovementXBuilder::new()
                          .amplitude(rng.gen_range(50.0, 150.0))
                          .frequency(rng.gen_range(0.05, 0.30))
                          .build())
        .weapon( WeaponBuilder::new()
                 .pattern(2)
                 .fire_angle(rng.gen_range(90.0, 120.0))
                 .fire_rate(1.0*FRAME_RATE+rng.next_f32()* 0.5)
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 2.0})
                           .sine_movement(SineMovementBuilder::new()
                                          .amplitude(20.0)
                                          .frequency(3.0)
                                          .build())
                           .physical(PhysicalBuilder::new().build())
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("orange-ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .fire_velocity(rng.gen_range(-300.0, -200.0))
                 .offset(-10.0)
                 .gun_cooldown_frames((FRAME_RATE * rng.gen_range(1.0, 3.0)) as i32)
                 .build())
        .team(Team{team:1})
        .build()
}

fn gen_enemy_1(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy1.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(x)
                           .y(y)
                           .xvel(rng.gen_range(-200.0,-150.0))
                           .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::new()
                     .sound_by_name("explosion001.wav".to_string())
                     .score_add(10)
                     .build())
        .shield( ShieldBuilder::new()
                 .ammount(11.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(rng.gen_range(20.0,40.0))
                        .frequency(rng.gen_range(0.5,2.0))
                        .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(rng.gen_range(3.0*FRAME_RATE,4.0*FRAME_RATE))
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 10.0})
                           .physical(PhysicalBuilder::new().build())
                           .collidable(Collidable{radius: 4.0})
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .fire_velocity(-300.0 - rng.next_f32() * 0.2)
                 .offset(-10.0)
                 .gun_cooldown_frames((FRAME_RATE * rng.gen_range(1.0, 3.0)) as i32)
                 .build())
        .team(Team{team:1})
        .build()
}

fn gen_bomb(rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    let sub_munition_prefab = PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("yellow-ball.png".to_string())
                  .layer(1.0)
                  .build())
        .timeout_death(TimeoutDeathBuilder::new()
                       .ticks(10000)
                       .build())
        .collidable(Collidable{radius: 4.0})
        .despawn_left(DespawnFarLeft{})
        .despawn_right(DespawnFarRight{})
        .team(Team{team:1})
        .bullet(Bullet{damage: 3.0});

    let get_sub = |prefab: &PrefabBuilder, mut angle: f32, vel: f32| {
        angle *= DEG2RAD as f32;
        prefab.clone()
            .physical(PhysicalBuilder::new()
                      .xvel(angle.cos() * -1.0 * vel)
                      .yvel(angle.sin() * vel)
                      .build())
            .build()
    };

    let mut spawner= Spawner::new();
    for angle in get_shot_angles( rng.gen_range(200.0, 360.0), rng.gen_range(5, 10)) {
        spawner.push( get_sub(&sub_munition_prefab, angle, rng.gen_range(100.0, 150.0)));
    }

    //the actuall bomb
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("bomb.png".to_string())
                  .layer(2.0)
                  .build())
        .physical(PhysicalBuilder::new().build())
        .timeout_death(TimeoutDeathBuilder::new()
                       .ticks( (4.0*FRAME_RATE) as i32)
                       .build())
        .death_event(DeathEventBuilder::new()
                     .spawner(Arc::new(spawner))
                     .sound_by_name("bomb-explode.wav".to_string())
                     .build())
        .build()
}

fn gen_enemy_5(x: f32, y: f32, mut rng: &mut rand::isaac::Isaac64Rng) -> Prefab {
    let bomb = gen_bomb(&mut rng);

    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy5.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-50.0)
                  .build())
        .auto_fire(AutoFire{})
        .shield(ShieldBuilder::new()
                .ammount(50.0)
                .build())
        .sine_movement(SineMovementBuilder::new()
                       .amplitude(5.0)
                       .frequency(1.5)
                       .build())
        .weapon(WeaponBuilder::new()
                .prefab(bomb)
                .fire_rate(rng.gen_range(4.0, 5.0)*FRAME_RATE)
                .fire_sound("bomb-launch.wav".to_string())
                .fire_velocity(rng.gen_range(50.0, 100.0)*-1.0)
                .gun_cooldown_frames( (5.0*FRAME_RATE) as i32 )
                .offset(-40.0)
                .build())
        .collidable(Collidable{radius: 30.0})
        .death_event(DeathEventBuilder::new()
                     .score_add(5)
                     .sound_by_name("explosion002.wav".to_string())
                     .build())
        .despawn_left(DespawnFarLeft{})
        .team(Team{team:1})
        .build()
}

fn gen_enemy_4(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    let base_builder = PrefabBuilder::new()
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("bad-pickup.wav".to_string())
                 .build())
        .drawable(DrawableBuilder::new()
                  .texture_by_name("null-powerup.png".to_string())
                  .build());

    let phy_builder = PhysicalBuilder::new()
        .xvel(-100.0);

    let mut spawner = Spawner::new();
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(5.0)
                           .build())
                 .build());
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(0.0)
                           .build())
                 .build());
    spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(-5.0)
                           .build())
                 .build());
    
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy4.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(x)
                           .y(y)
                           .xvel(-150.5+ rng.next_f32()*15.0)
                           .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 14.0})
        .despawn_left(DespawnFarLeft{})
        .death_event(DeathEventBuilder::new()
                     .sound_by_name("explosion-small.wav".to_string())
                     .spawner(Arc::new(spawner))
                     .score_add(5)
                     .build())
        .shield( ShieldBuilder::new()
                 .ammount(1.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(rng.gen_range(0.0,40.0))
                        .frequency(rng.gen_range(1.0, 3.0))
                        .build())
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

    let mut len_left = length - rng.gen_range(0.0, 100.0);
    while len_left > 0.0 {
        let mut spawner = Spawner::new();
        let mut cur_diff = difficulty;
        while cur_diff > 0.0 {
            let (fun,dif) = chooser.ind_sample(&mut rng);
            cur_diff -= dif;
            spawner.push(fun(rng.gen_range(1400.0, 1500.0), rng.gen_range(0.0, 700.0), &mut rng));
        }

        let offset = rng.gen_range(0.0, 500.0);
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

pub fn gen_boss_1_level(difficulty: f32, _length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    print!("boss level!\n");
    let base_builder = PrefabBuilder::new()
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .sound_by_name("bad-pickup.wav".to_string())
                 .build())
        .drawable(DrawableBuilder::new()
                  .texture_by_name("null-powerup.png".to_string())
                  .build());

    let phy_builder = PhysicalBuilder::new()
        .xvel(-100.0);

    let mut null_powerup_spawner = Spawner::new();
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(5.0)
                           .build())
                 .build());
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(0.0)
                           .build())
                 .build());
    null_powerup_spawner.push(base_builder.clone()
                 .physical(phy_builder.clone()
                           .yvel(-5.0)
                           .build())
                 .build());
 
    let shot_increase_powerup = gen_shot_increase(0.0, 0.0, &mut rng);
    let mut shot_increase_spawner = Spawner::new();
    shot_increase_spawner.push(shot_increase_powerup);

    let mut plan = SpawnPlan::new();
    let mut spawner = Spawner::new();
    let minion = PrefabBuilder::new()
        .physical(PhysicalBuilder::new()
                  .xvel(-0.0)
                  .build())
        .team(Team{team:1})
        .despawn_left(DespawnFarLeft{})
        .drawable(DrawableBuilder::new()
                  .texture_by_name("boss001_minion.png".to_string())
                  .build())
        .timeout_death(TimeoutDeathBuilder::new()
                       .ticks((FRAME_RATE * 100.0) as i32)
                       .build())
        .collidable(Collidable{radius:20.0})
        .sine_movement(SineMovementBuilder::new()
                       .amplitude(10.0)
                       .frequency(1.0)
                       .build())
        .sine_movement_x(SineMovementXBuilder::new()
                         .amplitude(20.0)
                         .frequency(0.9)
                         .build())
        .shield(ShieldBuilder::new()
                .ammount(30.0)
                .build())
        .bullet(Bullet{damage: 50.0})
        .death_event(DeathEventBuilder::new()
                     .score_add(10)
                     .sound_by_name("boss001_minion_death.wav".to_string())
                     .spawner(Arc::new(null_powerup_spawner))
                     .build())
        .weapon(WeaponBuilder::new()
                .fire_angle(360.0)
                .fire_rate(3.0 * FRAME_RATE)
                .fire_velocity(-270.0)
                .offset(-00.0)
                .gun_cooldown_frames((FRAME_RATE * 3.0) as i32)
                .pattern(8)
                .prefab(PrefabBuilder::new()
                        .physical(PhysicalBuilder::new().build())
                        .collidable(Collidable{radius:4.0})
                        .shield(ShieldBuilder::new()
                                .ammount(1.0)
                                .build())
                        .timeout_death(TimeoutDeathBuilder::new()
                                       .ticks((FRAME_RATE*10.0) as i32)
                                       .build())
                        .bullet(Bullet{damage:8.0})
                        .despawn_left(DespawnFarLeft{})
                        .despawn_right(DespawnFarRight{})
                        .team(Team{team:1})
                        .drawable(DrawableBuilder::new()
                                  .layer(1.0)
                                  .texture_by_name("red_ball.png".to_string())
                                  .build())
                        .build())
                .build())
        .auto_fire(AutoFire{})
        .build();
    let boss = PrefabBuilder::new()
        .auto_fire(AutoFire{})
        .drawable(DrawableBuilder::new()
                  .layer(1.0)
                  .texture_by_name("boss001.png".to_string())
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(1400.0)
                  .y(400.0)
                  .xvel(-100.0)
                  .build())
        .stop_at( StopAtBuilder::new()
                  .xloc(1000.0)
                  .build())
        .sine_movement(SineMovementBuilder::new()
                       .amplitude(200.0)
                       .frequency(0.2)
                       .build())
        .sine_movement_x(SineMovementXBuilder::new()
                       .amplitude(100.0)
                       .frequency(0.05)
                       .build())
        .team(Team{team:1})
        .death_event(DeathEventBuilder::new()
                     .score_add(1000)
                     .spawner(Arc::new(shot_increase_spawner))
                     .clear_spawn_plan(true)
                     .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(6.0*FRAME_RATE)
                 .fire_angle(120.0)
                 .fire_velocity(-60.0)
                 .gun_cooldown_frames((FRAME_RATE*4.0) as i32)
                 .fire_sound("boss001_shot.wav".to_string())
                 .prefab(minion.clone())
                 .pattern(3)
                 .offset(-0.0)
                 .build())
        .collidable(Collidable{radius: 60.0})
        .boss_health_draw(BossHealthDrawBuilder::new().build())
        .shield(ShieldBuilder::new()
                .ammount(500.0 + 10.0 * difficulty as f32)
                .build())
        .build();

    spawner.push(boss);
    plan.add(start_frame + 1, spawner);
    plan.add(start_frame + 1000000000, Spawner::new());
    return plan;
}

pub fn gen_level_normal(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut weights = vec![ 
        Weighted{ weight: 5, item: (gen_enemy_4 as GenThing, 2.0) },
        Weighted{ weight: 1, item: (gen_enemy_1 as GenThing, 10.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 30.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 50.0) },
        Weighted{ weight: 1, item: (gen_enemy_5 as GenThing, 20.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    return gen_level_from_weights(difficulty, length, start_frame, &mut rng, &mut weights);
}



pub fn gen_level(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    if start_frame == 0 {
        return gen_first_level(difficulty, length, start_frame, &mut rng);
    }

    let mut weights = vec![
        // Weighted{ weight: 9999, item: gen_boss_1_level as GenLevel },
        Weighted{ weight: 1, item: gen_level_bomber as GenLevel },
        Weighted{ weight: 1, item: gen_level_bad_upgrade as GenLevel },
        Weighted{ weight: 2, item: gen_level_simple as GenLevel },
        Weighted{ weight: 5, item: gen_level_normal as GenLevel },
    ];

    let chooser = WeightedChoice::new(&mut weights);

    return chooser.ind_sample(&mut rng)(difficulty, length, start_frame, &mut rng);
}

pub fn gen_first_level(difficulty: f32, length: f32, start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let mut ret = SpawnPlan::new();

    let mut spawner = Spawner::new();
    spawner.prefabs.push(gen_player());
    if start_frame == 0 {
        ret.insert(0, vec![spawner.clone()]);
    }

    let mut weights = vec![ 
        Weighted{ weight: 5, item: (gen_enemy_4 as GenThing, 10.0) },
        Weighted{ weight: 1, item: (gen_enemy_1 as GenThing, 20.0)},
        Weighted{ weight: 1, item: (gen_enemy_2 as GenThing, 40.0) },
        Weighted{ weight: 1, item: (gen_enemy_3 as GenThing, 60.0) },
        Weighted{ weight: 1, item: (gen_enemy_5 as GenThing, 30.0) },
        Weighted{ weight: 2, item: (gen_random_upgrade as GenThing, 1.0) },
    ]; 

    let chooser = WeightedChoice::new(&mut weights);

    let mut len_left = length - rng.gen_range(0.0, 100.0);
    while len_left > 0.0 {
        spawner = Spawner::new();
        let mut cur_diff = difficulty;
        while cur_diff > 0.0 {
            let (fun,dif) = chooser.ind_sample(&mut rng);
            cur_diff -= dif;
            spawner.push(fun(rng.gen_range(1400.0, 1500.0), rng.gen_range(0.0, 700.0), &mut rng));
        }

        let offset = rng.gen_range(0.0, 500.0);
        let when: u64 = start_frame + len_left as u64;

        // print!("when {}\n", when);
        ret.add(when, spawner);
        len_left -= offset;
    }
    return ret;
}

pub fn gen_star_spawner(start_frame: u64, mut rng: &mut rand::isaac::Isaac64Rng) -> SpawnPlan {
    let fun = |dist: f32, rng: &mut rand::isaac::Isaac64Rng| {
        let color_base = 1.0 - 1.0 * (dist * dist);
        let color_byte = (color_base * 255.0) as u8;
        PrefabBuilder::new()
            .drawable(DrawableBuilder::new()
                      .layer(0.0)
                      .texture_by_name("background_star.png".to_string())
                      .tint(Color{r:color_byte, g: color_byte, b: color_byte, a: color_byte})
                      .build())
            .despawn_left(DespawnFarLeft{})
            .physical(PhysicalBuilder::new()
                      .x(1300.0)
                      .y(rng.gen_range::<f32>(0.0,750.0))
                      .xvel(-250.0 * (color_base * color_base))
                      .build())
            .build()
    };
    let density = 20;

    let mut to_spawn = 1000;
    let mut cur_frame = start_frame;

    let mut ret = SpawnPlan::new();

    while to_spawn != 0 || to_spawn > 1000 {
        let step = rng.gen_range(0,density);
        cur_frame += step;
        to_spawn -= 1;

        let mut spawner = Spawner::new();
        if start_frame == 0 {
            to_spawn -= 3;
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
