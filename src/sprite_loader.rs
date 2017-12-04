extern crate cached;

#[allow(unused)]
#[allow(bad_style)]
use raylib::*;

use std::sync::Arc;


cached!{ LOAD_TEXTURE_CACHE >>
fn load_texture(fname: String) -> Option<Arc<Texture2D>> = {
        let opt = LoadTexture(fname.as_str() );
        return Some(Arc::new(opt));
    }
}

cached!{ LOAD_SOUND_CACHE >>
fn load_sound(fname: String) -> Option<Arc<Sound>> = {
        let opt = LoadSound(fname.as_str() );
        return Some(Arc::new(opt));
    }
}
