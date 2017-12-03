extern crate nalgebra;

type Vector2f = nalgebra::Vector2<f32>;

#[allow(unused)]
#[allow(bad_style)]
mod raylib_raw;

use std::ffi::CString;
use std::ops;
use std::clone::Clone;

use std::ptr;
use std::f32;


pub use self::raylib_raw::Color;
pub use self::raylib_raw::Image;
pub use self::raylib_raw::Texture2D;
pub use self::raylib_raw::Rectangle;

////////////////////////////////////////////////////////////
// Image stuff
impl ops::Drop for Image{
    fn drop(&mut self){
        //print!("unloading image\n");
        UnloadImage(self);
    }
}

impl Clone for Image{
    fn clone(&self) -> Image {
        let ret = ImageCopy(self);
        return ret;
    }
}

use self::raylib_raw::bool_;

#[allow(unused)]
#[allow(bad_style)]
pub fn InitWindow(h: i32, w: i32, title: &str){
    unsafe{
        let title_c = CString::new(title).unwrap();
        raylib_raw::InitWindow(h as i32 , w as i32 , title_c.as_ptr());
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn GetScreenWidth() -> i32 {
    unsafe {
        return raylib_raw::GetScreenWidth() as i32;
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn GetScreenHeight() -> i32 {
    unsafe {
        return raylib_raw::GetScreenHeight() as i32;
    }
}


#[allow(bad_style)]
#[allow(unused)]
pub fn SetConfigFlags(flags: u32) {
    unsafe {
        raylib_raw::SetConfigFlags(flags as i8);
    }
}


#[allow(bad_style)]
#[allow(unused)]
pub fn CloseWindow(){
    unsafe{
        raylib_raw::CloseWindow();
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn WindowShouldClose() -> bool {
    unsafe{
        return bool_::true_ == raylib_raw::WindowShouldClose();
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn DrawFPS(x: i32, y: i32) {
    unsafe{
        return raylib_raw::DrawFPS(x,y);
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn SetTargetFPS(val: i32) {
    unsafe{
        return raylib_raw::SetTargetFPS(val);
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn EndDrawing() {
    unsafe{
        raylib_raw::EndDrawing();
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn BeginDrawing() {
    unsafe{
        raylib_raw::BeginDrawing();
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn ClearBackground(color : Color) {
    unsafe{
        raylib_raw::ClearBackground(color);
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn DrawTextureEx(texture : Texture2D, pos: Vector2f, rot: f32, scale: f32, color: Color){
    let vec = raylib_raw::Vector2 { x: pos.x, y: pos.y };
    unsafe{
        raylib_raw::DrawTextureEx(texture, vec, rot, scale, color);
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn DrawTexturePro(texture: &Texture2D, sourceRec: Rectangle,
                      destRec: Rectangle, origin: Vector2f, rotation: f32,
                      tint: Color){
    unsafe{
        let vec = raylib_raw::Vector2 { x:origin.x, y:origin.y };
        let base = texture as *const Texture2D;
        let raw = ptr::read(base);
        raylib_raw::DrawTexturePro(raw, sourceRec, destRec, vec,
                                   rotation, tint);
    }
}



#[allow(bad_style)]
#[allow(unused)]
pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool {
    unsafe {
        return raylib_raw::IsKeyDown(key) == raylib_raw::bool_::true_;
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool {
    unsafe {
        return raylib_raw::IsKeyPressed(key) == raylib_raw::bool_::true_;
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool {
    unsafe {
        return raylib_raw::IsKeyReleased(key) == raylib_raw::bool_::true_;
    }
}





#[allow(bad_style)]
#[allow(unused)]
pub fn LoadTexture( fname: &str) -> Texture2D{
    unsafe{
        let name_c = CString::new(fname).unwrap();
        return raylib_raw::LoadTexture( name_c.as_ptr() );
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn LoadTextureFromImage( img: &Image) -> Texture2D{
    unsafe{
        let base = img as *const Image;
        let raw = ptr::read(base);
        return raylib_raw::LoadTextureFromImage(raw);
    }
}



#[allow(bad_style)]
#[allow(unused)]
pub fn LoadImage( fname: &str) -> Image{
    unsafe{
        let name_c = CString::new(fname).unwrap();
        return raylib_raw::LoadImage( name_c.as_ptr() );
    }
}


#[allow(bad_style)]
#[allow(unused)]
pub fn ImageCrop( img: &mut Image, rect: Rectangle) {
    unsafe {
        let ptr = img as *mut Image;
        raylib_raw::ImageCrop(ptr, rect);
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn ImageCopy( img: &Image) -> Image {
    unsafe {
        let base = img as *const Image;
        let raw = ptr::read(base);
        return raylib_raw::ImageCopy(raw);
    }
}

#[allow(bad_style)]
#[allow(unused)]
pub fn UnloadImage( img: &mut Image) {
    unsafe{
        let base = img as *mut Image;
        let raw = ptr::read(base);
        raylib_raw::UnloadImage(raw);
    }
}

        




#[allow(bad_style)]
#[allow(unused)]
pub fn GetMousePosition() -> Vector2f {
    unsafe {
        let pos = raylib_raw::GetMousePosition();
        Vector2f::new(pos.x, pos.y)
    }
}


#[allow(unused)]
#[allow(bad_style)]
pub fn DrawCircleV(center: &Vector2f, radius: f32, color: Color) {
    let vec = raylib_raw::Vector2{x:center.x, y:center.y};
    unsafe {
        raylib_raw::DrawCircleV(vec, radius, color);
    }
}

#[allow(unused)]
#[allow(bad_style)]
pub fn CheckCollisionCircles(center1: &Vector2f, radius1: f32,
                             center2: &Vector2f, radius2: f32) -> bool {
    let v1 = raylib_raw::Vector2{x:center1.x, y: center1.y };
    let v2 = raylib_raw::Vector2{x:center2.x, y: center2.y };
    unsafe{
        raylib_raw::CheckCollisionCircles(v1, radius1, v2, radius2) == bool_::true_
    }
}



pub use self::raylib_raw::PI;
pub use self::raylib_raw::DEG2RAD;
pub use self::raylib_raw::RAD2DEG;
pub use self::raylib_raw::FLAG_SHOW_LOGO;
pub use self::raylib_raw::FLAG_FULLSCREEN_MODE;
pub use self::raylib_raw::FLAG_WINDOW_RESIZABLE;
pub use self::raylib_raw::FLAG_WINDOW_DECORATED;
pub use self::raylib_raw::FLAG_WINDOW_TRANSPARENT;
pub use self::raylib_raw::FLAG_MSAA_4X_HINT;
pub use self::raylib_raw::FLAG_VSYNC_HINT;
pub use self::raylib_raw::KEY_SPACE;
pub use self::raylib_raw::KEY_ESCAPE;
pub use self::raylib_raw::KEY_ENTER;
pub use self::raylib_raw::KEY_BACKSPACE;
pub use self::raylib_raw::KEY_RIGHT;
pub use self::raylib_raw::KEY_LEFT;
pub use self::raylib_raw::KEY_DOWN;
pub use self::raylib_raw::KEY_UP;
pub use self::raylib_raw::KEY_F1;
pub use self::raylib_raw::KEY_F2;
pub use self::raylib_raw::KEY_F3;
pub use self::raylib_raw::KEY_F4;
pub use self::raylib_raw::KEY_F5;
pub use self::raylib_raw::KEY_F6;
pub use self::raylib_raw::KEY_F7;
pub use self::raylib_raw::KEY_F8;
pub use self::raylib_raw::KEY_F9;
pub use self::raylib_raw::KEY_F10;
pub use self::raylib_raw::KEY_F11;
pub use self::raylib_raw::KEY_F12;
pub use self::raylib_raw::KEY_LEFT_SHIFT;
pub use self::raylib_raw::KEY_LEFT_CONTROL;
pub use self::raylib_raw::KEY_LEFT_ALT;
pub use self::raylib_raw::KEY_RIGHT_SHIFT;
pub use self::raylib_raw::KEY_RIGHT_CONTROL;
pub use self::raylib_raw::KEY_RIGHT_ALT;
pub use self::raylib_raw::KEY_ZERO;
pub use self::raylib_raw::KEY_ONE;
pub use self::raylib_raw::KEY_TWO;
pub use self::raylib_raw::KEY_THREE;
pub use self::raylib_raw::KEY_FOUR;
pub use self::raylib_raw::KEY_FIVE;
pub use self::raylib_raw::KEY_SIX;
pub use self::raylib_raw::KEY_SEVEN;
pub use self::raylib_raw::KEY_EIGHT;
pub use self::raylib_raw::KEY_NINE;
pub use self::raylib_raw::KEY_A;
pub use self::raylib_raw::KEY_B;
pub use self::raylib_raw::KEY_C;
pub use self::raylib_raw::KEY_D;
pub use self::raylib_raw::KEY_E;
pub use self::raylib_raw::KEY_F;
pub use self::raylib_raw::KEY_G;
pub use self::raylib_raw::KEY_H;
pub use self::raylib_raw::KEY_I;
pub use self::raylib_raw::KEY_J;
pub use self::raylib_raw::KEY_K;
pub use self::raylib_raw::KEY_L;
pub use self::raylib_raw::KEY_M;
pub use self::raylib_raw::KEY_N;
pub use self::raylib_raw::KEY_O;
pub use self::raylib_raw::KEY_P;
pub use self::raylib_raw::KEY_Q;
pub use self::raylib_raw::KEY_R;
pub use self::raylib_raw::KEY_S;
pub use self::raylib_raw::KEY_T;
pub use self::raylib_raw::KEY_U;
pub use self::raylib_raw::KEY_V;
pub use self::raylib_raw::KEY_W;
pub use self::raylib_raw::KEY_X;
pub use self::raylib_raw::KEY_Y;
pub use self::raylib_raw::KEY_Z;
pub use self::raylib_raw::MOUSE_LEFT_BUTTON;
pub use self::raylib_raw::MOUSE_RIGHT_BUTTON;
pub use self::raylib_raw::MOUSE_MIDDLE_BUTTON;
pub use self::raylib_raw::MAX_TOUCH_POINTS;
pub use self::raylib_raw::GAMEPAD_PLAYER1;
pub use self::raylib_raw::GAMEPAD_PLAYER2;
pub use self::raylib_raw::GAMEPAD_PLAYER3;
pub use self::raylib_raw::GAMEPAD_PLAYER4;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_TRIANGLE;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_CIRCLE;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_CROSS;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_SQUARE;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_L1;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_R1;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_L2;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_R2;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_START;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_SELECT;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_UP;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_RIGHT;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_DOWN;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_LEFT;
pub use self::raylib_raw::GAMEPAD_PS3_BUTTON_PS;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_LEFT_X;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_LEFT_Y;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_RIGHT_X;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_RIGHT_Y;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_L2;
pub use self::raylib_raw::GAMEPAD_PS3_AXIS_R2;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_A;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_B;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_X;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_Y;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_LB;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_RB;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_SELECT;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_START;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_UP;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_RIGHT;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_DOWN;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_LEFT;
pub use self::raylib_raw::GAMEPAD_XBOX_BUTTON_HOME;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_LEFT_X;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_LEFT_Y;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_RIGHT_X;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_RIGHT_Y;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_LT;
pub use self::raylib_raw::GAMEPAD_XBOX_AXIS_RT;