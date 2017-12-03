#[macro_use] extern crate cached;
// `cached!` macro requires the `lazy_static!` macro
#[macro_use] extern crate lazy_static;

// extern crate hibitset;
extern crate rlua;
extern crate quickersort;
extern crate nalgebra;

mod raylib;
use raylib::{Texture2D, Rectangle, DrawTexturePro, Color};

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

use sprite_loader::load_texture;
mod sprite_loader;

use storage::*;
mod storage;




const FRAME_RATE: f32 = 60.0;
const FRAME_TIME: f32 = 1.0/FRAME_RATE;

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

fn do_file(state: &rlua::Lua, fname: &str) -> std::result::Result<(), rlua::Error> {
    let mut file = std::fs::File::open(fname).unwrap();
    let mut data = String::new();
    use std::io::Read;
    let _ = file.read_to_string(&mut data);

    state.eval::<()>(data.as_str(), Some(fname))
}

fn load_string_from_table(state: &rlua::Table, name: &str, into: &mut String){
    match state.get::<&str,String>(name) {
        Ok(t) => *into = t.to_string(),
        Err(_) => {},
    };
}

fn load_number_from_table(state: &rlua::Table, name: &str, into: &mut f32){
    match state.get::<&str,f32>(name) {
        Ok(t) => *into = t,
        Err(_) => {},
    };
}

#[allow(unused)]
fn load_entity(id: id_type,
               table: &mut rlua::Table,
               game_data: &std::sync::Arc<GameData>) {
        /*
    for pair in table.clone().pairs() {
        let component_name: String;
        let mut component_data: rlua::Table;

        match pair {
            Ok(tup) => {
                component_name = tup.0;
                component_data = tup.1;
            },
            _ => {
                return;
            },
        }

        match component_name.as_str() {
            _ => print!("unrecognized component name {}\n",component_name),
        };
    }
        */
}

#[derive(Clone)]
struct SineMovement {
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
struct Physical{
    x: f32,
    y: f32,
    xvel: f32,
    yvel: f32,
}

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
struct Drawable{
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
struct Collidable{
    radius: f32,
}

#[derive(Clone)]
struct PlayerControl{}
#[derive(Clone)]
struct DespawnFarRight{}
#[derive(Clone)]
struct DespawnFarLeft{}

#[derive(Clone)]
struct Bullet{
    damage: f32,
}

#[derive(Clone)]
struct Powerup {
    regen_increase: f32,
    fire_rate_increase: f32,
}

#[derive(Clone)]
struct PlayerStats {
    movement_speed: f32,
    owned: Vec<Powerup>,
}

#[derive(Clone)]
struct Weapon {
    fire_rate: f32,
    fire_velocity: f32,
    gun_cooldown_frames : i32,
    to_spawn : Bullet,

    drawable: Option<Drawable>,

    direction: f32,
    offset: f32,
}

#[derive(Clone)]
struct WeaponBuilder{
    thing: Weapon,
}

impl WeaponBuilder {
    fn new() -> Self {
        WeaponBuilder{ thing: Weapon{
            fire_rate: 1.0,
            fire_velocity: 0.1,
            gun_cooldown_frames: 0,
            to_spawn: Bullet{damage: 10.0},
            drawable: None,
            direction: 1.0,
            offset: 40.0,
        }}
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

    fn to_spawn(mut self, val: Bullet) -> Self{
        self.thing.to_spawn = val;
        self
    }

    fn drawable(mut self, val: Drawable) -> Self{
        self.thing.drawable = Some(val);
        self
    }

    fn direction(mut self, val: f32) -> Self{
        self.thing.direction = val;
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
struct Shield {
    max_shield: f32,
    ammount: f32,
    regen: f32,
}

#[derive(Clone)]
struct ShieldBuilder {
    thing: Shield,
}

impl ShieldBuilder {
    fn new() -> Self {
        Self{ thing: Shield{
            max_shield: 100.0,
            ammount: 100.0,
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
struct AutoFire{}

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

    unused_ids: Vec<id_type>,
    max_id: i64,

    frame_count: i64,
    to_destroy: Vec<id_type>,
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

            unused_ids: Vec::<id_type>::new(),
            max_id: 0,

            frame_count: 0,
            to_destroy: Vec::<id_type>::new(),
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
        self.frame_count+=1;

        self.do_player_input();
        self.do_sine_movement();
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

            if fire {
                let bul_id = self.spawn_bullet();

                let mut weapon = self.weapon_list.get(id).unwrap();
                weapon.gun_cooldown_frames = weapon.fire_rate as i32;

                let mut bul_phy = self.physical_list.get(bul_id).unwrap().clone();

                let phy1 = self.physical_list.get(id).unwrap().clone();

                self.drawable_list.add(bul_id, weapon.clone().drawable.unwrap().clone());

                bul_phy.xvel = weapon.direction * weapon.fire_velocity;

                bul_phy.x = phy1.x + weapon.offset * weapon.direction;
                bul_phy.y = phy1.y;

                self.physical_list.add(bul_id, bul_phy);
                self.weapon_list.add(id, weapon);
            }
        }
    }

    fn draw(&mut self){
        let mask = b_and(self.drawable_list.mask.clone() , self.physical_list.mask.clone());
        for id in 0..self.max_id {
            if  ! mask[id as usize] { continue; }
            let drw = self.drawable_list.get(id).unwrap().clone();
            let pos = self.physical_list.get(id).unwrap().clone();

            let txt = drw.texture;
            let src_rect = Rectangle {x:0, y:0, width:txt.width, height:txt.height};
            let dst_rect = Rectangle {
                x: (pos.x + (txt.width / 2) as f32) as i32,
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
    }

    fn spawn_bullet(&mut self) -> id_type {
        let id = self.alloc_id();

        self.physical_list.add(id, PhysicalBuilder::new().build());
        self.collidable_list.add(id, Collidable{ radius: 40.0 });
        self.despawn_right.add(id, DespawnFarRight{});
        self.despawn_left.add(id, DespawnFarLeft{});
        self.bullet_list.add(id, Bullet{damage:10.0});

        return id;
    }

    fn is_colliding(&mut self, a: id_type, b: id_type) -> bool {
        if self.physical_list.get(a).is_none() || self.physical_list.get(b).is_none() {
            return false;
        }
        let a_phy = self.physical_list.get(a).unwrap();
        let b_phy = self.physical_list.get(b).unwrap();

        let a_pos = Vector2::new( a_phy.x, a_phy.y);
        let b_pos = Vector2::new( b_phy.x, b_phy.y);

        let a_col = self.collidable_list.get(a).unwrap();
        let b_col = self.collidable_list.get(b).unwrap();

        return CheckCollisionCircles(&a_pos, a_col.radius,
                                     &b_pos, b_col.radius);
    }

    fn handle_collision(&mut self, a: id_type, b: id_type) {
        if self.controllable_list.contains(a) && self.powerup_list.contains(b) {
            let mut stats = self.player_stats_list.get(a).unwrap().clone();
            let power = self.powerup_list.get(b).unwrap().clone();

            stats.movement_speed *= 0.95;
            stats.owned.push(power);

            self.to_destroy.push(b);

            self.player_stats_list.add(a, stats);
        }

        if self.shield_list.contains(a) && self.bullet_list.contains(b) {
            let mut shield = self.shield_list.get(a).unwrap();
            let bullet = self.bullet_list.get(b).unwrap().clone();

            shield.ammount -= bullet.damage;
            self.shield_list.add(a, shield);

            self.to_destroy.push(b);
        }
    }


    fn do_death_check(&mut self){
        let mask = self.shield_list.mask.clone();

        for id in 0..mask.len() as id_type {
            if !mask[id as usize]{ continue; }

            let shield = self.shield_list.get(id).unwrap().clone();
            if shield.ammount < 0.0 {
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

        for id in to_check.clone() {
            //print!("checking id {} on frame {}\n", id, self.frame_count);
            self.physical_list.get(id).unwrap();
        }

        for outer_id in 0..self.max_id {
            for inner_id in outer_id+1..self.max_id {
                if self.is_colliding(outer_id as id_type, inner_id as id_type) {
                    self.handle_collision(outer_id as id_type, inner_id as id_type);
                    self.handle_collision(inner_id as id_type, outer_id as id_type);
                }
            }
        }
        self.do_destroy();
    }

    fn do_despawn(&mut self){
        let mask = b_and( b_or(self.despawn_left.mask.clone() , self.despawn_right.mask.clone()) , self.physical_list.mask.clone());

        for id in 0..mask.len() as id_type {
            if  !mask[id as usize] { continue; }

            let phy = self.physical_list.get(id).unwrap().clone();
            if self.despawn_left.contains(id) && phy.x < 20.0 {
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


            let player_speed = self.player_stats_list.get(id).unwrap().movement_speed;

            if IsKeyDown(KEY_W) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.y -= player_speed;
                self.physical_list.add(id, phy);
            }
            if IsKeyDown(KEY_S) {
                let mut phy = self.physical_list.get(id).unwrap().clone();
                phy.y += player_speed;
                self.physical_list.add(id, phy);
            }

            if IsKeyPressed(KEY_SPACE) {
                self.auto_fire_list.add(id, AutoFire{});
            }

            if IsKeyReleased(KEY_SPACE) {
                self.auto_fire_list.remove(id);
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

    fn do_movement(&mut self){
        let mask = self.physical_list.mask.clone();
        for id in 0..self.max_id as id_type {
            if ! mask[id as usize] { continue; }
            let mut phy = self.physical_list.get(id).unwrap().clone();
            phy.x += phy.xvel * FRAME_RATE;
            phy.y += phy.yvel * FRAME_RATE;
            self.physical_list.add(id, phy);
        }
    }
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

    InitWindow(1200, 1000, "Dodgem");
    SetTargetFPS(FRAME_RATE as i32);

    let mut gl = GameData::new();

    let mut id = gl.alloc_id();
    //player
    gl.drawable_list.add(id, DrawableBuilder::new()
                         .texture_by_name("player.png".to_string())
                         .layer(1.0)
                         .build());
    gl.physical_list.add(id, PhysicalBuilder::new()
                         .x(100.0)
                         .y(200.0)
                         .build());
    gl.controllable_list.add(id, PlayerControl{});
    gl.collidable_list.add(id, Collidable{ radius: 40.0});
    gl.player_stats_list.add(id, PlayerStats{
        movement_speed: 15.0,
        owned: vec![]
    });
    gl.weapon_list.add(id, WeaponBuilder::new()
                       .fire_rate(0.25 * FRAME_RATE)
                       .offset(80.0)
                       .direction(1.0)
                       .to_spawn(Bullet{damage: 10.0})
                       .gun_cooldown_frames(10)
                       .fire_velocity(1.0)
                       .drawable(DrawableBuilder::new()
                                 .texture_by_name("red_ball.png".to_string())
                                 .layer(1.0)
                                 .build())
                       .build());
    gl.shield_list.add(id, ShieldBuilder::new()
                  .ammount(30.0)
                  .regen(0.001)
                  .build());

    //powerup
    id = gl.alloc_id();
    gl.drawable_list.add(id, DrawableBuilder::new()
                         .texture_by_name("fire_rate_up.png".to_string())
                         .layer(1.0)
                         .build());
    gl.physical_list.add(id, PhysicalBuilder::new()
                         .x(800.0)
                         .y(200.0)
                         .xvel(-0.1)
                         .build());
    gl.collidable_list.add(id, Collidable{ radius: 40.0 });
    gl.despawn_left.add(id, DespawnFarLeft{});
    gl.powerup_list.add(id, Powerup{regen_increase: 1.05, fire_rate_increase: 0.0});

    //enemy
    id = gl.alloc_id();
    gl.drawable_list.add(id, DrawableBuilder::new()
                         .texture_by_name("enemy1.png".to_string())
                         .layer(1.0)
                         .build());
    gl.physical_list.add(id, PhysicalBuilder::new()
                         .x(1000.0)
                         .y(500.0)
                         .xvel(-0.01)
                         .build());
    gl.collidable_list.add(id, Collidable{ radius: 40.0 });
    gl.despawn_left.add(id, DespawnFarLeft{});
    gl.shield_list.add(id, ShieldBuilder::new()
                       .ammount(10.0)
                       .regen(0.0)
                       .build());
    gl.weapon_list.add(id, WeaponBuilder::new()
                       .fire_rate(2.0*FRAME_RATE)
                       .fire_velocity(0.4)
                       .direction(-1.0)
                       .offset(80.0)
                       .gun_cooldown_frames(1)
                       .drawable(DrawableBuilder::new()
                                 .texture_by_name("red_ball.png".to_string())
                                 .layer(1.0)
                                 .build())
                       .build());
    gl.sine_movement_list.add(id, SineMovementBuilder::new()
                              .frequency(1.0)
                              .amplitude(6.0)
                              .build());
    //gl.auto_fire_list.add(id, AutoFire{});

    //enemy
    id = gl.alloc_id();
    gl.drawable_list.add(id, DrawableBuilder::new()
                         .texture_by_name("enemy1.png".to_string())
                         .layer(1.0)
                         .build());
    gl.physical_list.add(id, PhysicalBuilder::new()
                         .x(1000.0)
                         .y(200.0)
                         .xvel(-0.01)
                         .build());
    gl.collidable_list.add(id, Collidable{radius:40.0 });
    gl.despawn_left.add(id, DespawnFarLeft{});
    gl.shield_list.add(id, ShieldBuilder::new()
                       .ammount(10.0)
                       .regen(0.0)
                       .build());
    gl.weapon_list.add(id, WeaponBuilder::new()
                       .fire_rate(2.0*FRAME_RATE)
                       .fire_velocity(0.4)
                       .direction(-1.0)
                       .offset(80.0)
                       .gun_cooldown_frames(30)
                       .drawable(DrawableBuilder::new()
                                 .texture_by_name("red_ball.png".to_string())
                                 .layer(1.0)
                                 .build())
                       .build());
    gl.sine_movement_list.add(id, SineMovementBuilder::new()
                              .frequency(0.8)
                              .amplitude(9.0)
                              .build());
    gl.auto_fire_list.add(id, AutoFire{});


    while ! WindowShouldClose() {
        gl.step();

        ClearBackground(Color{r:0, g:0, b:0, a:255} );
        BeginDrawing();
        DrawFPS(3,3);
        gl.draw();
        EndDrawing();
    }
}
