#[macro_use] extern crate cached;
// `cached!` macro requires the `lazy_static!` macro
#[macro_use] extern crate lazy_static;

// extern crate hibitset;
extern crate quickersort;
extern crate nalgebra;
extern crate rand;

use std::f32;

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
type id_type=i64;

#[allow(unused)]
use std::sync::{Arc, RwLock, Mutex, MutexGuard, TryLockResult, LockResult};
#[allow(unused)]

use sprite_loader::*;
mod sprite_loader;

use storage::*;
mod storage;


use std::collections::{HashMap,HashSet};


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

#[allow(unused)]
fn b_and(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
    let small = std::cmp::min(a.len(), b.len());
    let large = std::cmp::max(a.len(), b.len());

    let mut ret = Vec::<bool>::new();

    for i in 0..small {
        ret.push(a[i] && b[i]);
    }

    for _ in small..large {
        ret.push(false);
    }
    return ret;
}

fn b_or(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
    let small = std::cmp::min(a.len(), b.len());
    let large = std::cmp::max(a.len(), b.len());


    let mut ret = Vec::<bool>::new();

    for i in 0..small {
        ret.push(a[i] || b[i]);
    }

    for i in small..large {
        ret.push(
            (a.len() > i && a[i]) ||
            (b.len() > i && b[i]));
    }
    return ret;
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
}

struct DrawableBuilder{
    thing: Drawable,
}

impl DrawableBuilder {
    fn new() -> Self {
        DrawableBuilder{ thing: Drawable{
            texture: load_texture("no-texture.png".to_string()).unwrap(),
            layer: 0.1,
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

    fn build(self) -> Drawable {
        return self.thing;
    }
}

#[derive(Clone)]
pub struct DeathEvent{
    sound: Arc<Sound>,
    spawner: Arc<Spawner>
}

struct DeathEventBuilder{
    thing: DeathEvent,
}

impl DeathEventBuilder {
    fn new() -> Self {
        DeathEventBuilder{ thing: DeathEvent {
            sound: load_sound("scilence.wav".to_string()).unwrap(),
            spawner: Arc::new(Spawner::new()),
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



struct GameData{
    drawable_list: VectorStorage<Drawable>,
    physical_list: VectorStorage<Physical>,
    collidable_list: VectorStorage<Collidable>,
    controllable_list: VectorStorage<PlayerControl>,
    bullet_list: VectorStorage<Bullet>,
    shield_list: VectorStorage<Shield>,
    despawn_left: VectorStorage<DespawnFarLeft>,
    despawn_right: VectorStorage<DespawnFarRight>,
    powerup_list: VectorStorage<Powerup>,
    player_stats_list: VectorStorage<PlayerStats>,
    weapon_list: VectorStorage<Weapon>,
    auto_fire_list: VectorStorage<AutoFire>,
    sine_movement_list: VectorStorage<SineMovement>,
    team_list: VectorStorage<Team>,
    install_list: VectorStorage<Install>,
    death_event_list: VectorStorage<DeathEvent>,
    stop_at_list: VectorStorage<StopAt>,

    unused_ids: Vec<id_type>,
    max_id: i64,

    frame_count: u64,
    to_destroy: Vec<id_type>,

    spawn_plan: Option<HashMap<u64, Vec<Spawner>>>,
}

impl GameData {
    fn new() -> Self {
        Self{

            drawable_list: VectorStorage::<Drawable>::new(),
            physical_list: VectorStorage::<Physical>::new(),
            collidable_list: VectorStorage::<Collidable>::new(),
            controllable_list: VectorStorage::<PlayerControl>::new(),
            bullet_list: VectorStorage::<Bullet>::new(),
            shield_list: VectorStorage::<Shield>::new(),
            despawn_left: VectorStorage::<DespawnFarLeft>::new(),
            despawn_right: VectorStorage::<DespawnFarRight>::new(),
            powerup_list: VectorStorage::<Powerup>::new(),
            player_stats_list: VectorStorage::<PlayerStats>::new(),
            weapon_list: VectorStorage::<Weapon>::new(),
            auto_fire_list: VectorStorage::<AutoFire>::new(),
            sine_movement_list: VectorStorage::<SineMovement>::new(),
            team_list: VectorStorage::<Team>::new(),
            install_list: VectorStorage::<Install>::new(),
            death_event_list: VectorStorage::<DeathEvent>::new(),
            stop_at_list: VectorStorage::<StopAt>::new(),

            unused_ids: Vec::<id_type>::new(),
            max_id: 0,

            frame_count: 0,
            to_destroy: Vec::<id_type>::new(),
            spawn_plan: None,
        }
    }
    fn destroy(&mut self, id: id_type){
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
        self.team_list.remove(id);
        self.install_list.remove(id);
        self.death_event_list.remove(id);
        self.stop_at_list.remove(id);

        self.free_id(id);
    }


    fn alloc_id(&mut self) -> id_type {
        let out: id_type;
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

    fn free_id(&mut self, id: id_type) {
        self.unused_ids.push(id);
    }

    fn step(&mut self){
        {
            let frame = &self.frame_count.clone();
            let olst;
            if let Some(ref mut plan) = self.spawn_plan {
                olst = plan.remove(&frame).clone();
            } else {
                olst = None;
            }

            if let Some(lst) = olst.clone() {
                print!("Spawning on frame {}\n", frame);
                for spawner in lst {
                    #[allow(mutable_transmutes)]
                    let mut gd=unsafe{&mut std::mem::transmute::<&GameData, &mut GameData>(&self) };
                    spawner.spawn(&mut gd);
                }
            }
        }


        self.frame_count+=1;

        self.do_player_input();
        self.do_sine_movement();
        self.do_install();
        self.do_stop_at();
        self.do_movement();
        self.do_despawn();
        self.do_collision();
        self.do_death_check();
        self.do_shield_regen();
        self.do_weapon_cooldown();
        self.do_weapon_fire();

        self.do_destroy();
    }


    fn do_weapon_cooldown(&mut self) {
        let mask = self.weapon_list.mask.clone();
        for id in 0..mask.len() as id_type {
            if !mask[id as usize] { continue; }

            let mut weapon = self.weapon_list.get(id).unwrap().clone();

            if weapon.gun_cooldown_frames > 0 {
                weapon.gun_cooldown_frames -= 1;
            }
            self.weapon_list.add(id, weapon);
        }
    }

    fn do_weapon_fire(&mut self) {
        let mask = b_and( b_and( self.weapon_list.mask.clone(), self.auto_fire_list.mask.clone() ) , self.physical_list.mask.clone());

        let high_id = self.max_id;
        for id in 0..high_id {
            if high_id < id || !mask[id as usize] { continue; }

            let fire = {
                let ref weapon = self.weapon_list.get(id).unwrap();
                weapon.gun_cooldown_frames <= 0
            };

            let pattern = {
                let ref weapon = self.weapon_list.get(id).unwrap();
                weapon.pattern
            };



            let prefab = {
                let ref weapon = self.weapon_list.get(id).unwrap();
                weapon.prefab.clone()
            };


            if fire {
                let mut weapon = self.weapon_list.get(id).unwrap().clone();
                let phy1 = self.physical_list.get(id).unwrap().clone();
                if let Some(sound) = weapon.fire_sound.clone() {
                    PlaySound(&*sound);
                }
                weapon.gun_cooldown_frames = weapon.fire_rate as i32;

                for angle in get_shot_angles(weapon.fire_angle, pattern) {
                    #[allow(mutable_transmutes)]
                    let mut gd =unsafe{&mut std::mem::transmute::<&GameData,&mut GameData>(&self)};
                    let bul_id = prefab.spawn(&mut gd);

                    let mut bul_phy = self.physical_list.get(bul_id).unwrap().clone();

                    bul_phy.xvel = (DEG2RAD as f32 * angle).cos() *  weapon.fire_velocity;
                    bul_phy.yvel = (DEG2RAD as f32 * angle).sin() *  weapon.fire_velocity;
                    bul_phy.x = phy1.x + weapon.offset * weapon.direction;
                    bul_phy.y = phy1.y;

                    self.physical_list.add(bul_id, bul_phy);

                }
                self.weapon_list.add(id, weapon);
            }
        }
    }

    fn draw_by_id( &self, id: id_type) {
            let drw = self.drawable_list.get(id).unwrap().clone();
            let pos = self.physical_list.get(id).unwrap().clone();

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
                            Color{r:255, g:255, b:255, a:255});
    }

    fn draw(&mut self){
        let mask = b_and(self.drawable_list.mask.clone() , self.physical_list.mask.clone());
        for id in 0..self.max_id {
            if  ! mask[id as usize] { continue; }

            self.draw_by_id(id);
        }
        //draw UI
        //
        //draw shields
        DrawRectangle(1250, 80, 40, (self.get_player_shield_fraction()*100.0) as i32, Color{r:50, g:50, b:255, a:255});
        DrawRectangleLines(1250, 80, 40, 100, Color{r:50, g:50, b:255, a:255});

        let cargo = self.get_player_cargo();
        for i in 0..std::cmp::min(4, cargo.len()){
            #[allow(mutable_transmutes)]
            let gd=unsafe{&mut std::mem::transmute::<&GameData, &mut GameData>(&self) };

            let mut prefab = cargo[i].clone();
            prefab.physical = Some(PhysicalBuilder::new()
                .x(1250.0)
                .y(i as f32 *50.0+200.0)
                .build());

            let id = prefab.spawn(gd);
            self.to_destroy.push(id);
            self.draw_by_id(id);
        }
        self.do_destroy();

        //draw install bar
        DrawRectangle(1250, 400, 40, (self.get_player_install_fraction()*100.0) as i32, Color{r:200, g:200, b:200, a:255});
        DrawRectangleLines(1250, 400, 40, 100, Color{r:200, g:200, b:200, a:255});


    }

    fn get_player_cargo(&mut self) -> Vec<Prefab> {
        let mask = self.controllable_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            let ship_stats = self.player_stats_list.get(id).unwrap().clone();
            return ship_stats.owned;
        }
        return vec![];
    }

    fn get_player_install_fraction(&mut self) -> f32{
        let mask = self.controllable_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            let stats = self.player_stats_list.get(id).unwrap().clone();
            return stats.install_progress as f32 / PART_INSTALLED_AT as f32;
        }
        return 0.0;
    }



    fn get_player_shield_fraction(&mut self) -> f32{
        let mask = self.controllable_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            if let Some(shield) = self.shield_list.get(id) {
                return shield.ammount / shield.max_shield;
            }
        }
        return 1.0;
    }

    fn is_colliding(&mut self, a: id_type, b: id_type) -> bool {
        if self.physical_list.get(a).is_none() || self.physical_list.get(b).is_none() {
            return false;
        }
        let a_phy = self.physical_list.get(a).unwrap();
        let b_phy = self.physical_list.get(b).unwrap();

        let a_col = self.collidable_list.get(a).unwrap();
        let b_col = self.collidable_list.get(b).unwrap();



        let a_pos = Vector2::new( a_phy.x , a_phy.y );
        let b_pos = Vector2::new( b_phy.x , b_phy.y );

        //DrawCircleV( &Vector2f::new(a_pos.x + 2.0*a_col.radius , a_pos.y), a_col.radius, Color{r:255, g:255, b:255, a:255});
        return CheckCollisionCircles(&a_pos, a_col.radius,
                                     &b_pos, b_col.radius);
    }

    fn handle_collision(&mut self, a: id_type, b: id_type) {
        if self.controllable_list.contains(a) && self.powerup_list.contains(b) {
            let mut stats = self.player_stats_list.get(a).unwrap().clone();
            let power = self.powerup_list.get(b).unwrap().clone();

            stats.movement_speed = stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32 + 1.0);

            let mut prefab = self.id_to_prefab(b);
            prefab.physical = None;
            stats.owned.push(prefab);

            PlaySound(&*power.pickup_sound);
            self.to_destroy.push(b);

            self.player_stats_list.add(a, stats);
        }

        if self.shield_list.contains(a) && self.bullet_list.contains(b) {
            let a_phy = self.physical_list.get(a).unwrap();
            let b_phy = self.physical_list.get(a).unwrap();

            if  self.team_list.get(a).unwrap_or(Team{team:-1}).clone().team != 
                self.team_list.get(b).unwrap_or(Team{team:-2}).clone().team {

                let mut shield = self.shield_list.get(a).unwrap();
                let bullet = self.bullet_list.get(b).unwrap().clone();

                shield.ammount -= bullet.damage;
                self.shield_list.add(a, shield);

                self.to_destroy.push(b);
            }
        }
    }

    fn do_install(&mut self){
        let mask = b_and(self.player_stats_list.mask.clone(), self.install_list.mask.clone());

        for id in 0..mask.len() as id_type {
            if ! mask[id as usize ] { continue; }

            let mut stats = self.player_stats_list.get(id).unwrap();

            if stats.owned.len() == 0 { 
                stats.install_progress = 0; 
                self.player_stats_list.add(id, stats);
                continue;
            }


            stats.install_progress += 1;

            if stats.install_progress >= PART_INSTALLED_AT {
                stats.install_progress = 0;
                PlaySound(&*stats.install_finish_sound);
                let upgrade_prefab = stats.owned[0].clone();
                stats.install_progress == 0;

                //apply the upgrade
                let mut weapon = self.weapon_list.get(id).clone().unwrap();
                let mut shield = self.shield_list.get(id).clone().unwrap();
                let upgrade = upgrade_prefab.powerup.unwrap().clone();

                weapon.fire_rate *= upgrade.fire_rate_increase;
                shield.regen *= upgrade.regen_increase;
                shield.max_shield *= upgrade.shield_increase;
                weapon.pattern += upgrade.shot_increase;

                {
                    let bullet: &mut Bullet = &mut weapon.clone().prefab.bullet.clone().unwrap();
                    bullet.damage *= upgrade.fire_damage_increase;

                    let mut wc = weapon.clone();
                    let mut bullet_prefab = std::sync::Arc::make_mut(&mut wc.prefab);
                    bullet_prefab.bullet = Some(bullet.clone());
                    weapon.prefab = Arc::new(bullet_prefab.clone());
                }

                self.shield_list.add(id, shield);
                self.weapon_list.add(id, weapon);
                stats.owned.remove(0);
            }

            stats.movement_speed = stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32 + 1.0);
            self.player_stats_list.add(id, stats);
        }
    }


    fn do_death_check(&mut self){
        let mask = self.shield_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            let shield = self.shield_list.get(id).unwrap().clone();
            let pos = self.physical_list.get(id).unwrap().clone();
            if shield.ammount < 0.0 {
                if let Some(death_event) = self.death_event_list.get(id) {
                    PlaySound(&death_event.sound);

                    #[allow(mutable_transmutes)]
                    let mut gd=unsafe{&mut std::mem::transmute::<&GameData, &mut GameData>(&self) };
                    death_event.spawner.spawn_at_pos(&mut gd, &pos);
                }
                self.to_destroy.push(id);
            }
        }

        self.do_destroy();
    }

    fn do_destroy(&mut self){
        self.to_destroy.sort();
        self.to_destroy.dedup_by(|a, b| { a == b } );
        for id in self.to_destroy.clone() {
            //print!("destroying id {} on frame {}\n", id, self.frame_count);
            self.destroy(id);
        }
        self.to_destroy.clear();
    }

    fn do_shield_regen(&mut self){
        let mask = self.shield_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            let mut shield = self.shield_list.get(id).unwrap().clone();
            shield.ammount +=  shield.regen * FRAME_TIME;
            shield.ammount = min_float(shield.max_shield, shield.ammount);

            self.shield_list.add(id, shield);
        }
    }

    //do the dumb n squared algorithm since I expect n to be small
    fn do_collision(&mut self){
        let mask = b_and(self.physical_list.mask.clone() , self.collidable_list.mask.clone());

        let mut to_check =  Vec::<id_type>::new();
        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            to_check.push(id);
        }

        /* //debug draw
        for id in to_check.clone() {
            let phy = self.physical_list.get(id).clone().unwrap();
            let col = self.collidable_list.get(id).clone().unwrap();

            DrawCircle(phy.x as i32, phy.y as i32, col.radius, Color{r:255, g:255, b:255, a:255});
        }
        // */

        let mut groups : Vec<Vec<id_type>>= vec![ vec![]];
        {
            let steps_x = 10;
            let steps_y = 5;
            let delta_x = GetScreenWidth() / steps_x;
            let delta_y = GetScreenHeight() / steps_y;
            for i in -2..steps_x+2 {
                let x = delta_x * i;
                for j in -2..steps_y+2 {
                    let mut group: Vec<id_type> = Vec::<id_type>::new();
                    let y = delta_y * j;
                    let rect = Rectangle{x: x, y: y, width: delta_x, height: delta_y };

                    for id in to_check.clone() {
                        if self.is_in_rect(&rect, id) {
                            group.push(id);
                        }
                    }
                    groups.push(group);
                }
            }
        }

        let mut already_checked = HashSet::<(id_type, id_type)>::new();
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

        self.do_destroy();
    }

    fn is_in_rect(&self, rect: &Rectangle, id: id_type) -> bool {
        let pos = self.physical_list.get(id).unwrap();
        let col = self.collidable_list.get(id).unwrap();

        let vec = Vector2f::new(pos.x, pos.y);
        CheckCollisionCircleRec(vec, col.radius, *rect)
    }

    fn do_despawn(&mut self){
        let mask = b_and( b_or(self.despawn_left.mask.clone() , self.despawn_right.mask.clone()) , self.physical_list.mask.clone());

        for id in 0..mask.len() as id_type {
            if  !mask[id as usize] { continue; }

            let phy = self.physical_list.get(id).unwrap().clone();
            if self.despawn_left.contains(id) && phy.x < -80.0 {
                self.to_destroy.push(id);
            } 
            if self.despawn_right.contains(id) && phy.x > GetScreenWidth() as f32 + 120.0 {
                self.to_destroy.push(id);
            }
        }
        self.do_destroy();
    }

    fn do_player_input(&mut self){
        let in_mask = b_and( b_and( self.controllable_list.mask.clone() , self.physical_list.mask.clone() ),  self.player_stats_list.mask.clone());

        for id in 0..self.max_id {
            if !in_mask[id as usize]{ continue; }


            let player_speed = self.player_stats_list.get(id).unwrap().movement_speed * FRAME_TIME;

            if IsKeyDown(KEY_W) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.y -= player_speed;
                phy.y = max_float(phy.y, 40.0);
                self.physical_list.add(id, phy);
            }
            if IsKeyDown(KEY_S) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.y += player_speed;
                phy.y = min_float(phy.y, GetScreenHeight() as f32 - 40.0);
                self.physical_list.add(id, phy);
            }

            if IsKeyDown(KEY_D) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.x += player_speed;
                phy.x = min_float(phy.x, GetScreenWidth() as f32 - 140.0);
                self.physical_list.add(id, phy);
            }

            if IsKeyDown(KEY_A) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.x -= player_speed;
                phy.x = max_float(phy.x, 50.0);
                self.physical_list.add(id, phy);
            }

            if IsKeyPressed(KEY_SPACE) {
                self.auto_fire_list.add(id, AutoFire{});
                self.install_list.remove(id);
            }

            if IsKeyReleased(KEY_SPACE) {
                self.auto_fire_list.remove(id);
                self.install_list.add(id, Install{});
            }
        }
    }

    fn do_sine_movement(&mut self){
        let mask = self.sine_movement_list.mask.clone();
        for id in 0..mask.len() as id_type {
            if  !mask[id as usize] { continue; }

            let mut phy = self.physical_list.get(id).unwrap().clone();
            let mut sine = self.sine_movement_list.get(id).unwrap().clone();

            let freq = sine.frequency;
            let amplitude = sine.amplitude;
            let step =  sine.step;

            let last = amplitude * f32::sin(3.14 * freq * (step-1) as f32 /FRAME_RATE);
            let curr = amplitude * f32::sin(3.14 * freq * (step) as f32 /FRAME_RATE);
            let diff = curr - last;

            sine.step += 1;

            phy.y += diff;

            self.physical_list.add(id, phy);
            self.sine_movement_list.add(id, sine);
        }
    }

    fn do_stop_at(&mut self){
        let mask = self.stop_at_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if ! mask[id as usize] { continue; }

            let stop_at = self.stop_at_list.get(id).unwrap();
            let mut phy = self.physical_list.get(id).unwrap();

            if stop_at.xloc >= phy.x {
                phy.xvel = 0.0;
                self.physical_list.add(id, phy);
                self.stop_at_list.remove(id);
            }
        }
    }

    fn do_movement(&mut self){
        let mask = self.physical_list.mask.clone();
        for id in 0..self.max_id as id_type {
            if ! mask[id as usize] { continue; }
            let mut phy = self.physical_list.get(id).unwrap().clone();
            phy.x += phy.xvel * FRAME_TIME;
            phy.y += phy.yvel * FRAME_TIME;
            self.physical_list.add(id, phy);
        }
    }

    fn id_to_prefab(&mut self, id: id_type) -> Prefab {
        let mut ret = PrefabBuilder::new();
        if let Some(val) = self.drawable_list.get(id) {
            ret = ret.drawable(val);
        }
        if let Some(val) = self.physical_list.get(id) {
            ret = ret.physical(val);
        }
        if let Some(val) = self.collidable_list.get(id) {
            ret = ret.collidable(val);
        }
        if let Some(val) = self.controllable_list.get(id) {
            ret = ret.controllable(val);
        }
        if let Some(val) = self.bullet_list.get(id) {
            ret = ret.bullet(val);
        }
        if let Some(val) = self.shield_list.get(id) {
            ret = ret.shield(val);
        }
        if let Some(val) = self.despawn_left.get(id) {
            ret = ret.despawn_left(val);
        }
        if let Some(val) = self.despawn_right.get(id) {
            ret = ret.despawn_right(val);
        }
        if let Some(val) = self.powerup_list.get(id) {
            ret = ret.powerup(val);
        }
        if let Some(val) = self.player_stats_list.get(id) {
            ret = ret.player_stats(val);
        }
        if let Some(val) = self.weapon_list.get(id) {
            ret = ret.weapon(val);
        }
        if let Some(val) = self.auto_fire_list.get(id) {
            ret = ret.auto_fire(val);
        }
        if let Some(val) = self.sine_movement_list.get(id) {
            ret = ret.sine_movement(val);
        }
        if let Some(val) = self.team_list.get(id) {
            ret = ret.team(val);
        }
        if let Some(val) = self.install_list.get(id) {
            ret = ret.install(val);
        }
        if let Some(val) = self.death_event_list.get(id) {
            ret = ret.death_event(val);
        }
        if let Some(val) = self.stop_at_list.get(id) {
            ret = ret.stop_at(val);
        }


        return ret.build();
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
    {
        //raylib configuration flags
        #[allow(unused_mut)]
        let mut flags: u32 = 0;
        //flags |= FLAG_SHOW_LOGO;
        //flags |= FLAG_FULLSCREEN_MODE;
        flags |= FLAG_WINDOW_RESIZABLE;
        //flags |= FLAG_WINDOW_DECORATED;
        //flags |= FLAG_WINDOW_TRANSPARENT;
        //flags |= FLAG_MSAA_4X_HINT;
        //flags |= FLAG_VSYNC_HINT;
        SetConfigFlags(flags);
    }

    InitWindow(1300, 750, "Dodgem");
    InitAudioDevice();
    SetTargetFPS(FRAME_RATE as i32);

    let mut gl = GameData::new();

    gl.spawn_plan = Some(gen_level(10.0, 100.0));


    while ! WindowShouldClose() {
        gl.step();

        ClearBackground(Color{r:0, g:0, b:0, a:255} );
        BeginDrawing();
        DrawFPS(3,3);
        gl.draw();
        EndDrawing();
    }
}
