#include <iostream>
#include <raylib.h>

#include <exception>
#include <algorithm>
#include <vector>
#include <map>
#include <unordered_map>
#include <optional>
#include <memory>
#include <lua.h>


using namespace std;

using id_type=size_t;

#include "storage.hpp"

const float FRAME_RATE=60.0f;
const float FRAME_TIME=1.0f/60.0f;


std::unordered_map<std::string, std::shared_ptr<Texture2D>> texture_cache;

#define BSET(factory, x,type) \
    factory& x(type val) { \
        thing.x = val; \
        return *this; \
    }

#define BHDR(type) \
    type thing; \
    auto build() {\
        return thing; \
    }


std::shared_ptr<Texture2D> get_texture(char const * fname){
    auto where = texture_cache.find(fname);
    if (where != texture_cache.end() ){
        return where->second;
    }


    std::string str = fname;
    auto ptr = std::make_shared<Texture2D>();
    *ptr = LoadTexture(fname);
    if( ! ptr->height ){
        throw std::runtime_error("0 height texture");
    }

    texture_cache.emplace(str, ptr);

    return ptr;
}

enum class ShapeType{
    INVALID_SHAPE=0,
    CIRCLE,
    RECTANGLE,
};

vector<bool> operator&(const vector<bool> &lhs, const vector<bool> &rhs){
    size_t small = std::min(lhs.size(), rhs.size());
    size_t large = std::max(lhs.size(), rhs.size());

    vector<bool> ret;

    for(size_t i=0; i< small; i++){
        if( lhs[i] && rhs[i] ) {
            ret.push_back(true);
        } else {
            ret.push_back(false);
        }
    }

    for (size_t i = small; i < large; i++){
        ret.push_back(false);
    }
    return ret;
}

vector<bool> operator|(const vector<bool> &lhs, const vector<bool> &rhs){
    size_t small = std::min(lhs.size(), rhs.size());
    size_t large = std::max(lhs.size(), rhs.size());

    vector<bool> ret;

    for(size_t i=0; i< small; i++){
        if( lhs[i] || rhs[i] ) {
            ret.push_back(true);
        } else {
            ret.push_back(false);
        }
    }

    for (size_t i = small; i < large; i++){
        bool in_rhs = rhs.size() > i && rhs[i];
        bool in_lhs = lhs.size() > i && lhs[i];
        ret.push_back( in_rhs | in_lhs);
    }
    return ret;
}



struct Physical{
    float x,y;
    float xvel, yvel;
};

struct PhysicalBuilder{
    BHDR(Physical);

    BSET(PhysicalBuilder, x, float);
    BSET(PhysicalBuilder, y, float);
    BSET(PhysicalBuilder, xvel, float);
    BSET(PhysicalBuilder, yvel, float);
};

struct Drawable{
    std::shared_ptr<Texture2D> texture;
    float radius;
    float layer;

    Drawable(){
        radius = 0.1;
        layer = 0.2;
        texture = get_texture("no-texture.png");
    }
};

struct DrawableBuilder{
    Drawable thing;
    auto build(){
        return thing;
    }
    BSET(DrawableBuilder, radius, float);
    BSET(DrawableBuilder, layer, float);

    DrawableBuilder& texture_name( std::string val){
        thing.texture = get_texture(val.c_str());
        if( ! thing.texture ){
            throw std::runtime_error("Error setting the texture");
        }
        return *this;
    }
};

struct Shape {
    struct Circle {
        float radius;
    };
    struct Rectangle{
        float height, width;
    };

    ShapeType type;
    union {
        Circle circle;
        Rectangle rectangle;
    };
};

struct Collidable{
    Shape shape;
};

struct PlayerControl{};
struct DespawnFarRight{};
struct DespawnFarLeft{};

struct Bullet{
    float damage;
};

struct Powerup {
    float regen_increase=0;
    float fire_rate_increase=0;
};

struct PlayerStats {
    float movement_speed=15.0f;

    vector<Powerup> owned;
};

struct Weapon {
    float fire_rate;
    float fire_velocity;
    int gun_cooldown_frames = 0;
    Bullet to_spawn;

    Drawable drawable;

    bool to_right=true;
    float offset = 40;
};


struct WeaponBuilder {
    Weapon thing;

    BSET(WeaponBuilder, fire_rate, float);
    BSET(WeaponBuilder, offset, float);
    BSET(WeaponBuilder, fire_velocity, float);
    BSET(WeaponBuilder, to_right, bool);
    BSET(WeaponBuilder, gun_cooldown_frames, int);
    BSET(WeaponBuilder, drawable, Drawable);
    BSET(WeaponBuilder, to_spawn, Bullet);

    Weapon build(){
        return thing;
    }
};

struct Shield {
    float max_shield;
    float ammount;
    float regen;
};

struct ShieldBuilder {
    Shield thing{};

    ShieldBuilder & ammount(float val){
        if( thing.max_shield < val && thing.max_shield == 0.0f ) {
            thing.max_shield = val;
        }
        thing.ammount = val;
        return *this;
    }

    BSET(ShieldBuilder, regen, float);
    BSET(ShieldBuilder, max_shield, float);

    Shield build(){
        return thing;
    }
};

struct AutoFire{};

struct GameLevel{
    VectorStorage<Drawable> drawable_list;
    VectorStorage<Physical> physical_list;
    VectorStorage<Collidable> collidable_list;
    NullStorage<PlayerControl> controllable_list;
    HashStorage<Bullet> bullet_list;
    HashStorage<Shield> shield_list;
    NullStorage<DespawnFarLeft> despawn_left;
    NullStorage<DespawnFarRight> despawn_right;
    HashStorage<Powerup> powerup_list;
    HashStorage<PlayerStats> player_stats_list;
    HashStorage<Weapon> weapon_list;
    HashStorage<AutoFire> auto_fire_list;

    void destroy(id_type id){
        auto helper = [&](auto &x){x.remove(id);};

        helper(drawable_list);
        helper(physical_list);
        helper(collidable_list);
        helper(bullet_list);
        helper(despawn_left);
        helper(despawn_right);
        helper(powerup_list);
        helper(player_stats_list);
        helper(weapon_list);
        helper(auto_fire_list);

        free_id(id);
    }

    vector<id_type> unused_ids;
    size_t max_id = 0;

    id_type player_id;

    id_type alloc_id(){
        id_type out;
        if ( unused_ids.size() ) {
            id_type val = unused_ids.back();
            unused_ids.pop_back();
            out = val;
        } else {
            out = max_id ++;
        }

        return out;
    }

    void free_id(id_type id){
        unused_ids.push_back(id);
    }

    size_t frame_count =0;
    void step(){
        frame_count++;

        do_player_input();
        do_movement();
        do_despawn();
        do_collision();
        do_death_check();
        do_shield_regen();
        do_weapon_cooldown();
        do_weapon_fire();

        do_destroy();
    }

    vector<id_type> to_destroy;

    void do_weapon_cooldown() {
        auto mask = weapon_list.mask;
        for(size_t id=0; id<max_id; id++) {
            if( !mask[id] ) continue;

            auto &weapon = weapon_list.get(id);

            if( weapon -> gun_cooldown_frames > 0 ) {
                weapon->gun_cooldown_frames --;
            }
        }
    }

    void do_weapon_fire() {
        auto mask = weapon_list.mask & auto_fire_list.mask & physical_list.mask;
        size_t high_id = max_id;
        for(size_t id=0; id<high_id; id++) {
            if( high_id < id || !mask[id] ) continue;

            auto &weapon1 = weapon_list.get(id);

            if( weapon1 -> gun_cooldown_frames <= 0 ) {
                weapon1->gun_cooldown_frames += weapon1->fire_rate;

                id_type bul_id = spawn_bullet();

                auto &weapon2 = weapon_list.get(id);
                auto & bul_phy = physical_list.get(bul_id);
                auto &phy1 = physical_list.get(id);

                drawable_list.add(bul_id, weapon2->drawable);

                int dir = (weapon2->to_right * 2) - 1;

                bul_phy->xvel *= dir * weapon2->fire_velocity;

                bul_phy->x = phy1->x + weapon2->offset * dir;
                bul_phy->y = phy1->y;
            }
        }
    }

    void verify_draw(){
        auto mask = drawable_list.mask & physical_list.mask;
        for(size_t id=0; id<max_id; id++) {
            if(! mask[id] ) continue;

            if( ! drawable_list.get(id) ) {
                throw std::runtime_error("drawable should not be none");
            }

            if( ! drawable_list.get(id)->texture ) {
                throw std::runtime_error("drawable's texture should not be null");
            }
        }
    }

    void draw(){
        verify_draw();
        auto mask = drawable_list.mask & physical_list.mask;
        for(id_type id=0; id<max_id; id++) {
            if( ! mask[id] ) continue;
            auto &drw = drawable_list.get(id);
            auto &pos = physical_list.get(id);

            auto txt = drw->texture;
            if(!txt) {
                throw std::runtime_error("txt should not be null");
            }
            Rectangle src_rect = {0, 0, txt->width, txt->height};
            Rectangle dst_rect = Rectangle {
                (int)(pos->x + txt->width / 2),
                (int)(pos->y - txt->height / 2),
                (int)(txt->width),
                (int)(txt->height)
            };
            DrawTexturePro( *drw->texture, src_rect, dst_rect, Vector2{0.0, 0.0}, 0, RAYWHITE);
        }
    }

    id_type spawn_bullet(){
        id_type id = alloc_id();

        physical_list.add(id, Physical{ 0, 0, 0.25, 0 } );
        collidable_list.add(id, Collidable{ Shape{ ShapeType::CIRCLE, Shape::Circle{ 8.0f } } });
        despawn_right.add(id, DespawnFarRight{});
        despawn_left.add(id, DespawnFarLeft{});
        bullet_list.add(id, Bullet{10});

        return id;
    }

    bool is_colliding(id_type a, id_type b){
        auto a_phy = *physical_list.get(a);
        auto b_phy = *physical_list.get(b);

        auto a_pos = Vector2{ a_phy.x, a_phy.y };
        auto b_pos = Vector2{ b_phy.x, b_phy.y };

        auto a_col = *collidable_list.get(a);
        auto b_col = *collidable_list.get(b);

        auto col_to_rect = [&] (id_type c){
                auto c_phy = *physical_list.get(c);
                auto c_col = *collidable_list.get(c);
                auto c_pos = Vector2{ c_phy.x, c_phy.y };
                if (c_col.shape.type == ShapeType::RECTANGLE ){
                    return Rectangle {
                        (int)(c_pos.x - c_col.shape.rectangle.width / 2.0),
                        (int)(c_pos.y - c_col.shape.rectangle.height / 2.0),
                        (int)(c_pos.x + c_col.shape.rectangle.width / 2.0),
                        (int)(c_pos.y + c_col.shape.rectangle.height / 2.0)
                    };
                } else {
                    throw std::runtime_error("The shape was not a Rectangle");
                }
        };

        if( a_col.shape.type == ShapeType::CIRCLE &&
                    b_col.shape.type == ShapeType::CIRCLE ){
            return CheckCollisionCircles(a_pos, a_col.shape.circle.radius,
                                  b_pos, b_col.shape.circle.radius);
        } else if ( a_col.shape.type == ShapeType::CIRCLE &&
                    b_col.shape.type == ShapeType::RECTANGLE) {
            auto b_rect = col_to_rect(b);
            return CheckCollisionCircleRec(a_pos, a_col.shape.circle.radius, b_rect);
        } else if ( a_col.shape.type == ShapeType::RECTANGLE &&
                    b_col.shape.type == ShapeType::CIRCLE ) {
            auto a_rect = col_to_rect(b);
            return CheckCollisionCircleRec(b_pos, b_col.shape.circle.radius, a_rect);
        } else if ( a_col.shape.type == ShapeType::RECTANGLE &&
                    b_col.shape.type == ShapeType::RECTANGLE) {
            auto a_rect = col_to_rect(b);
            auto b_rect = col_to_rect(b);

            return CheckCollisionRecs(a_rect, b_rect);
        } else {
            throw std::runtime_error("could not find shape types to match");
        }
    }

    void handle_collision(id_type a, id_type b) {
        if( controllable_list.contains(a) && powerup_list.contains(b) ){
            auto &stats = player_stats_list.get(a);
            auto &power = powerup_list.get(a);

            stats->movement_speed *= 0.95;
            stats->owned.push_back(*power);

            to_destroy.push_back(b);
        }

        if( shield_list.contains(a) && bullet_list.contains(b) ) {
            auto &shield = shield_list.get(a);
            auto &bullet = bullet_list.get(b);

            shield->ammount -= bullet->damage;

            to_destroy.push_back(b);
        }

        do_destroy();
    }


    void do_death_check(){
        auto mask = shield_list.mask;

        for( id_type id = 0; id != max_id; id ++){
            if(!mask[id]) continue;

            auto shield = shield_list.get(id);
            if( shield->ammount < 0 ){
                to_destroy.push_back(id);
            }
        }

        do_destroy();
    }

    void do_destroy(){
        for(auto id: to_destroy){
            destroy(id);
        }
        to_destroy.clear();
    }

    void do_auto_fire(){
        auto mask = shield_list.mask;

        for( id_type id = 0; id != max_id; id ++){
            if(!mask[id]) continue;

            auto shield = shield_list.get(id);
            shield->ammount +=  shield->regen * FRAME_TIME;
        }
    }

    void do_shield_regen(){
        auto mask = shield_list.mask;

        for( id_type id = 0; id != max_id; id ++){
            if(!mask[id]) continue;

            auto shield = shield_list.get(id);
            shield->ammount +=  shield->regen * FRAME_TIME;
            shield->ammount = std::min(shield->max_shield, shield->ammount);
        }
    }

    //do the dumb n squared algorithm since I expect n to be small
    void do_collision(){
        auto mask = (physical_list.mask & collidable_list.mask);

        vector<id_type> to_check;
        for( id_type id = 0; id != max_id; id ++){
            if(!mask[id]) continue;

            to_check.push_back(id);
        }

        for( id_type outer_id = 0; outer_id != to_check.size(); outer_id ++){
            for(id_type inner_id = outer_id + 1 ; inner_id < to_check.size(); inner_id ++){
                if( is_colliding(outer_id, inner_id) ){
                    handle_collision(outer_id, inner_id);
                    handle_collision(inner_id, outer_id);
                }
            }
        }
    }

    void do_despawn(){
        auto mask = (despawn_left.mask | despawn_right.mask) & physical_list.mask;

        for(size_t i=0; i<max_id; i++){
            if( !mask[i] ) continue;

            auto &phy = physical_list.get(i);
            if( despawn_left.contains(i) && phy->x < 20.0f ) {
                to_destroy.push_back(i);
            } 
            if( despawn_right.contains(i) && phy->x > GetScreenWidth() + 120  ) {
                to_destroy.push_back(i);
            }
        }
        do_destroy();
    }

    void do_player_input(){
        auto in_mask = controllable_list.mask & physical_list.mask & player_stats_list.mask;

        for( size_t id=0; id < max_id; id++){
            if (!in_mask[id]) continue;


            float player_speed = player_stats_list.get(id)->movement_speed;

            if (IsKeyDown(KEY_W)) {
                auto &phy = physical_list.get(id);
                phy->y -= player_speed;
            }
            if (IsKeyDown(KEY_S)) {
                auto &phy = physical_list.get(id);
                phy->y += player_speed;
            }

            if (IsKeyPressed(KEY_SPACE)) {
                auto_fire_list.add(id, AutoFire{});
            }

            if (IsKeyReleased(KEY_SPACE)) {
                auto_fire_list.remove(id);
            }
        }
    }

    void do_movement(){
        for( auto &phy_opt : physical_list ){
            if ( phy_opt ){
                auto &phy = *phy_opt;
                phy.x += phy.xvel * FRAME_RATE;
                phy.y += phy.yvel * FRAME_RATE;
            }
        }
    }
};

int main(){
    InitWindow(1200, 1000, "Dodgem");
    SetTargetFPS(FRAME_RATE);

    GameLevel gl;

    id_type id = gl.alloc_id();

    //player
    gl.drawable_list.add(id, DrawableBuilder{}
                         .texture_name("player.png")
                         .layer(1.0)
                         .build());
    gl.physical_list.add(id, Physical{ 100, 200, 0, 0 } );
    gl.controllable_list.add(id, PlayerControl{});
    gl.collidable_list.add(id, Collidable{ Shape{ ShapeType::CIRCLE, Shape::Circle{ 40.0f } } });
    gl.player_stats_list.add(id, PlayerStats{});
    gl.weapon_list.add(id, WeaponBuilder{}
                       .fire_rate(0.75 * FRAME_RATE)
                       .offset(80)
                       .to_right(true)
                       .to_spawn(Bullet{10})
                       .gun_cooldown_frames(10)
                       .fire_velocity(3)
                       .drawable(DrawableBuilder{}
                                 .texture_name("red_ball.png")
                                 .layer(1.0)
                                 .build())
                       .build());
    gl.shield_list.add(id, ShieldBuilder{}
                  .ammount(30)
                  .regen(0.001)
                  .build());

    //*
    //powerup
    id = gl.alloc_id();
    gl.drawable_list.add(id, DrawableBuilder{}
                         .texture_name("fire_rate_up.png")
                         .layer(1.0f)
                         .build());
    gl.physical_list.add(id, Physical{ 800, 200, -0.1, 0 } );
    gl.collidable_list.add(id, Collidable{ Shape{ ShapeType::CIRCLE, Shape::Circle{ 20.0f } } });
    gl.despawn_left.add(id, DespawnFarLeft{});
    gl.powerup_list.add(id, Powerup{1.05, 0.0});

    //enemy
    id = gl.alloc_id();
    gl.drawable_list.add(id, DrawableBuilder{}
                         .texture_name("enemy1.png")
                         .layer(1.0f)
                         .build());
    gl.physical_list.add(id, Physical{ 1000, 500, -0.01, 0 } );
    gl.collidable_list.add(id, Collidable{ Shape{ ShapeType::CIRCLE, Shape::Circle{ 40.0f } } });
    gl.despawn_left.add(id, DespawnFarLeft{});
    gl.shield_list.add(id, ShieldBuilder{}
                       .ammount(10)
                       .regen(0)
                       .build());
    gl.weapon_list.add(id, WeaponBuilder{}
                       .fire_rate(0.25*FRAME_RATE)
                       .fire_velocity(0.4)
                       .to_right(false)
                       .offset(80)
                       .gun_cooldown_frames(1)
                       .drawable(DrawableBuilder{}
                                 .texture_name("red_ball.png")
                                 .layer(1.0)
                                 .build())
                       .build());
    gl.auto_fire_list.add(id, AutoFire{});
    // */

    while(! WindowShouldClose()){
        gl.step();

        ClearBackground(RAYWHITE);
        BeginDrawing();
        DrawFPS(3,3);
        gl.draw();
        EndDrawing();
    }
    return 0;
}
