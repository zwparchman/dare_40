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
    pub team: Option<Team>,
    pub install: Option<Install>
}

impl Prefab {
    pub fn spawn(&self, gd: &mut GameData) -> id_type{
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
        if let Some(val) = self.team.clone() {
            gd.team_list.add(id,val);
        }
        if let Some(val) = self.install.clone() {
            gd.install_list.add(id,val);
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
            team: None,
            install: None,
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
    pub fn team(mut self, val:Team) -> Self {
        self.thing.team = Some(val);
        self
    }
    pub fn install(mut self, val:Install) -> Self {
        self.thing.install = Some(val);
        self
    }



    pub fn build(self) -> Prefab {
        return self.thing
    }

    pub fn clone_build(&self) -> Prefab {
        return self.thing.clone()
    }
}

#[derive(Clone)]
pub struct Spawner {
    prefabs: Vec<Prefab>,
}

impl Spawner {
    fn new() -> Self {
        Self {prefabs: vec![]}
    }

    pub fn spawn(&self , mut gd: &mut GameData){
        for fab in self.prefabs.clone() {
            fab.spawn(&mut gd);
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
        .collidable(Collidable{ radius: 40.0})
        .player_stats( PlayerStats{
            movement_speed: 15.0,
            base_speed: 15.0,
            owned: vec![],
            install_progress: 0,
        })
        .shield( ShieldBuilder::new()
                 .regen(0.01)
                 .ammount(30.0)
                 .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(0.2*FRAME_RATE)
                 .prefab(PrefabBuilder::new()
                           .physical(PhysicalBuilder::new().build())
                           .team(Team{team:0})
                           .bullet(Bullet{damage: 10.0})
                           .despawn_right(DespawnFarRight{})
                           .collidable(Collidable{radius: 8.0})
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .fire_velocity(0.4)
                 .offset(80.0)
                 .gun_cooldown_frames(1)
                 .build())
        .team(Team{team:0})
        .build()
}

fn gen_fire_rate_increase(x: f32, y: f32, rng: &mut rand::isaac::Isaac64Rng) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("fire_rate_up.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                  .x(x)
                  .y(y)
                  .xvel(-0.1)
                  .build())
        .collidable(Collidable{radius: 20.0})
        .despawn_left(DespawnFarLeft{})
        .powerup(PowerupBuilder::new()
                 .fire_rate_increase(0.95)
                 .build())
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
                           .xvel(-0.05+ rng.next_f32()*0.01)
                           .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 40.0})
        .despawn_left(DespawnFarLeft{})
        .shield( ShieldBuilder::new()
                 .ammount(11.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(20.0 + rng.next_f32()*20.0 )
                        .frequency(0.5 + rng.next_f32() * 2.0 )
                        .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(2.0*FRAME_RATE+rng.next_f32()* 0.5)
                 .prefab(PrefabBuilder::new()
                           .team(Team{team:1})
                           .despawn_left(DespawnFarLeft{})
                           .bullet(Bullet{damage: 10.0})
                           .physical(PhysicalBuilder::new().build())
                           .collidable(Collidable{radius: 8.0})
                           .drawable(DrawableBuilder::new()
                                     .texture_by_name("red_ball.png".to_string())
                                     .layer(1.0)
                                     .build())
                           .build())
                 .fire_velocity(-0.2 - rng.next_f32() * 0.2)
                 .offset(-80.0)
                 .gun_cooldown_frames(1)
                 .build())
        .team(Team{team:1})
        .build()
}

//*
pub fn gen_level(difficulty: f32, length: f32) -> HashMap<u64, Vec<Spawner>>{
    let mut ret = HashMap::<u64,Vec<Spawner>>::new();

    let mut rng = rand::isaac::Isaac64Rng::new_unseeded();

    let mut spawner = Spawner::new();
    spawner.prefabs.push(gen_player());
    ret.insert(0, vec![spawner.clone()]);

    for i in 1..10 {
        spawner = Spawner::new();
        for j in 0..3 {
            //spawner.prefabs.push(gen_enemy_1(1400.0, rng.gen_range(0.0, 700.0), &mut rng));
            spawner.prefabs.push(
                gen_fire_rate_increase(1400.0,
                                       rng.gen_range(0.0, 700.0),
                                       &mut rng));
        }
        ret.insert(150*i, vec![spawner.clone()]);
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
