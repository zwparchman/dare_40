#[allow(unused)]
use *;

use rand;
use rand::Rng;
use std::collections::HashMap;

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
}

impl Prefab {
    pub fn spawn(&self, gd: &mut EcsWorld) -> id_type{
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

    pub fn build(self) -> Prefab {
        return self.thing
    }
}

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
                 .regen(0.10)
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
                 .gun_cooldown_frames(rng.gen_range(1,80))
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
                 .gun_cooldown_frames(1)
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
                 .gun_cooldown_frames(1)
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
                     .sound_by_name("explosion001.wav".to_string())
                     .spawner(Arc::new(spawner))
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

//*
pub fn gen_level(_difficulty: f32, _length: f32) -> HashMap<u64, Vec<Spawner>>{
    let mut ret = HashMap::<u64,Vec<Spawner>>::new();

    let mut rng = rand::isaac::Isaac64Rng::new_unseeded();

    let mut spawner = Spawner::new();
    spawner.prefabs.push(gen_player());
    ret.insert(0, vec![spawner.clone()]);

    for i in 1..1000 {
        spawner = Spawner::new();
        for _j in 0..(rng.gen_range(0.0,30.0)/10.0/i as f32) as i32 {
            spawner.prefabs.push(gen_enemy_1(rng.gen_range(1400.0, 1500.0), rng.gen_range(0.0, 700.0), &mut rng));
        }
        spawner.prefabs.push(gen_enemy_2(rng.gen_range(1400.0, 1500.0), rng.gen_range(0.0, 700.0), &mut rng));
        spawner.prefabs.push(gen_enemy_3(rng.gen_range(1400.0, 1500.0), rng.gen_range(300.0, 400.0), &mut rng));
        for _ in 1..3 {
            spawner.prefabs.push(gen_enemy_4(rng.gen_range(1400.0, 1500.0), rng.gen_range(100.0, 600.0), &mut rng));
        }
        spawner.prefabs.push(gen_enemy_5(rng.gen_range(1400.0, 1500.0), rng.gen_range(100.0, 600.0), &mut rng));
        spawner.prefabs.push( gen_random_upgrade(1400.0, rng.gen_range(0.0, 700.0), &mut rng));

        let when: u64 = 1 + (FRAME_RATE * 4.0 * (i-1) as f32) as u64;
        ret.insert( when, vec![spawner.clone()]);
    }
    /*
    let mut cur = 0.0;

    while cur < length {
        ret.insert(0, vec![]);
        cur += length;
    }
    */

    return ret;
}
// */
