#[macro_use] extern crate log;
#[macro_use] extern crate cached;
// `cached!` macro requires the `lazy_static!` macro
#[macro_use] extern crate lazy_static;

// extern crate hibitset;
extern crate quickersort;
extern crate nalgebra;
extern crate rand;
extern crate hibitset;
extern crate pretty_env_logger;

use std::f32;

mod ecs_world;
use ecs_world::*;

mod raylib;
use raylib::{Texture2D, Rectangle, DrawTexturePro, Color};

mod level_gen;
#[allow(unused)]
#[allow(bad_style)]
use level_gen::*;

#[allow(unused)]
#[allow(bad_style)]
use raylib::*;

use nalgebra::{Point2,Vector2};
#[allow(unused)]
type  Vector2f = Vector2<f32>;
#[allow(unused)]
type  Point2f = Point2<f32>;

#[allow(non_camel_case_types)]
type IDType=i64;

#[allow(unused)]
use std::sync::{Arc, RwLock, Mutex, MutexGuard, TryLockResult, LockResult};

#[allow(unused)]
use sprite_loader::*;
mod sprite_loader;

use storage::*;
mod storage;

use std::collections::{HashSet};

const FRAME_RATE: f32 = 60.0;
const FRAME_TIME: f32 = 1.0/FRAME_RATE;

const SLOWDOWN_FACTOR: f32 = 0.6;

fn max_float(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min_float(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Clone)]
pub struct StopAt {
    xloc: f32,
}

struct StopAtBuilder {
    thing: StopAt,
}

impl StopAtBuilder {
    fn new() -> Self {
        Self{ thing: StopAt{xloc: -102.0}}
    }
    fn xloc(mut self, val: f32) -> Self {
        self.thing.xloc = val;
        self
    }
    fn build(self) -> StopAt {
        return self.thing
    }
}


#[derive(Clone)]
pub struct SineMovementX {
    frequency: f32,
    step: i32,
    amplitude: f32,
}

struct SineMovementXBuilder {
    thing: SineMovementX,
}

impl SineMovementXBuilder {
    fn new() -> Self {
        SineMovementXBuilder{ thing: SineMovementX{frequency: 1.0, step: 0, amplitude: 0.0} }
    }
    fn frequency(mut self, val: f32) -> Self {
        self.thing.frequency = val;
        self
    }
    fn amplitude(mut self, val: f32) -> Self {
        self.thing.amplitude = val;
        self
    }
    #[allow(unused)]
    fn step(mut self, val: i32) -> Self {
        self.thing.step = val;
        self
    }

    fn build(self) -> SineMovementX {
        return self.thing
    }
}



#[derive(Clone)]
pub struct SineMovement {
    frequency: f32,
    step: i32,
    amplitude: f32,
}

struct SineMovementBuilder {
    thing: SineMovement,
}

impl SineMovementBuilder {
    fn new() -> Self {
        SineMovementBuilder{ thing: SineMovement{frequency: 1.0, step: 0, amplitude: 0.0} }
    }
    fn frequency(mut self, val: f32) -> Self {
        self.thing.frequency = val;
        self
    }
    fn amplitude(mut self, val: f32) -> Self {
        self.thing.amplitude = val;
        self
    }
    #[allow(unused)]
    fn step(mut self, val: i32) -> Self {
        self.thing.step = val;
        self
    }

    fn build(self) -> SineMovement {
        return self.thing
    }
}

#[derive(Clone)]
pub struct Physical{
    x: f32,
    y: f32,
    xvel: f32,
    yvel: f32,
}

#[derive(Clone)]
struct PhysicalBuilder{
    thing: Physical,
}

impl PhysicalBuilder{
    fn new() -> Self {
        PhysicalBuilder{ thing: Physical{x:0.0, y:0.0, xvel:0.0, yvel:0.0}}
    }
    fn x(mut self, val: f32) -> Self{
        self.thing.x = val;
        self
    }
    fn y(mut self, val: f32) -> Self{
        self.thing.y = val;
        self
    }
    fn xvel(mut self, val: f32) -> Self{
        self.thing.xvel = val;
        self
    }
    #[allow(unused)]
    fn yvel(mut self, val: f32) -> Self{
        self.thing.yvel = val;
        self
    }

    fn build(self) -> Physical {
        return self.thing;
    }
}

#[derive(Clone)]
pub struct Drawable{
    texture: Arc<Texture2D>,
    layer: f32,
    tint: Color,
}

struct DrawableBuilder{
    thing: Drawable,
}

impl DrawableBuilder {
    fn new() -> Self {
        DrawableBuilder{ thing: Drawable{
            texture: load_texture("no-texture.png".to_string()).unwrap(),
            layer: 0.1,
            tint: Color{r:255, g: 255, b:255, a:255},
        }}
    }
    fn layer(mut self, val: f32) -> Self{
        self.thing.layer = val;
        self
    }
    fn texture_by_name(mut self, val: String) -> Self{
        self.thing.texture = load_texture(val).unwrap();
        self
    }

    fn tint(mut self, val: Color) -> Self{
        self.thing.tint = val;
        self
    }

    fn build(self) -> Drawable {
        return self.thing;
    }
}

#[derive(Clone)]
pub struct DeathEvent{
    sound: Arc<Sound>,
    spawner: Arc<Spawner>,
    score_add: i64,
    clear_spawn_plan: bool,
}

impl DeathEvent{
    fn die(&self, mut world: &mut EcsWorld, pos: &Physical) -> ( bool, i64) {
        PlaySound(&self.sound);
        self.spawner.spawn_at_pos(&mut world, &pos);
        (self.clear_spawn_plan, self.score_add)
    }
}

struct DeathEventBuilder{
    thing: DeathEvent,
}

impl DeathEventBuilder {
    fn new() -> Self {
        DeathEventBuilder{ thing: DeathEvent {
            sound: load_sound("scilence.wav".to_string()).unwrap(),
            spawner: Arc::new(Spawner::new()),
            score_add: 0,
            clear_spawn_plan: false,
        }}
    }
    fn spawner(mut self, val: Arc<Spawner>) -> Self {
        self.thing.spawner = val;
        self
    }
    fn sound_by_name(mut self, val: String) -> Self{
        self.thing.sound = load_sound(val).unwrap();
        self
    }

    fn score_add(mut self, val: i64) -> Self {
        self.thing.score_add = val;
        self
    }

    fn clear_spawn_plan(mut self, val: bool) -> Self {
        self.thing.clear_spawn_plan = val;
        self
    }

    fn build(self) -> DeathEvent  {
        return self.thing;
    }
}



#[derive(Clone)]
pub struct Collidable{
    radius: f32,
}

#[derive(Clone)]
pub struct PlayerControl{}
#[derive(Clone)]
pub struct DespawnFarRight{}
#[derive(Clone)]
pub struct DespawnFarLeft{}

#[derive(Clone)]
pub struct Bullet{
    damage: f32,
}

#[derive(Clone)]
pub struct Powerup {
    regen_increase: f32,
    fire_rate_increase: f32,
    fire_damage_increase: f32,
    shield_increase: f32,
    pickup_sound: Arc<Sound>,
    shot_increase: i32,
}

#[derive(Clone)]
pub struct PowerupBuilder {
    thing: Powerup,
}

impl PowerupBuilder {
    fn new() -> Self {
        Self { thing: Powerup{
            regen_increase: 1.0,
            fire_rate_increase: 1.0,
            fire_damage_increase: 1.0,
            shield_increase: 1.0,
            pickup_sound: load_sound("silence.wav".to_string()).unwrap(),
            shot_increase: 0,
        }}
    }

    #[allow(unused)]
    fn shot_increase(mut self, val: i32) -> Self {
        self.thing.shot_increase = val;
        self
    }

    fn sound_by_name(mut self, val: String) -> Self {
        self.thing.pickup_sound = load_sound(val).unwrap();
        self
    }

    fn fire_rate_increase(mut self, val: f32) -> Self{
        self.thing.fire_rate_increase = val;
        self
    }

    fn regen_increase(mut self, val: f32) -> Self{
        self.thing.regen_increase = val;
        self
    }

    fn fire_damage_increase(mut self, val: f32) -> Self{
        self.thing.fire_damage_increase = val;
        self
    }

    fn shield_increase(mut self, val: f32) -> Self{
        self.thing.shield_increase = val;
        self
    }

    fn build(self) -> Powerup {
        return self.thing;
    }
}



const PART_INSTALLED_AT: i32 = 120;
#[derive(Clone)]
pub struct PlayerStats {
    movement_speed: f32,
    base_speed: f32,
    owned: Vec<Prefab>,
    install_progress: i32,
    install_finish_sound: Arc<Sound>,
}

#[derive(Clone)]
pub struct Weapon {
    fire_rate: f32,
    fire_velocity: f32,
    gun_cooldown_frames : i32,
    prefab : Arc<Prefab>,
    pattern: i32,

    direction: f32,
    offset: f32,
    fire_sound: Option<Arc<Sound>>,
    fire_angle: f32,
}

#[derive(Clone)]
pub struct WeaponBuilder{
    thing: Weapon,
}

impl WeaponBuilder {
    fn new() -> Self {
        WeaponBuilder{ thing: Weapon{
            fire_rate: 1.0,
            fire_velocity: 0.1,
            gun_cooldown_frames: 0,
            prefab: Arc::new(PrefabBuilder::new().build()),
            direction: 1.0,
            offset: 40.0,
            pattern: 1,
            fire_sound: None,
            fire_angle: 90.0,
        }}
    }

    fn fire_angle(mut self, val: f32) -> Self {
        self.thing.fire_angle = val;
        self
    }

    fn fire_sound(mut self, val: String) -> Self{
        self.thing.fire_sound = load_sound(val);
        self
    }

    fn pattern(mut self, val: i32) -> Self{
        self.thing.pattern = val;
        self
    }

    fn fire_rate(mut self, val: f32) -> Self{
        self.thing.fire_rate = val;
        self
    }

    fn fire_velocity(mut self, val: f32) -> Self{
        self.thing.fire_velocity = val;
        self
    }

    fn gun_cooldown_frames(mut self, val: i32) -> Self{
        self.thing.gun_cooldown_frames = val;
        self
    }

    fn prefab(mut self, val: Prefab) -> Self{
        self.thing.prefab = Arc::new(val);
        self
    }

    fn offset(mut self, val: f32) -> Self{
        self.thing.offset = val;
        self
    }

    fn build(self) -> Weapon {
        return self.thing;
    }

}

#[derive(Clone)]
pub struct BossHealthDraw {
}

pub struct BossHealthDrawBuilder {
    thing: BossHealthDraw,
}

impl BossHealthDrawBuilder {
    fn new() -> Self {
        Self {
            thing: BossHealthDraw{},
        }
    }

    fn build(self) -> BossHealthDraw {
        self.thing
    }
}

#[derive(Clone)]
pub struct TimeoutDeath{
    ticks: i32,
}

pub struct TimeoutDeathBuilder {
    thing: TimeoutDeath,
}

impl TimeoutDeathBuilder {
    fn new() -> Self {
        Self{ thing: TimeoutDeath{
            ticks: 0,
        }}
    }

    fn ticks(mut self, val: i32) -> Self{
        self.thing.ticks = val;
        self
    }

    fn build(self) -> TimeoutDeath {
        return self.thing;
    }
}

#[derive(Clone)]
pub struct Shield {
    max_shield: f32,
    ammount: f32,
    regen: f32,
}

#[derive(Clone)]
pub struct ShieldBuilder {
    thing: Shield,
}

impl ShieldBuilder {
    fn new() -> Self {
        Self{ thing: Shield{
            max_shield: 10.0,
            ammount: 10.0,
            regen: 0.0,
        }}
    }
    #[allow(unused)]
    fn max_shield(mut self, val: f32) -> Self{
        self.thing.max_shield = val;
        self
    }

    fn ammount(mut self, val: f32) -> Self{
        self.thing.ammount = val;
        self.thing.max_shield = max_float(self.thing.max_shield, val);
        self
    }

    fn regen(mut self, val: f32) -> Self{
        self.thing.regen = val;
        self
    }
    fn build(self) -> Shield {
        return self.thing;
    }
}

#[derive(Clone)]
pub struct AutoFire{}

#[derive(Clone)]
pub struct Install{}

#[derive(Clone)]
pub struct Team{team: i32}


pub struct GameData{
    frame_count: u64,

    spawn_plan: SpawnPlan,
    star_spawner: SpawnPlan,

    world: EcsWorld,
    difficulty: f32,
    wave: i32,
    score: i64,
    paused: bool,

    rng: rand::isaac::Isaac64Rng,
}

impl GameData {
    fn new() -> Self {
        use rand::Rng;
        use rand::SeedableRng;
        let seed = rand::thread_rng().gen::<u64>();
        debug!("Seed {}", seed);
        Self{
            frame_count: 0,
            spawn_plan: SpawnPlan::new(),
            star_spawner: SpawnPlan::new(),
            world: EcsWorld::new(),
            difficulty: 50.0,
            score: 0,
            wave: 0,
            // rng: rand::isaac::Isaac64Rng::from_seed(&[1,2,3]),
            rng: rand::isaac::Isaac64Rng::from_seed(&[seed]),
            paused: false,
        }
    }

    fn spawn_stars(&mut self){
        self.star_spawner.execute(self.frame_count, &mut self.world);
        if self.star_spawner.is_empty() {
            self.star_spawner = gen_star_spawner(self.frame_count , &mut self.rng);
        }
    }

    fn spawn_main(&mut self){
        let did_spawn = self.spawn_plan.execute(self.frame_count, &mut self.world);
        if did_spawn {
            trace!("Spawned main on  frame {}", self.frame_count);
        }

        if self.spawn_plan.is_empty() {
            trace!("Creating new main spawn plan on frame:{} difficulty: {} wave: {}", self.frame_count, self.difficulty, self.wave);
            let player_shield_fraction = self.get_player_shield_fraction();
            self.difficulty += 3.0 * player_shield_fraction;
            self.wave += 1;
            if self.wave == 20 {
                self.difficulty += 20.0;
                self.spawn_plan = gen_boss_1_level(self.difficulty,
                                                   500.0,
                                                   self.frame_count ,
                                                   &mut self.rng);
            } else {
                self.spawn_plan = gen_level(self.difficulty,
                                            500.0,
                                            self.frame_count ,
                                            &mut self.rng);
            }
        }
    }

    fn step(&mut self){
        self.spawn_stars();
        self.spawn_main();

        if IsKeyPressed(KEY_P) {
            self.paused = ! self.paused;
        }

        if ! self.paused {
            self.do_player_input();
            self.do_sine_movement();
            self.do_sine_movement_x();
            self.do_install();
            self.do_stop_at();
            self.do_movement();
            self.do_despawn();
            self.do_collision();
            self.do_timeout_death();
            self.do_death_check();
            self.do_shield_regen();
            self.do_weapon_cooldown();
            self.do_weapon_fire();

            self.world.maintain();
            self.frame_count+=1;
        }
    }


    fn do_weapon_cooldown(&mut self) {
        let mask = self.world.weapon_list.mask.clone();
        for id in mask {
            let mut weapon = self.world.weapon_list.get(id as IDType).unwrap();

            if weapon.gun_cooldown_frames > 0 {
                weapon.gun_cooldown_frames -= 1;
            }
            self.world.weapon_list.add(id as IDType, weapon);
        }
    }

    fn do_weapon_fire(&mut self) {
        let mask = self.world.weapon_list.mask.clone() &
                   self.world.auto_fire_list.mask.clone() &
                   self.world.physical_list.mask.clone();

        for id in mask {
            let fire = {
                let ref weapon = self.world.weapon_list.get(id as IDType).unwrap();
                weapon.gun_cooldown_frames <= 0
            };

            let pattern = {
                let ref weapon = self.world.weapon_list.get(id as IDType).unwrap();
                weapon.pattern
            };



            let prefab = {
                let ref weapon = self.world.weapon_list.get(id as IDType).unwrap();
                weapon.prefab.clone()
            };


            if fire {
                let mut weapon = self.world.weapon_list.get(id as IDType).unwrap().clone();
                let phy1 = self.world.physical_list.get(id as IDType).unwrap().clone();
                if let Some(sound) = weapon.fire_sound.clone() {
                    PlaySound(&*sound);
                }
                weapon.gun_cooldown_frames = weapon.fire_rate as i32;

                for angle in get_shot_angles(weapon.fire_angle, pattern) {
                    let bul_id = prefab.spawn(&mut self.world);

                    let mut bul_phy = self.world.physical_list.get(bul_id).unwrap().clone();

                    bul_phy.xvel = (DEG2RAD as f32 * angle).cos() *  weapon.fire_velocity;
                    bul_phy.yvel = (DEG2RAD as f32 * angle).sin() *  weapon.fire_velocity;
                    bul_phy.x = phy1.x + weapon.offset * weapon.direction;
                    bul_phy.y = phy1.y;

                    self.world.physical_list.add(bul_id, bul_phy);

                }
                self.world.weapon_list.add(id as IDType, weapon);
            }
        }
    }

    fn draw_by_id( &self, id: IDType) {
            let drw = self.world.drawable_list.get(id).unwrap();
            let pos = self.world.physical_list.get(id).unwrap();

            let txt = drw.texture;
            let src_rect = Rectangle {x:0, y:0, width:txt.width, height:txt.height};
            let dst_rect = Rectangle {
                x: (pos.x - (txt.width / 2) as f32) as i32,
                // x: pos.x as i32,
                y: (pos.y - (txt.height / 2) as f32) as i32,
                width: (txt.width) as i32,
                height: (txt.height) as i32
            };
            DrawTexturePro( &*txt,
                            src_rect,
                            dst_rect,
                            Vector2::new(0.0, 0.0),
                            0.0,
                            drw.tint.clone());
    }

    fn draw(&mut self){
        let mask = self.world.drawable_list.mask.clone() & self.world.physical_list.mask.clone();
        for id in mask {
            self.draw_by_id(id as IDType);
        }
        //draw UI
        //
        //draw shields
        DrawRectangle(1250, 80, 40, (self.get_player_shield_fraction()*100.0) as i32, Color{r:50, g:50, b:255, a:255});
        DrawRectangleLines(1250, 80, 40, 100, Color{r:50, g:50, b:255, a:255});

        let cargo = self.get_player_cargo();
        for i in 0..std::cmp::min(4, cargo.len()){

            let mut prefab = cargo[i].clone();
            prefab.physical = Some(PhysicalBuilder::new()
                .x(1250.0)
                .y(i as f32 *50.0+200.0)
                .build());

            let id = prefab.spawn(&mut self.world);
            self.world.destroy_later(id);
            self.draw_by_id(id);
        }
        self.world.maintain();

        //draw install bar
        DrawRectangle(1250, 400, 40, (self.get_player_install_fraction()*100.0) as i32, Color{r:200, g:200, b:200, a:255});
        DrawRectangleLines(1250, 400, 40, 100, Color{r:200, g:200, b:200, a:255});

        //draw score
        {
            let white = Color{r: 255, g:255, b:255, a:255};
            let text = format!("Score: {}", self.score);
            DrawText(text.as_str(), 100,20,20, white);
        }

        //draw boss health
        for id in self.world.boss_health_draw_list.mask.clone() & self.world.shield_list.mask.clone(){
            let boss_shield = self.world.shield_list.get(id as IDType).unwrap();
            let boss_shield_fraction = boss_shield.ammount / boss_shield.max_shield;
            DrawRectangle(300,
                          30,
                          (boss_shield_fraction * 400.0) as i32,
                          30,
                          Color{r:255, g:0, b:0, a: 255});
            DrawRectangleLines(300,
                               30,
                               400,
                               30, Color{r:255, g:0, b:0, a: 255});
        }
    }

    fn get_player_cargo(&mut self) -> Vec<Prefab> {
        let mask = self.world.controllable_list.mask.clone();

        for id in mask {
            let ship_stats = self.world.player_stats_list.get(id as IDType).unwrap();
            return ship_stats.owned;
        }
        return vec![];
    }

    fn get_player_install_fraction(&mut self) -> f32{
        let mask = self.world.controllable_list.mask.clone();

        for id in mask {
            let stats = self.world.player_stats_list.get(id as IDType).unwrap();
            return stats.install_progress as f32 / PART_INSTALLED_AT as f32;
        }
        return 0.0;
    }

    fn get_shield_fraction_by_id(&self, id: IDType) -> f32 {
        if let Some(shield) = self.world.shield_list.get(id) {
            return shield.ammount / shield.max_shield;
        }
        return 1.0;
    }

    fn get_player_shield_fraction(&self) -> f32{
        let mask = self.world.controllable_list.mask.clone();

        for id in mask {
            return self.get_shield_fraction_by_id(id as IDType);
        }
        return 1.0;
    }

    fn is_colliding(&mut self, a: IDType, b: IDType) -> bool {
        if self.world.physical_list.get(a).is_none() || self.world.physical_list.get(b).is_none() {
            return false;
        }
        let a_phy = self.world.physical_list.get(a).unwrap();
        let b_phy = self.world.physical_list.get(b).unwrap();

        let a_col = self.world.collidable_list.get(a).unwrap();
        let b_col = self.world.collidable_list.get(b).unwrap();



        let a_pos = Vector2::new( a_phy.x , a_phy.y );
        let b_pos = Vector2::new( b_phy.x , b_phy.y );

        //DrawCircleV( &Vector2f::new(a_pos.x + 2.0*a_col.radius , a_pos.y), a_col.radius, Color{r:255, g:255, b:255, a:255});
        return CheckCollisionCircles(&a_pos, a_col.radius,
                                     &b_pos, b_col.radius);
    }

    fn handle_collision(&mut self, a: IDType, b: IDType) {
        if self.world.controllable_list.contains(a) && self.world.powerup_list.contains(b) {
            let mut stats = self.world.player_stats_list.get(a).unwrap();
            let power = self.world.powerup_list.get(b).unwrap();

            stats.movement_speed = stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32 + 1.0);

            let mut prefab = self.id_to_prefab(b);
            prefab.physical = None;
            stats.owned.push(prefab);

            PlaySound(&*power.pickup_sound);
            self.world.destroy_later(b);

            self.world.player_stats_list.add(a, stats);
        }

        if self.world.shield_list.contains(a) && self.world.bullet_list.contains(b) {
            if  self.world.team_list.get(a).unwrap_or(Team{team:-1}).team != 
                self.world.team_list.get(b).unwrap_or(Team{team:-2}).team {

                let mut shield = self.world.shield_list.get(a).unwrap();
                let bullet = self.world.bullet_list.get(b).unwrap();

                shield.ammount -= bullet.damage;
                self.world.shield_list.add(a, shield);

                self.world.destroy_later(b);
            }
        }
    }

    fn do_install(&mut self){
        let mask = self.world.player_stats_list.mask.clone() & self.world.install_list.mask.clone();

        for id in mask {
            let mut stats = self.world.player_stats_list.get(id as IDType).unwrap();

            if stats.owned.len() == 0 { 
                stats.install_progress = 0; 
                self.world.player_stats_list.add(id as IDType, stats);
                continue;
            }


            stats.install_progress += 1;

            if stats.install_progress >= PART_INSTALLED_AT {
                stats.install_progress = 0;
                PlaySound(&*stats.install_finish_sound);
                let upgrade_prefab = stats.owned[0].clone();
                stats.install_progress == 0;

                //apply the upgrade
                let mut weapon = self.world.weapon_list.get(id as IDType).unwrap();
                let mut shield = self.world.shield_list.get(id as IDType).unwrap();
                let upgrade = upgrade_prefab.powerup.unwrap();

                weapon.fire_rate *= upgrade.fire_rate_increase;
                shield.regen *= upgrade.regen_increase;
                shield.max_shield *= upgrade.shield_increase;
                weapon.pattern += upgrade.shot_increase;

                {
                    let bullet: &mut Bullet = &mut weapon.prefab.bullet.clone().unwrap();
                    bullet.damage *= upgrade.fire_damage_increase;

                    let mut wc = weapon.clone();
                    let bullet_prefab = std::sync::Arc::make_mut(&mut wc.prefab);
                    bullet_prefab.bullet = Some(bullet.clone());
                    weapon.prefab = Arc::new(bullet_prefab.clone());
                }

                self.world.shield_list.add(id as IDType, shield);
                self.world.weapon_list.add(id as IDType, weapon);
                stats.owned.remove(0);
            }

            stats.movement_speed = stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32 + 1.0);
            self.world.player_stats_list.add(id as IDType, stats);
        }
    }


    fn do_death_check(&mut self){
        let mask = self.world.shield_list.mask.clone();

        for id in mask {
            let shield = self.world.shield_list.get(id as IDType).unwrap();
            let pos = self.world.physical_list.get(id as IDType).unwrap();
            if shield.ammount < 0.0 {
                if let Some(death_event) = self.world.death_event_list.get(id as IDType) {

                    let death_val = death_event.die(&mut self.world, &pos);
                    self.score += death_val.1;
                    if death_val.0 {
                        self.spawn_plan.clear();
                    }
                }
                self.world.destroy_later(id as IDType);
            }
        }

        self.world.maintain();
    }

    fn do_timeout_death(&mut self){
        let mask = self.world.timeout_death_list.mask.clone();

        for id in mask {
            let mut timeout = self.world.timeout_death_list.get(id as IDType).unwrap();
            if timeout.ticks == 0 {
                let pos = self.world.physical_list.get(id as IDType).unwrap();

                if let Some(death_event) = self.world.death_event_list.get(id as IDType) {

                    let death_val = death_event.die(&mut self.world, &pos);
                    self.score += death_val.1;

                    if death_val.0 {
                        self.spawn_plan.clear();
                    }
                }

                self.world.destroy_later(id as IDType);
            } else {
                timeout.ticks -= 1;
                self.world.timeout_death_list.add(id as IDType, timeout);
            }
        }

        self.world.maintain();

    }


    fn do_shield_regen(&mut self){
        let mask = self.world.shield_list.mask.clone();

        for id in mask {
            let mut shield = self.world.shield_list.get(id as IDType).unwrap();
            shield.ammount +=  shield.regen * FRAME_TIME;
            shield.ammount = min_float(shield.max_shield, shield.ammount);

            self.world.shield_list.add(id as IDType, shield);
        }
    }

    //do the dumb n squared algorithm since I expect n to be small
    fn do_collision(&mut self){
        let mask = self.world.physical_list.mask.clone() & self.world.collidable_list.mask.clone();

        let mut to_check =  Vec::<IDType>::new();
        for id in mask {
            to_check.push(id as IDType);
        }

        /* //debug draw
        for id in to_check.clone() {
            let phy = self.world.physical_list.get(id).clone().unwrap();
            let col = self.world.collidable_list.get(id).clone().unwrap();

            DrawCircle(phy.x as i32, phy.y as i32, col.radius, Color{r:255, g:255, b:255, a:255});
        }
        // */

        let mut groups : Vec<Vec<IDType>>= vec![ vec![]];
        {
            let steps_x = 10;
            let steps_y = 5;
            let delta_x = GetScreenWidth() / steps_x;
            let delta_y = GetScreenHeight() / steps_y;
            for i in -2..steps_x+2 {
                let x = delta_x * i;
                for j in -2..steps_y+2 {
                    let mut group: Vec<IDType> = Vec::<IDType>::new();
                    let y = delta_y * j;
                    let rect = Rectangle{x: x, y: y, width: delta_x, height: delta_y };

                    for id in to_check.iter() {
                        if self.is_in_rect(&rect, *id) {
                            group.push(*id);
                        }
                    }
                    groups.push(group);
                }
            }
        }

        let mut already_checked = HashSet::<(IDType, IDType)>::new();
        for group in groups {
            if group.len() <= 1 { continue; }
            for outer in 0..group.len() -1 {
                for inner in outer..group.len() {
                    if self.is_colliding(group[outer], group[inner]) {
                        if already_checked.contains( &(group[inner], group[outer]) ) { continue; }
                        if already_checked.contains( &(group[outer], group[inner]) ) { continue; }

                        self.handle_collision(group[outer], group[inner]);
                        self.handle_collision(group[inner], group[outer]);
                        already_checked.insert((group[outer], group[inner]));
                    }
                }
            }
        }

        self.world.maintain();
    }

    fn is_in_rect(&self, rect: &Rectangle, id: IDType) -> bool {
        let pos = self.world.physical_list.get(id).unwrap();
        let col = self.world.collidable_list.get(id).unwrap();

        let vec = Vector2f::new(pos.x, pos.y);
        CheckCollisionCircleRec(vec, col.radius, *rect)
    }

    fn do_despawn(&mut self){
        let mask = (self.world.despawn_far_left.mask.clone() | self.world.despawn_far_right.mask.clone()) & self.world.physical_list.mask.clone();

        for id in mask {
            let phy = self.world.physical_list.get(id as IDType).unwrap();
            if self.world.despawn_far_left.contains(id as IDType) && phy.x < -80.0 {
                self.world.destroy_later(id as IDType);
            } 
            if self.world.despawn_far_right.contains(id as IDType) && phy.x > GetScreenWidth() as f32 + 120.0 {
                self.world.destroy_later(id as IDType);
            }
        }
        self.world.maintain();
    }

    fn do_player_input(&mut self){
        let mask = self.world.controllable_list.mask.clone() &
                   self.world.physical_list.mask.clone() &
                   self.world.player_stats_list.mask.clone();
        for id in mask {
            let player_speed = self.world.player_stats_list
                                .get(id as IDType)
                                .unwrap().movement_speed * FRAME_TIME;

            if IsKeyDown(KEY_W) {
                let mut phy = self.world.physical_list.get(id as IDType).unwrap();
                phy.y -= player_speed;
                phy.y = max_float(phy.y, 40.0);
                self.world.physical_list.add(id as IDType, phy);
            }
            if IsKeyDown(KEY_S) {
                let mut phy = self.world.physical_list.get(id as IDType).unwrap();
                phy.y += player_speed;
                phy.y = min_float(phy.y, GetScreenHeight() as f32 - 40.0);
                self.world.physical_list.add(id as IDType, phy);
            }

            if IsKeyDown(KEY_D) {
                let mut phy = self.world.physical_list.get(id as IDType).unwrap();
                phy.x += player_speed;
                phy.x = min_float(phy.x, GetScreenWidth() as f32 - 140.0);
                self.world.physical_list.add(id as IDType, phy);
            }

            if IsKeyDown(KEY_A) {
                let mut phy = self.world.physical_list.get(id as IDType).unwrap();
                phy.x -= player_speed;
                phy.x = max_float(phy.x, 50.0);
                self.world.physical_list.add(id as IDType, phy);
            }

            if IsKeyPressed(KEY_SPACE) {
                self.world.auto_fire_list.add(id as IDType, AutoFire{});
                self.world.install_list.remove(id as IDType);
            }

            if IsKeyReleased(KEY_SPACE) {
                self.world.auto_fire_list.remove(id as IDType);
                self.world.install_list.add(id as IDType, Install{});
            }
        }
    }

    fn do_sine_movement(&mut self){
        let mask = self.world.sine_movement_list.mask.clone();
        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            let mut sine = self.world.sine_movement_list.get(id as IDType).unwrap();

            let freq = sine.frequency;
            let amplitude = sine.amplitude;
            let step =  sine.step;

            let last = amplitude * f32::sin(3.14 * freq * (step-1) as f32 /FRAME_RATE);
            let curr = amplitude * f32::sin(3.14 * freq * (step) as f32 /FRAME_RATE);
            let diff = curr - last;

            sine.step += 1;

            phy.y += diff;

            self.world.physical_list.add(id as IDType, phy);
            self.world.sine_movement_list.add(id as IDType, sine);
        }
    }

    fn do_sine_movement_x(&mut self){
        let mask = self.world.sine_movement_x_list.mask.clone();
        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            let mut sine = self.world.sine_movement_x_list.get(id as IDType).unwrap();

            let freq = sine.frequency;
            let amplitude = sine.amplitude;
            let step =  sine.step;

            let last = amplitude * f32::sin(3.14 * freq * (step-1) as f32 /FRAME_RATE);
            let curr = amplitude * f32::sin(3.14 * freq * (step) as f32 /FRAME_RATE);
            let diff = curr - last;

            sine.step += 1;

            phy.x += diff;

            self.world.physical_list.add(id as IDType, phy);
            self.world.sine_movement_x_list.add(id as IDType, sine);
        }
    }


    fn do_stop_at(&mut self){
        let mask = self.world.stop_at_list.mask.clone();

        for id in mask {

            let stop_at = self.world.stop_at_list.get(id as IDType).unwrap();
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();

            if stop_at.xloc >= phy.x {
                phy.xvel = 0.0;
                self.world.physical_list.add(id as IDType, phy);
                self.world.stop_at_list.remove(id as IDType);
            }
        }
    }

    fn do_movement(&mut self){
        let mask = self.world.physical_list.mask.clone();
        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            phy.x += phy.xvel * FRAME_TIME;
            phy.y += phy.yvel * FRAME_TIME;
            self.world.physical_list.add(id as IDType, phy);
        }
    }

    fn id_to_prefab(&mut self, id: IDType) -> Prefab {
        self.world.id_to_prefab(id)
    }

}
fn get_shot_angles(angle: f32, count: i32) -> Vec<f32> {
    let mut ret = vec![];
    let delta = angle / (count+1) as f32;

    for i in 0..count {
        ret.push( delta * (i as f32 + 1.0) - angle/2.0)
    }

    return ret;
}

fn main(){
    pretty_env_logger::init().unwrap();
    {
        //raylib configuration flags
        #[allow(unused_mut)]
        let mut flags: u32 = 0;
        //flags |= FLAG_SHOW_LOGO;
        //flags |= FLAG_FULLSCREEN_MODE;
        flags |= FLAG_WINDOW_RESIZABLE;
        //flags |= FLAG_WINDOW_DECORATED;
        //flags |= FLAG_WINDOW_TRANSPARENT;
        flags |= FLAG_MSAA_4X_HINT;
        //flags |= FLAG_VSYNC_HINT;
        SetConfigFlags(flags);
    }

    InitWindow(1300, 750, "Dodgem");
    InitAudioDevice();
    SetTargetFPS(FRAME_RATE as i32);

    let mut gl = GameData::new();

    gl.spawn_plan = gen_level(10.0, 100.0, 0, &mut gl.rng);


    while ! WindowShouldClose() {
        gl.step();

        ClearBackground(Color{r:0, g:0, b:0, a:255} );
        BeginDrawing();
        DrawFPS(3,3);
        gl.draw();
        EndDrawing();
    }
}
