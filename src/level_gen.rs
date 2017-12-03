#[allow(unused)]
use *;

use rand;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Prefab {
    drawable: Option<Drawable>,
    physical: Option<Physical>,
    collidable: Option<Collidable>,
    controllable: Option<PlayerControl>,
    bullet: Option<Bullet>,
    shield: Option<Shield>,
    despawn_left: Option<DespawnFarLeft>,
    despawn_right: Option<DespawnFarRight>,
    powerup: Option<Powerup>,
    player: Option<PlayerStats>,
    weapon: Option<Weapon>,
    auto_fire: Option<AutoFire>,
    sine: Option<SineMovement>,
}

impl Prefab {
    fn spawn(&self, gd: &mut GameData){
        let id = gd.alloc_id();

        if let Some(val) = self.drawable.clone() {
            gd.drawable_list.add(id, val);
        }
        if let Some(val) = self.physical.clone() {
            gd.physical_list.add(id,val);
        }
        if let Some(val) = self.collidable.clone() {
            print!("adding {} to collidable\n", id);
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
    }
}

#[derive(Clone)]
pub struct PrefabBuilder {
    thing: Prefab,
}

impl PrefabBuilder{
    fn new() -> Self {
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
        }}
    }

    fn drawable(mut self, val:Drawable) -> Self {
        self.thing.drawable = Some(val);
        self
    }
    fn physical(mut self, val:Physical) -> Self {
        self.thing.physical = Some(val);
        self
    }
    fn collidable(mut self, val:Collidable) -> Self {
        self.thing.collidable = Some(val);
        self
    }
    fn controllable(mut self, val:PlayerControl) -> Self {
        self.thing.controllable = Some(val);
        self
    }
    fn bullet(mut self, val:Bullet) -> Self {
        self.thing.bullet = Some(val);
        self
    }
    fn shield(mut self, val:Shield) -> Self {
        self.thing.shield = Some(val);
        self
    }
    fn despawn_left(mut self, val:DespawnFarLeft) -> Self {
        self.thing.despawn_left = Some(val);
        self
    }
    fn despawn_right(mut self, val:DespawnFarRight) -> Self {
        self.thing.despawn_right = Some(val);
        self
    }
    fn powerup(mut self, val:Powerup) -> Self {
        self.thing.powerup = Some(val);
        self
    }
    fn player_stats(mut self, val:PlayerStats) -> Self {
        self.thing.player = Some(val);
        self
    }
    fn weapon(mut self, val:Weapon) -> Self {
        self.thing.weapon = Some(val);
        self
    }
    fn auto_fire(mut self, val:AutoFire) -> Self {
        self.thing.auto_fire = Some(val);
        self
    }
    fn sine_movement(mut self, val:SineMovement) -> Self {
        self.thing.sine = Some(val);
        self
    }

    fn build(self) -> Prefab {
        return self.thing
    }

    fn clone_build(&self) -> Prefab {
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
        .drawable(DrawableBuilder::new()
                  .texture_by_name("player.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(100.0)
                           .y(200.0)
                           .build())
        .controllable(PlayerControl{})
        .collidable(Collidable{ radius: 40.0})
        .player_stats( PlayerStats{
            movement_speed: 15.0,
            owned: vec![]
        })
        .shield( ShieldBuilder::new()
                 .regen(0.01)
                 .ammount(30.0)
                 .build())
        .weapon( WeaponBuilder::new()
                 .fire_rate(0.6*FRAME_RATE)
                 .to_spawn(Bullet{damage: 10.0})
                 .fire_velocity(0.4)
                 .direction(1.0)
                 .offset(80.0)
                 .gun_cooldown_frames(1)
                 .drawable(DrawableBuilder::new()
                           .texture_by_name("red_ball.png".to_string())
                           .layer(1.0)
                           .build())
                 .build())
        .build()
}

fn gen_enemy_1(x: f32, y: f32) -> Prefab{
    PrefabBuilder::new()
        .drawable(DrawableBuilder::new()
                  .texture_by_name("enemy1.png".to_string())
                  .layer(1.0)
                  .build())
        .physical(PhysicalBuilder::new()
                           .x(x)
                           .y(y)
                           .xvel(-0.025)
                           .build())
        .auto_fire(AutoFire{})
        .collidable(Collidable{ radius: 40.0})
        .shield( ShieldBuilder::new()
                 .ammount(11.0)
                 .build())
        .sine_movement( SineMovementBuilder::new()
                        .amplitude(40.0)
                        .frequency(0.5)
                        .build())
        .weapon( WeaponBuilder::new()
                 .to_spawn(Bullet{damage: 10.0})
                 .fire_rate(2.0*FRAME_RATE)
                 .fire_velocity(0.4)
                 .direction(-1.0)
                 .offset(80.0)
                 .gun_cooldown_frames(1)
                 .drawable(DrawableBuilder::new()
                           .texture_by_name("red_ball.png".to_string())
                           .layer(1.0)
                           .build())
                 .build())
        .build()
}

//*
pub fn gen_level(difficulty: f32, length: f32) -> HashMap<u64, Vec<Spawner>>{
    let mut ret = HashMap::<u64,Vec<Spawner>>::new();

    let rng = rand::isaac::Isaac64Rng::new_unseeded();

    let mut spawner = Spawner::new();
    spawner.prefabs.push(gen_player());
    spawner.prefabs.push(gen_enemy_1(800.0, 200.0));
    spawner.prefabs.push(gen_enemy_1(800.0, 300.0));
    ret.insert(0, vec![spawner]);
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
