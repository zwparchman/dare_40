#[macro_use] extern crate smart_default;
#[macro_use] extern crate rlua_table_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate cached;
// `cached!` macro requires the `lazy_static!` macro
#[macro_use] extern crate lazy_static;

extern crate quickersort;
extern crate nalgebra;
extern crate rand;
extern crate hibitset;
extern crate pretty_env_logger;
extern crate rlua;

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
// const FRAME_RATE: f32 = 240.0;
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
struct StateWrapper{
    rstate: Arc<Mutex<rlua::Lua>>,
}

impl StateWrapper {
    fn new() -> StateWrapper {
        StateWrapper {
            rstate: Arc::new(Mutex::new(rlua::Lua::new()))
        }
    }

    fn lock(&self ) -> LockResult<MutexGuard<rlua::Lua>> {
        self.rstate.lock()
    }
}
unsafe impl std::marker::Sync for StateWrapper{}
unsafe impl std::marker::Send for StateWrapper{}

#[derive(Default, Clone, FromLuaTable)]
pub struct Drag {
    x: f32,
    y: f32,
}

#[derive(Default, Clone, FromLuaTable)]
pub struct StopAt {
    xloc: f32,
}


#[derive(Default, Clone, FromLuaTable)]
pub struct SineMovementX {
    frequency: f32,
    step: i32,
    amplitude: f32,
}

#[derive(Default, Clone, FromLuaTable)]
pub struct SineMovement {
    frequency: f32,
    step: i32,
    amplitude: f32,
}


#[derive(Default, Clone, FromLuaTable)]
pub struct ClampY {
    low: f32,
    high: f32,
}

#[derive(Clone, Default, FromLuaTable)]
pub struct PointAlongMovementVector{
    angular_offset: f32,
    last_x: f32,
    last_y: f32,
}

#[derive(Default, Clone, FromLuaTable)]
pub struct Physical{
    x: f32,
    y: f32,

    xvel: f32,
    yvel: f32,

    xacc: f32,
    yacc: f32,

    angle: f32,
    angular_velocity: f32,
}

#[derive(Clone, SmartDefault, FromLuaTable)]
pub struct Animation {
    #[default = "1"]
    frames: i32,

    times: Vec<f32>,
    current_time: f32,
    current_frame: i32,
}

impl Animation {
    fn step(&mut self) {
        self.current_time += FRAME_TIME;
        if self.times[self.current_frame as usize] < self.current_time {
            self.current_frame = (self.current_frame + 1) % self.frames;
            self.current_time = 0.0;
        }
    }

    fn get_src_rect(&self, drw: &Drawable) -> Rectangle {
        let txt = drw.texture.val.clone();
        let total_width = txt.get_width();
        let total_height = txt.get_height();

        let frame_width = total_width / self.frames;
        Rectangle{
            x: frame_width * self.current_frame,
            y: total_height,
            width: frame_width,
            height: total_height,
        }
    }
}

#[derive(Clone, Default, FromLuaTable)]
pub struct Drawable{
    texture: TextureHandle,
    layer: f32,
    tint: Color,
}

#[derive(Clone)]
pub struct TextureHandle{
    pub val: Arc<Texture2D>,
}

impl TextureHandle {
    pub fn from_texture2d(texture: Texture2D) -> Self {
        Self{
            val: Arc::new(texture),
        }
    }

    pub fn from_file_str(fname: &str) -> Self {
        Self {
            val: load_texture(fname.to_string()).unwrap(),
        }
    }
}

impl std::default::Default for TextureHandle {
    fn default() -> Self {
        TextureHandle::from_file_str("no-texture.png")
    }
}



impl rlua::UserData for TextureHandle {
    fn add_methods(_methods: &mut rlua::UserDataMethods<Self>) {
    }
}

#[derive(Clone, SmartDefault)]
pub struct ImageHandle {
    #[default = "load_image(\"no-texture.png\".to_string()).unwrap()"]
    pub val: Arc<Image>
}

impl ImageHandle {
    fn from_image(img: Image) -> Self {
        Self{
            val: Arc::new(img),
        }
    }
}

impl rlua::UserData for ImageHandle {
    fn add_methods(methods: &mut rlua::UserDataMethods<Self>) {
        methods.add_method("crop",|_, this, (x,y,w,h):(i32, i32, i32, i32)|{
            let mut img: Image = std::sync::Arc::make_mut(&mut this.val.clone()).clone();
            ImageCrop(&mut img, Rectangle{x:x, y:y, width:w, height:h});

            return Ok(ImageHandle::from_image(img));
        });
        methods.add_method("to_texture",|_lua, this, _arg: ()| {
            let img: Image = std::sync::Arc::make_mut(&mut this.val.clone()).clone();
            Ok(TextureHandle::from_texture2d(LoadTextureFromImage(&img)))
        });
    }
}

#[derive(Clone, Default)]
struct SoundHandle {
    val: Option<Arc<Sound>>,
}

fn register_sound(lua: &rlua::Lua) -> Result<(), rlua::Error> {
    let fun = lua.create_function(
        |_, table: rlua::Table| -> Result<SoundHandle, rlua::Error> {
            if let Ok(fname) = table.get::<&str, String>("file") {
                return Ok(SoundHandle::from_file_string(fname));
            } 
            return Ok(SoundHandle::none());
        }
    )?;
    lua.globals().set("Sound", fun).unwrap();
    Ok(())
}

impl rlua::UserData for SoundHandle {
    fn add_methods(_methods: &mut rlua::UserDataMethods<Self>) {
    }
}

impl SoundHandle {
    fn none() -> Self {
        Self { val: None, }
    }

    fn from_file_string(fname: String) -> Self {
        Self {
            val: load_sound(fname),
        }
    }

    #[allow(unused)]
    fn from_file_str(fname: &str) -> Self {
        Self {
            val: load_sound(fname.to_string()),
        }
    }

    fn play(&self) {
        if let Some(ref val) = self.val {
            PlaySound(&*val);
        }
    }
}

#[derive(Clone)]
pub struct DeathEvent{
    sound: SoundHandle,
    spawner: Arc<Spawner>,
    score_add: i64,
    clear_spawn_plan: bool,
}

impl FromLuaTable for DeathEvent {
    fn from_lua_table(table: &rlua::Table) -> Self {
        Self {
            sound: table.get("sound").unwrap_or(SoundHandle::none()),
            spawner: Arc::new( table.get("spawner").unwrap_or(Spawner::new()) ),
            score_add: table.get("score_add").unwrap_or(0),
            clear_spawn_plan: table.get("clear_spawn_plan").unwrap_or(false),
        }
    }
}

impl DeathEvent{
    fn die(&self, mut world: &mut EcsWorld, pos: &Physical) -> ( bool, i64) {
        &self.sound.play();
        self.spawner.spawn_at_pos(&mut world, &pos);
        (self.clear_spawn_plan, self.score_add)
    }

}

#[derive(Clone, Default, FromLuaTable)]
pub struct DrawCollidable{}

#[derive(Clone, Default, FromLuaTable)]
pub struct Collidable{
    radius: f32,
}

#[derive(Clone, Default, FromLuaTable)]
pub struct PlayerControl{}
#[derive(Clone, Default, FromLuaTable)]
pub struct DespawnFarRight{}
#[derive(Clone, Default, FromLuaTable)]
pub struct DespawnFarLeft{
    at: f32,
}

#[derive(Clone, Default, FromLuaTable)]
pub struct DespawnY{}

#[derive(Clone, Default, FromLuaTable)]
pub struct AvoidPlayerY{
    speed: f32,
}

#[derive(Clone, Default, FromLuaTable)]
pub struct FollowPlayerY{
    speed: f32,
    offset: f32,
}

#[derive(Clone, Default, FromLuaTable)]
pub struct Bullet{
    damage: f32,
}

#[derive(Clone, SmartDefault, FromLuaTable)]
pub struct Powerup {
    #[default = "1.0"]
    regen_increase: f32,
    #[default = "1.0"]
    fire_rate_increase: f32,
    #[default = "1.0"]
    fire_damage_increase: f32,
    #[default = "1.0"]
    shield_increase: f32,
    pickup_sound: SoundHandle,
    shot_increase: i32,
}
const PART_INSTALLED_AT: i32 = (FRAME_RATE * 2.0 ) as i32;

#[derive(Clone, Default, FromLuaTable)]
pub struct PlayerStats {
    movement_speed: f32,
    base_speed: f32,
    owned: Vec<Prefab>,
    install_progress: i32,
    install_finish_sound: SoundHandle,
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
    fire_sound: SoundHandle,
    fire_angle: f32,
}

impl FromLuaTable for Weapon {
    fn from_lua_table(table: &rlua::Table) -> Self {
        Self {
            fire_rate: FRAME_RATE * table.get("fire_rate").unwrap_or(10.0),
            fire_velocity: table.get("fire_velocity").unwrap_or(100.0),
            gun_cooldown_frames: (FRAME_RATE * table.get("gun_cooldown_frames").unwrap_or(0) as f32) as i32,
            prefab: Arc::new( table.get("prefab").unwrap_or(PrefabBuilder::new().build())),
            pattern: table.get("pattern").unwrap_or(1),
            direction: table.get("direction").unwrap_or(1.0),
            offset: table.get("offset").unwrap_or(0.0),
            fire_sound: table.get("fire_sound").unwrap_or(SoundHandle::none()),
            fire_angle: table.get("fire_angle").unwrap_or(90.0),
        }
    }
}

#[derive(Clone, Default, FromLuaTable)]
pub struct BossHealthDraw {}

#[derive(Clone, Default)]//, FromLuaTable)]
pub struct TimeoutDeath{
    ticks: i32,
}
impl FromLuaTable for TimeoutDeath {
    fn from_lua_table(table: &rlua::Table) -> Self {
        Self {
            ticks: (table.get::<_,f32>("time").unwrap_or(0.0) * FRAME_RATE ) as i32
        }
    }
}

#[derive(Clone, Default)]
pub struct Shield {
    max_shield: f32,
    ammount: f32,
    regen: f32,
}

impl FromLuaTable for Shield {
    fn from_lua_table(table: &rlua::Table) -> Self {
        let ammount = table.get("ammount").unwrap_or(1.0);
        let max_shield = table.get("max_shield").unwrap_or(ammount);
        let regen = table.get("regen").unwrap_or(0.0);
        Self {
            ammount: ammount,
            max_shield: max_shield,
            regen: regen,
        }
    }
}

#[derive(Clone, Default, FromLuaTable)]
pub struct AutoFire{}

#[derive(Clone, Default, FromLuaTable)]
pub struct Install{}

#[derive(Clone, Default, FromLuaTable)]
pub struct Team{team: i32}


pub struct GameData {
    frame_count: u64,

    spawn_plan: SpawnPlan,
    star_spawner: SpawnPlan,

    world: EcsWorld,
    difficulty: f32,
    wave: i32,
    score: i64,
    paused: bool,

    state: StateWrapper,

    #[allow(unused)]
    rng: rand::isaac::Isaac64Rng,
}

enum GameRequest {
    Normal,
    Restart,
    Quit,
}

pub struct Application {
    game: Option<GameData>,
    state: StateWrapper,
}

impl Application {
    fn new() -> Self {
        let state;
        match load_config("src/config.lua") {
            Ok(val) => {
                state = val;
            },
            _ => {
                error!("Can not start the program without a valid config, aborting");
                std::process::exit(1);
            }
        }

        let h: f32;
        let w: f32;
        {
            let locked = state.lock().unwrap();

            h = locked.globals().get("window_height").unwrap();
            w = locked.globals().get("window_width").unwrap();
        }

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

        InitWindow(w as i32, h as i32, "Dodgem");
        InitAudioDevice();
        SetTargetFPS(FRAME_RATE as i32);

        let gl = GameData::new(state.clone());


        Self {
            game: Some(gl),
            state: state,
        }
    }

    fn run(&mut self) -> () {

        while ! WindowShouldClose() {
            let req;
            if let Some(ref mut game) = self.game {
                req = game.step();

                ClearBackground(Color{r:0, g:0, b:0, a:255} );
                BeginDrawing();
                DrawFPS(3,3);
                game.draw();
                EndDrawing();
            } else {
                req = GameRequest::Quit;
            }

            match req {
                GameRequest::Restart => {
                    debug!("Restart requested");
                    match load_config("src/config.lua") {
                        Ok(val) => self.state = val,
                        _ => {},
                    }

                    self.game = Some(GameData::new(self.state.clone()));
                },
                GameRequest::Normal => {
                }

                GameRequest::Quit => {
                    break;
                }
            }
        }
    }
}

impl GameData {
    fn new(state: StateWrapper) -> Self {
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
            state: state,
        }
    }

    fn spawn_stars(&mut self){
        if self.star_spawner.is_empty() {
            let locked = self.state.lock().unwrap();
            self.star_spawner = gen_star_spawner2(self.frame_count, &locked);
        }
        self.star_spawner.execute(self.frame_count, &mut self.world);
    }

    fn spawn_main(&mut self){

        if self.spawn_plan.is_empty() {
            trace!("Creating new main spawn plan on frame:{} difficulty: {} wave: {}", self.frame_count, self.difficulty, self.wave);
            let player_shield_fraction = self.get_player_shield_fraction();
            self.difficulty += 3.3 * player_shield_fraction;
            self.wave += 1;
            if self.wave == 20 {
                let lua = self.state.lock().unwrap();
                self.spawn_plan = gen_level_spawner_from_lua(self.frame_count,
                                                             self.difficulty,
                                                             5.0 * FRAME_RATE,
                                                             "gen_boss_a_level",
                                                             &*lua);

            } else if self.wave == 40 {
                self.difficulty *= 1.2;
                let lua = self.state.lock().unwrap();
                self.spawn_plan = gen_level_spawner_from_lua(self.frame_count,
                                                             self.difficulty,
                                                             5.0 * FRAME_RATE,
                                                             "gen_boss_b_level",
                                                             &*lua);

            } else {
                let lua = self.state.lock().unwrap();
                self.spawn_plan = gen_level_spawner_from_lua(self.frame_count,
                                                             self.difficulty,
                                                             5.0 * FRAME_RATE,
                                                             "gen_level",
                                                             &*lua);
            }
        }

        let did_spawn = self.spawn_plan.execute(self.frame_count, &mut self.world);
        if did_spawn {
            trace!("Spawned main on  frame {}", self.frame_count);
        }
    }

    fn step(&mut self) -> GameRequest {
        self.spawn_stars();
        self.spawn_main();

        if IsKeyPressed(KEY_P) {
            self.paused = ! self.paused;
        }

        if IsKeyPressed(KEY_F10) {
            match load_config("src/config.lua") {
                Ok(state) => {
                    debug!("Reloaded config file");
                    self.state = state;
                }
                _ => {},
            }
        }

        if IsKeyPressed(KEY_O) {
            let lua = self.state.lock().unwrap();
            let mut plan = gen_level_spawner_from_lua(self.frame_count,
                                                  self.difficulty,
                                                  10 as f32,
                                                  "respawn_player",
                                                  &*lua);

            plan.execute(self.frame_count, &mut self.world);
        }

        if IsKeyPressed(KEY_R) {
            return GameRequest::Restart;
        }

        if ! self.paused {
            self.do_player_input();
            self.do_sine_movement();
            self.do_sine_movement_x();
            self.do_install();
            self.do_stop_at();
            self.do_drag();
            self.do_movement();
            self.do_point_along_movement_vector();
            self.do_clamp_y();
            self.do_despawn();
            self.do_follow_player();
            self.do_avoid_player();
            self.do_collision();
            self.do_timeout_death();
            self.do_death_check();
            self.do_shield_regen();
            self.do_weapon_cooldown();
            self.do_weapon_fire();
            self.do_animation_step();

            self.world.maintain();
            self.frame_count+=1;
        }

        return GameRequest::Normal;
    }

    fn do_animation_step(&mut self){
        let mask = self.world.animation_list.mask.clone();

        for id in mask {
            let mut anim = self.world.animation_list.get(id as IDType).unwrap();
            anim.step();
            self.world.animation_list.add(id as IDType, anim);
        }
    }

    fn do_drag(&mut self) {
        let mask = self.world.drag_list.mask.clone() &
            self.world.physical_list.mask.clone();

        for id in mask {
            let drag = self.world.drag_list.get(id as IDType).unwrap();
            let mut phy  = self.world.physical_list.get(id as IDType).unwrap();

            phy.xvel *= 1.0 - ( drag.x * FRAME_TIME );
            phy.yvel *= 1.0 - ( drag.y * FRAME_TIME );

            self.world.physical_list.add(id as IDType, phy);
        }
    }

    fn get_player_id(&self) -> Option<IDType> {
        let mask = self.world.controllable_list.mask.clone();
        for id in mask {
            return Some(id as IDType);
        }
        None
    }

    fn do_clamp_y(&mut self){
        let mask = self.world.clamp_y_list.mask.clone() &
                   self.world.physical_list.mask.clone();
        for id_ in mask {
            let id = id_ as IDType;

            let mut phy = self.world.physical_list.get(id).unwrap();
            let clamp = self.world.clamp_y_list.get(id).unwrap();

            if phy.y < clamp.low {
                phy.y = clamp.low;
            } else if phy.y > clamp.high {
                phy.y = clamp.high;
            }

            self.world.physical_list.add(id, phy);
        }
    }

    fn do_avoid_player(&mut self){
        let mask = self.world.avoid_player_y_list.mask.clone() &
                   self.world.physical_list.mask.clone();

        let player_id_opt = self.get_player_id();
        if player_id_opt.is_none() { return }
        let player_id = player_id_opt.unwrap();

        let player_phy = self.world.physical_list.get(player_id as IDType).unwrap();

        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            let fol = self.world.avoid_player_y_list.get(id as IDType).unwrap();

            let diff = - player_phy.y + phy.y;
            let to_move = diff.abs();
            let dir = { if diff > 0.0 { 1.0 } else { -1.0 } };
            let will_move = min_float(to_move, fol.speed * FRAME_TIME);

            phy.y += will_move * dir;

            self.world.physical_list.add(id as IDType, phy);
        }
    }



    fn do_follow_player(&mut self){
        let mask = self.world.follow_player_y_list.mask.clone() &
                   self.world.physical_list.mask.clone();

        let player_id_opt = self.get_player_id();
        if player_id_opt.is_none() { return }
        let player_id = player_id_opt.unwrap();

        let player_phy = self.world.physical_list.get(player_id as IDType).unwrap();

        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            let fol = self.world.follow_player_y_list.get(id as IDType).unwrap();

            let diff = player_phy.y - phy.y + fol.offset;
            let to_move = diff.abs();
            let dir = { if diff > 0.0 { 1.0 } else { -1.0 } };
            let will_move = min_float(to_move, fol.speed * FRAME_TIME);

            phy.y += will_move * dir;

            self.world.physical_list.add(id as IDType, phy);
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
                weapon.fire_sound.play();
                weapon.gun_cooldown_frames = weapon.fire_rate as i32;

                for mut angle in get_shot_angles(weapon.fire_angle, pattern) {
                    let bul_id = prefab.spawn(&mut self.world);

                    let mut bul_phy;
                    if let Some(bul_phy2) = self.world.physical_list.get(bul_id) {
                        bul_phy = bul_phy2.clone();
                    } else {
                        print!("Error, bullet_phy does not exist for id {}\n", bul_id);
                        continue;
                    }

                    angle -= phy1.angle;

                    bul_phy.xvel = (DEG2RAD as f32 * angle).cos() *  weapon.fire_velocity;
                    bul_phy.yvel = (DEG2RAD as f32 * angle).sin() *  weapon.fire_velocity;
                    bul_phy.x = phy1.x + weapon.offset * weapon.direction;
                    bul_phy.y = phy1.y;
                    bul_phy.angle = -angle;

                    self.world.physical_list.add(bul_id, bul_phy);

                }
                self.world.weapon_list.add(id as IDType, weapon);
            }
        }
    }

    fn draw_by_id( &mut self, id: IDType) {
            let drw = self.world.drawable_list.get(id).unwrap();
            let pos = self.world.physical_list.get(id).unwrap();

            let txt = &drw.texture.val;
            let mut half_width = txt.get_width()/2;
            let mut half_height = txt.get_height() / 2;

            let src_rect;
            let dst_rect;

            if let Some(animation) = self.world.animation_list.get(id) {
                src_rect = animation.get_src_rect(&drw);
                dst_rect = Rectangle {
                    x: pos.x as i32,
                    y: pos.y as i32,
                    width: src_rect.width,
                    height: src_rect.height,
                };
                half_width = src_rect.width / 2;
                half_height = src_rect.height / 2;
                self.world.animation_list.add(id, animation.clone());
            } else {
                src_rect = Rectangle {x:0, y:0, width: txt.get_width(), height:txt.get_height()};
                dst_rect = Rectangle {
                    x: (pos.x  as f32) as i32,
                    y: (pos.y as f32) as i32,
                    width: (txt.get_width()) as i32,
                    height: (txt.get_height()) as i32
                };
            }

            if self.world.draw_collidable_list.contains(id) {
                let phy = self.world.physical_list.get(id).clone().unwrap();
                let col = self.world.collidable_list.get(id).clone().unwrap();
                DrawCircle(phy.x as i32, phy.y as i32, col.radius, Color{r:255, g:255, b:255, a:255});
            }
            let origin = Vector2::new(half_width as f32, half_height as f32);
            DrawTexturePro( &*txt,
                            src_rect,
                            dst_rect,
                            origin,
                            pos.angle,
                            drw.tint.clone());
    }

    fn draw(&mut self){
        let mask = self.world.drawable_list.mask.clone() & self.world.physical_list.mask.clone();
        let mut to_draw = vec![];
        for id in mask {
            to_draw.push((id, self.world.drawable_list.get(id as IDType).unwrap()));
        }

        to_draw.sort_by_key( |tup| { (tup.1.layer * 1000.0) as i64  });

        for (id, _drw) in to_draw {
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
            prefab.physical = Some({
                let mut phy = Physical::default();
                phy.x = 1250.0;
                phy.y = i as f32 * 50.0 + 200.0;
                phy
            });

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

        return CheckCollisionCircles(&a_pos, a_col.radius,
                                     &b_pos, b_col.radius);
    }

    fn handle_collision(&mut self, a: IDType, b: IDType) {
        if self.world.controllable_list.contains(a) && self.world.powerup_list.contains(b) {
            let mut stats = self.world.player_stats_list.get(a).unwrap();
            let power = self.world.powerup_list.get(b).unwrap();

            stats.movement_speed=stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32);

            let mut prefab = self.id_to_prefab(b);
            prefab.physical = None;
            stats.owned.push(prefab);

            power.pickup_sound.play();
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
                stats.install_finish_sound.play();
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

                let damage;

                {
                    let bullet: &mut Bullet = &mut weapon.prefab.bullet.clone().unwrap();
                    bullet.damage *= upgrade.fire_damage_increase;
                    damage = bullet.damage;

                    let mut wc = weapon.clone();
                    let bullet_prefab = std::sync::Arc::make_mut(&mut wc.prefab);
                    bullet_prefab.bullet = Some(bullet.clone());
                    weapon.prefab = Arc::new(bullet_prefab.clone());
                }


                debug!("powerup data: Current stats: fire_rate {} regen {} max_shield {} pattern {} damage {}",
                       weapon.fire_rate,
                       shield.regen,
                       shield.max_shield,
                       weapon.pattern,
                       damage);

                self.world.shield_list.add(id as IDType, shield);
                self.world.weapon_list.add(id as IDType, weapon);
                stats.owned.remove(0);
            }

            stats.movement_speed = stats.base_speed * SLOWDOWN_FACTOR.powf(stats.owned.len() as f32 );
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
        let mask = (self.world.despawn_far_left.mask.clone() |
                    self.world.despawn_far_right.mask.clone() |
                    self.world.despawn_y_list.mask.clone() ) &
                    self.world.physical_list.mask.clone();

        for id in mask {
            let phy = self.world.physical_list.get(id as IDType).unwrap();
            if let Some(desp) = self.world.despawn_far_left.get(id as IDType) {
                if desp.at > phy.x {
                    self.world.destroy_later(id as IDType);
                }
            }
            if self.world.despawn_far_right.contains(id as IDType) && phy.x > GetScreenWidth() as f32 + 120.0 {
                self.world.destroy_later(id as IDType);
            }

            if self.world.despawn_y_list.contains(id as IDType) && 
                    ( phy.y < -20.0 || phy.y > GetScreenHeight() as f32 + 20.0) {
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

    fn do_point_along_movement_vector(&mut self) {
        let mask = self.world.point_along_movement_vector_list.mask.clone() &
            self.world.physical_list.mask.clone();

        for id in mask {
            let mut point = self.world.point_along_movement_vector_list.get(id as IDType).unwrap();
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();

            let diff_x = phy.x - point.last_x;
            let diff_y = phy.y - point.last_y;

            phy.angle = f32::atan2(-diff_x, -diff_y) * RAD2DEG as f32 + point.angular_offset + 90.0;

            point.last_x = phy.x;
            point.last_y = phy.y;

            self.world.point_along_movement_vector_list.add(id as IDType, point);
            self.world.physical_list.add(id as IDType, phy);
        }
    }

    fn do_movement(&mut self){
        let mask = self.world.physical_list.mask.clone();
        for id in mask {
            let mut phy = self.world.physical_list.get(id as IDType).unwrap();
            phy.xvel += phy.xacc * FRAME_TIME;
            phy.yvel += phy.yacc * FRAME_TIME;
            phy.x += phy.xvel * FRAME_TIME;
            phy.y += phy.yvel * FRAME_TIME;

            phy.angle += phy.angular_velocity * FRAME_TIME;

            while phy.angle > 720.0 {
                phy.angle -= 360.0
            }

            while phy.angle < -720.0 {
                phy.angle += 360.0
            }

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

fn do_file(state: &rlua::Lua, fname: &str) -> std::result::Result<(), rlua::Error> {
    let mut file = std::fs::File::open(fname).unwrap();
    let mut data = String::new();
    use std::io::Read;
    let _ = file.read_to_string(&mut data);

    state.eval::<()>(data.as_str(), Some(fname))
}

trait FromLuaTable {
    fn from_lua_table(table: &rlua::Table) -> Self;
}

fn load_config( fname: &str) -> Result<StateWrapper, rlua::Error> {
    let state = StateWrapper::new();
    {
        let locked = state.lock().unwrap();
        register_sound(&locked).unwrap();
        register_level_gen(&locked).unwrap();
        register_ecs(&locked).unwrap();


        match do_file(&locked, fname) {
            Ok(_) => {
                trace!("config loaded sucessfully");
            },
            Err(e) => {
                error!("config load failed: {}", e);
                return Err(e);
            },
        }
    }
    return Ok(state);
}

fn main(){
    pretty_env_logger::init().unwrap();

    let mut app = Application::new();
    app.run();
}
