json = require "json"
weighted_rand = require "weighted_rand"


window_height = 760
window_width = 1300

DEGREE_TO_RAD = math.pi * 2 / 360.0
RAD_TO_DEGREE = 360.0 / math.pi * 2

function rng_range(low, high)
    diff = high - low
    return low + math.random() * diff
end

function gen_random_upgrade(x,y)
    lst = {
        gen_fire_rate_increase,
        gen_fire_damage_increase,
        gen_shield_increase,
        gen_regen_increase,
    }
    return lst[ math.random(1, #lst) ](x,y)
end
function gen_fire_rate_increase(x,y)
    return powerup_factory(x,
                           y,
                           "fire-rate-up",
                           {
                               fire_rate_increase = 0.95,
                               pickup_sound = Sound { file="powerup-pickup.wav" },
                           })

end
function gen_fire_damage_increase(x,y)
    return powerup_factory(x,
                           y,
                           "damage-up",
                           {
                               fire_damage_increase = 1.05,
                               pickup_sound = Sound { file="powerup-pickup.wav" },
                           })

end
function gen_shield_increase(x,y)
    return powerup_factory(x,
                           y,
                           "shield-up",
                           {
                               shield_increase = 1.05,
                               pickup_sound = Sound { file="powerup-pickup.wav" },
                           })

end
function gen_regen_increase(x,y)
    return powerup_factory(x,
                           y,
                           "shield-regen",
                           {
                               regen_increase = 1.05,
                               pickup_sound = Sound { file="powerup-pickup.wav" },
                           })
end

function gen_shot_increase(x,y)
    return powerup_factory(x,
                           y,
                           "shot-number-increase",
                           {
                               shot_increase = 1,
                               pickup_sound = Sound { file = "powerup-pickup.wav" }
                           })
end


function gen_enemy_a (x,y)
    return {
        drawable = {
            texture = Texture{file="enemy_a.png"},
            layer=1.0,
        },
        animation = get_animation_from_json("enemy_a.json"),
        physical = {
            x=x,
            y=y,
            xvel= rng_range(-250.0, -200.0)
        },
        auto_fire = {},
        collidable = {
            radius = 20.0,
        },
        despawn_far_left = {},
        death_event = {
            sound = Sound { file = "explosion001.wav" },
            score_add = 10
        },
        follow_player_y = {
            speed = 120.0,
            offset=rng_range(-50, 50),
        },
        shield = {
            ammount = 11.0,
        },
        team = { team = 1 },
        weapon = {
            fire_rate = rng_range(2.0, 3.0),
            fire_velocity = rng_range( -400.0, -350.0),
            prefab = Prefab{
                team = { team = 1 },
                despawn_far_left = {},
                bullet = {
                    damage = 10.0,
                },
                physical = {},
                collidable = {
                    radius = 4.0,
                },
                animation = get_animation_from_json("red_ball.json"),
                drawable = {
                    texture = Texture { file = "red_ball.png" },
                    layer = 1.0,
                },
                offset = -10.0,
                gun_cooldown_frames = 0,
            },
        },
    }
end

function gen_enemy_b(x,y)
    return {
        drawable = {
            texture = Texture{ file = "enemy_b.png" },
            layer = 1.0,
        },
        point_along_movement_vector = {
            angular_offset = 180,
        },
        physical = {
            x=x,
            y=y,
            xvel = rng_range(-200, -150),
        },
        auto_fire = {},
        collidable = { radius = 20.0 },
        despawn_far_left = {},
        death_event = {
            score_add = 20,
            sound = Sound { file = "explosion001.wav" },
        },
        shield = {
            ammount = 11.0,
        },
        sine_movement = {
            amplitude = rng_range(100.0, 200),
            frequency = rng_range(0.1, 0.4),
        },
        team = { team = 1, },
        weapon = {
            fire_rate = rng_range(1.7, 2.0),
            prefab = Prefab{
                team = { team = 1 },
                despawn_far_left = {},
                bullet = { damage = 10 },
                physical = {},
                collidable = { radius = 4.0 },
                sine_movement = {
                    amplitude = 30,
                    frequency = 2,
                },
                sine_movement = {
                    amplitude = 30,
                    frequency = 2.5,
                },
                drawable = {
                    texture = Texture{ file = "green-ball.png" },
                    layer = 1.0,
                },
                animation = get_animation_from_json("green-ball.json"),
            },
            fire_velocity = rng_range(-300, -280),
            offset = -10,
            gun_cooldown_frames = rng_range(2.0, 4.0),
        },
    }
end

function gen_enemy_c(x,y)
    return {
        drawable = {
            texture = Texture{ file = "enemy_c.png" },
            layer = 1,
        },
        physical = {
            x=x,
            y=y,
            xvel = rng_range(-100.5, -50),
        },
        stop_at = {
            xloc = rng_range(900, 1000),
        },
        auto_fire = {},
        collidable = { radius = 20 },
        despawn_far_left = {},
        death_event = {
            sound = Sound { file = "explosion001.wav" },
            score_add = 50,
        },
        shield = {
            ammount = 30,
        },
        sine_movement = {
            amplitude = rng_range(300.0, 320),
            frequency = rng_range(0.1, 1.0),
        },
        sine_movement_x = {
            amplitude = rng_range(50.0, 150),
            frequency = rng_range(0.1, 0.5),
        },
        clamp_y = {
            low = 0,
            high = 768,
        },
        team = { team = 1 },
        weapon = {
            pattern = 2,
            fire_angle = rng_range(90, 120),
            fire_rate = rng_range(1.0, 1.5),
            fire_velocity = rng_range(-300, -200),
            prefab = Prefab{
                team = { team = 1 },
                despawn_far_left = {},
                bullet = { damage = 2 },
                sine_movement = {
                    amplitude = 20,
                    frequency = 3,
                },
                physical = {},
                collidable = { radius = 4.0 },
                drawable = {
                    texture = Texture { file = "orange-ball.png" },
                    layer = 1,
                },
                offset = -10,
                gun_cooldown_frames = rng_range(1,3)
            }
        }
    }

end

function powerup_factory(x,y, base_name, powerup_stats)
    return {
        drawable = {
            texture = Texture{file=base_name .. ".png"},
            layer = 1,
        },
        physical = {
            x=x,
            y=y,
            xvel=-200,
        },
        collidable = { radius = 20 },
        despawn_far_left = {},
        powerup = powerup_stats,
        animation = get_animation_from_json(base_name .. ".json"),
    }
end

function get_shot_angles(angle, count)
    local ret = {}
    local delta = angle / (count+1)

    for i=0,count do
        table.insert(ret, delta * (i+1) - angle / 2.0)
    end
    return ret
end

function gen_boss_a_level(start_frame, difficulty, length)
    local null_cluster = gen_null_powerup_cluster()
    local shot_increase_spawner = Spawner()
    shot_increase_spawner:push( Prefab(gen_shot_increase(0,0)))

    local minion = {
        physical = {},
        team = { team = 1 },
        despawn_far_left = {},
        despawn_y = {},
        drawable = { texture = Texture{ file = "boss_a_minion.png" } },
        timeout_death = { time = 30 },
        collidable = { radius = 20.0 },
        sine_movement = {
            amplitude = 10,
            frequency = 1,
        },
        sine_movement_x = {
            amplitude = 20,
            frequency = 0.9,
        },
        shield = { ammount = 30 },
        bullet = { damage = 50 },
        death_event = {
            score_add = 10,
            sound = Sound{ file = "boss_a_minion_death.wav" },
            spawner = null_cluster,
        },
        physical = { angular_velocity = 30},
        weapon = {
            fire_angle = 360,
            fire_rate = 3,
            fire_velocity = -270,
            gun_cooldown_frames = 3,
            pattern = 8,
            prefab = Prefab{
                physical = {},
                collidable = { radius = 4 },
                shield = { amount = 1 },
                timeout_death = { time = 10 },
                bullet = { damage = 8 },
                despawn_far_left = {},
                despawn_far_right = {},
                despawn_y = {},
                team = { team = 1 },
                drawable = {
                    texture = Texture { file = "red_ball.png" },
                    layer = 1,
                },
                animation = get_animation_from_json("red_ball.json"),
            },
        },
        auto_fire = {},
    }

    local boss =  {
        auto_fire = {},
        drawable = {
            texture = Texture{ file = "boss_a.png" },
        },
        animation = get_animation_from_json("boss_a.json"),
        physical = {
            x= 1400,
            y=400,
            xvel=-100,
        },
        stop_at = { xloc = 1000 },
        sine_movement = {
            amplitude = 200,
            frequency = 0.2,
        },
        sine_movement_x = {
            amplitude = 100,
            frequency = 0.05,
        },
        team = { team = 1 },
        death_event = {
            score_add = 1000,
            spawner = shot_increase_spawner,
            clear_spawn_plan = true,
        },
        weapon = {
            fire_rate = 6,
            fire_angle = 120,
            fire_velocity = -60,
            gun_cooldown_frames = 4,
            fire_sound = Sound{file="boss_a_shot.wav"},
            prefab = Prefab(minion),
            pattern = 3,
            offset = -20,
        },
        collidable = { radius = 60 },
        boss_health_draw = {},
        shield = { ammount = 500 + difficulty },
    }

    local boss_spawner = Spawner()
    boss_spawner:push(Prefab(boss))
    local plan = SpawnPlan()

    plan:add(start_frame + 1, boss_spawner)
    plan:add(start_frame + 10000000, Spawner())

    return plan
end

function gen_boss_b_level(start_frame, difficulty, length)
    local missile_builder = {
        physical = {},
        team = { team = 1 },
        follow_player_y = { speed = 100 },
        shield = { ammount = 30.0 },
        collidable = { radius = 4 },
        drawable = {
            texture = Texture{ file = "missile.png" },
            layer = 1,
        },
        despawn_far_left = {},
        bullet = { damage = 12 },
        drag = { y = 0.9, },
        point_along_movement_vector = {
            angular_offset = 180,
        },
    }

    local missile_spawner = Spawner()
    for i=0,4 do
        for _, dir in pairs({-1,1}) do
            local xoff = 10 * i * dir
            local yoff = 10 * dir
            local xvel = 50 * i
            local yvel = i * 120 * dir
            local xacc = -400

            missile_builder['physical']['xoff'] = xoff
            missile_builder['physical']['yoff'] = yoff
            missile_builder['physical']['xvel'] = xvel
            missile_builder['physical']['yvel'] = yvel
            missile_builder['physical']['xacc'] = xacc

            missile_spawner:push(Prefab(missile_builder))
        end
    end

    local missile_spawner_prefab = {
        physical = {},
        timeout_death = {time=0.001},
        death_event = {
            spawner = missile_spawner,
        },
    }

    local shot_increase_spawner = Spawner()
    shot_increase_spawner:push( Prefab(gen_shot_increase(0,0)) )

    local boss = {
        drawable = {
            texture = Texture{ file="boss_b.png" },
        },
        animation = get_animation_from_json("boss_b.json"),
        physical = {
            x = 1400,
            y = 400,
            xvel = -100,
        },
        team = { team = 1 },
        sine_movement = {
            amplitude = 200, 
            frequency = 1,
        },
        follow_player_y = {},
        weapon = {
            fire_rate = 5,
            fire_velocity = 0,
            fire_sound = Sound { file = 'missile-launch.wav' },
            prefab = Prefab(missile_spawner_prefab),
        },
        auto_fire = {},
        stop_at = { xloc = 1100 },
        shield = { ammount = 800 },
        boss_health_draw = {},
        collidable = { radius = 50 },
        avoid_player_y = { speed = 80 },
        clamp_y = {
            low = 50,
            high = 700,
        },
        death_event = {
            sound = Sound{file="explosion001.wav"},
            spawner = shot_increase_spawner,
            clear_spawn_plan = true,
        },
    }


    local plan = SpawnPlan()
    plan:add_prefab(start_frame + 10, Prefab(boss))
    plan:add_prefab(start_frame + 10000000, Prefab{})

    return plan
end

--[[
function gen_boss_3_level(start_frame, difficulty, length)
end
--]]

function gen_enemy_e(x,y)
    local txt = Texture{ file="yellow-ball.png" }
    local function gen_sub(angle)
        angle = angle * DEGREE_TO_RAD
        local velocity = rng_range(100, 150)
        local submunition = {
            drawable = {
                texture = txt,
                layer = 1.0,
            },
            timeout_death = { time = 20 },
            collidable = { radius = 4.0 },
            despawn_far_left = {},
            despawn_far_right = {},
            despawn_y = {},
            team = { team = 1 },
            bullet = { damage = 3 },
            physical = {
                xvel = math.cos( angle ) * -1 * velocity,
                yvel = math.sin( angle ) * -1 * velocity,
            }
        }

        return submunition
    end

    local spawner = Spawner()
    for _,v in ipairs(get_shot_angles( rng_range(200, 360), rng_range(5,10) )) do
        spawner:push(Prefab(gen_sub(v)))
    end

    local bomb = Prefab{
        drawable = {
            texture = Texture { file = "bomb.png" },
            layer = 2,
        },
        physical = {},
        drag = {
            x = 0.8,
        },
        timeout_death = {
            time = 4,
        },
        death_event = {
            spawner = spawner,
            sound = Sound{file="bomb-explode.wav"},
        },
    }

    return {
        drawable = {
            texture = Texture{file="enemy_e.png"},
            layer= 1,
        },
        physical = { 
            x=x,
            y=y,
            xvel = rng_range(-80, -60),
        },
        auto_fire = {},
        shield = { ammount = 100 },
        sine_movement = {
            amplitude = 30.0,
            frequency = 0.1,
        },
        point_along_movement_vector = {
            angular_offset = 180,
        },
        weapon = {
            prefab = bomb,
            fire_rate = rng_range(4,5),
            fire_velocity = rng_range(-400, -300),
            gun_cooldown_frames = 5,
            offset = -40
        },
        collidable = { radius = 20.0 },
        death_event = {
            score_add = 20,
            sound = Sound{ file = "explosion002.wav" },
        },
        despawn_far_left = {},
        team = { team = 1 },
    }
end

function gen_null_powerup_cluster()
    local snd = Sound { file="bad-pickup.wav" }
    local spawner = Spawner()

    for _,i in pairs({-7, 0, 7}) do
        local null_powerup = powerup_factory(0,0, "null-powerup", { pickup_sound = snd })
        local base = null_powerup['physical']['xvel']
        null_powerup['collidable']['radius'] = 8
        null_powerup['physical']['yvel'] = math.sin(DEGREE_TO_RAD * i) * base
        null_powerup['physical']['xvel'] = math.cos(DEGREE_TO_RAD * i) * base
        spawner:push(Prefab(null_powerup))
    end
    return spawner
end

function gen_enemy_d(x,y)
    local spawner = gen_null_powerup_cluster()
    
    return {
        drawable = {
            texture = Texture { file = "enemy_d.png" },
            layer = 1,
        },
        physical = {
            x=x,
            y=y,
            xvel = rng_range(-150, -100),
        },
        collidable = { radius = 14.0 },
        despawn_far_left = {},
        shield = { ammount = 1 },
        sine_movement = {
            amplitude = rng_range(0.0, 40),
            frequency = rng_range(1.0, 3),
        },
        team = { team = 1 },
        death_event = {
            sound = Sound { file = "explosion-small.wav" },
            spawner = spawner,
            score_add = 5,
        },
    }
end

function gen_bomber_level(start_frame, difficulty, length)
    picker = weighted_rand:new()
    picker:add(5, {fun = gen_random_upgrade, cost = 2})
    picker:add(1, {fun = gen_enemy_a, cost = 10})
    picker:add(1, {fun = gen_enemy_b, cost = 20})
    picker:add(1, {fun = gen_enemy_c, cost = 40})
    picker:add(1, {fun = gen_enemy_d, cost = 80})
    picker:add(20, {fun = gen_enemy_e, cost = 80})
    return gen_level_from_weights(start_frame, difficulty, length, picker)
end

function gen_enemy1_level(start_frame, difficulty, length)
    picker = weighted_rand:new()
    picker:add(5, {fun = gen_random_upgrade, cost = 2})
    picker:add(20, {fun = gen_enemy_a, cost = 10/2})
    picker:add(5, {fun = gen_enemy_b, cost = 20})
    picker:add(1, {fun = gen_enemy_c, cost = 40})
    picker:add(1, {fun = gen_enemy_d, cost = 80})
        --{weight = 0, value = {fun = gen_enemy_e, cost = 40}},

    return gen_level_from_weights(start_frame, difficulty, length, picker)
end

function gen_enemy4_level(start_frame, difficulty, length)
    picker = weighted_rand:new()
    picker:add(5, {fun = gen_random_upgrade, cost = 2})
    picker:add(1, {fun = gen_enemy_a, cost = 40})
    picker:add(1, {fun = gen_enemy_b, cost = 20})
    picker:add(1, {fun = gen_enemy_c, cost = 10})
    picker:add(50, {fun = gen_enemy_d, cost = 1})
        --{weight = 0, value = {fun = gen_enemy_e, cost = 40}},

    return gen_level_from_weights(start_frame, difficulty, length, picker)
end

function text_floater(x,y,text)
    return {
        physical = {
            x=x,
            y=y,
            xvel=-100,
        },
        drawable = {
            texture = Texture {
                text = text,
            },
            layer = -1,
        },
        despawn_far_left = {
            at = -400,
        },
    }
end

function gen_level(start_frame, difficulty, length)
    difficulty = difficulty + 0
    print("difficulty: "..difficulty)

    if false then
        return gen_boss_a_level(start_frame, difficulty, length)
    end

    local plan =  gen_random_level(start_frame, difficulty, length)
    if start_frame == 0 then
        plan:add_prefab(0, Prefab(text_floater(1300, 100, "WASD to move")))
        plan:add_prefab(0, Prefab(text_floater(1300, 200, "Space to fire")))
        plan:add_prefab(0, Prefab(text_floater(1900, 100, "Pickups slow you down until installed")))
        plan:add_prefab(0, Prefab(text_floater(1900, 200, "Stop firing to install pickups")))
    end
    return plan
end

function gen_random_level(start_frame, difficulty, length)
    local insert = table.insert
    local picker = weighted_rand:new()
    picker:add(3, gen_enemy1_level)

    if difficulty > 80 then
        picker:add(3, gen_normal_level)
    end
    if difficulty > 100 then
        picker:add(3, gen_bomber_level)
    end

    if difficulty > 150 then
        picker:add(1, gen_enemy4_level)
    end

    local fun = picker:get()
    return fun(start_frame, difficulty, length)
end

function gen_normal_level(start_frame, difficulty, length)
    picker = weighted_rand:new()
    picker:add(5, {fun = gen_random_upgrade, cost = 2})
    picker:add(50, {fun = gen_enemy_a, cost = 10})
    picker:add(30, {fun = gen_enemy_b, cost = 20})
    picker:add(20, {fun = gen_enemy_c, cost = 40})
    picker:add(5, {fun = gen_enemy_d, cost = 50})

    if difficulty > 100.0 then
        picker:add(10, {fun = gen_enemy_e, cost = 120})
    end

    return gen_level_from_weights(start_frame, difficulty, length, picker)
end

function gen_level_from_weights(start_frame, difficulty, length, picker) 
    local plan = SpawnPlan()

    -- difficulty = difficulty * 2
    while difficulty > 0  do
        local x = rng_range(1400, 1500)
        local y = rng_range(0, window_height)
        local v = picker:get()
        local fun = v['fun']
        local cost = v['cost']

        difficulty = difficulty - cost
        local prefab = Prefab(fun(x,y))

        local rand = math.random()
        local offset = (rand * length)
        local frame = start_frame + offset
        if difficulty < 0 then
            frame = start_frame + length
        end
        local iframe = math.floor(frame)
        plan:add_prefab(iframe, prefab)
    end
    return plan
end

function respawn_player(frame, diff, len)
    local plan = SpawnPlan()
    local prefab  = Prefab(gen_player())
    plan:add_prefab(frame, prefab)
    return plan
end

function loud(thing)
    print("-------------------------------")
    print(thing)
    print("-------------------------------")
end

function get_animation_from_json(fname)
    local dat = get_json(fname)
    if dat == nil then return nil end

    local frames = #dat['frames']
    assert(frames > 0, "must have at least one frame")
    local times = {}
    for k,frame in ipairs(dat['frames']) do
        local dur = frame['duration'] / 1000
        table.insert(times, dur)
    end
    local ret = {
        frames = frames,
        times = times,
    }
    return ret
end

function get_json(fname)
    local file = io.open(fname, "r")
    if file == nil then return nil end

    local contents = file:read("a")
    return json.decode(contents)
end

function get_frames_from_json(fname)
    local file = io.open(fname, "r")
    local contents = file:read("a")
    local dat = json.decode(contents)
    local frames = #dat['frames']
    return frames
end

function gen_player ()
    player_speed = 400
    local frames = get_frames_from_json("player.json")
    return {
        --draw_collidable = {},
        install={},
        drawable={
            texture=Texture{ file="player.png" },
            layer=1.0
        },
        animation = get_animation_from_json("player.json"),
        physical = {
            x=400, 
            y=400,
        },
        player_control = {},
        player_stats = {
            movement_speed = player_speed,
            base_speed = player_speed,
            install_progress = 0,
            install_finish_sound = Sound { file="item-install.wav" },
        },
        shield = {
            regen = 1.0,
            ammount = 30.0,
            max_shield = 30.0,
        },
        clamp_y = {
            low = 0.0,
            high = window_height,
        },
        collidable = { radius = 30.0, },
        team = { team = 0, },
        weapon = {
            fire_angle = 60,
            fire_rate = 0.3,
            fire_velocity = 300.0,
            fire_sound = Sound { file="laser001.wav" },
            prefab = Prefab{
                physical = {angle=180},
                team = { team = 0 },
                collidable = { radius = 4.0, },
                despawn_far_right = {},
                drawable = {
                    texture = Texture{ file = "red_ball.png" },
                    layer = 1.0,
                },
                animation = get_animation_from_json("red_ball.json"),
                bullet = { damage = 10 },
            },
            offset = 40.0,
        }
    }
end

function gen_star_spawner(start_frame)
    local function fun()
        local dist = math.random()
        local color_base = 1.0 - 1.0 * ( dist * dist )
        local color_byte = color_base * 255.0

        local ltint = Color{ r=color_byte, g=color_byte, b=color_byte, a=color_byte}
        return {
            drawable = {
                layer = -4,
                texture = Texture{ file = "background_star.png" },
                tint = ltint,
            },
            despawn_far_left = {},
            physical = {
                x=1300,
                y=math.random()*window_height,
                xvel = ( -250.0 * (color_base * color_base) ),
            },
        }
    end

    local density = 2
    local to_spawn = 400

    local cur_frame = start_frame
    local plan = SpawnPlan()

    if start_frame == 0 then
        to_spawn = window_width / density
    end

    local spawner = Spawner()
    if start_frame == 0 then
        spawner:push(Prefab(gen_player()))
        plan:add(0, spawner)
    end

    while to_spawn >= 0 do
        step = math.random(0,math.tointeger(density))
        cur_frame = cur_frame + step
        to_spawn = to_spawn -1

        local spawner = Spawner()
        if start_frame == 0 then
            local star = fun()
            local phy = star['physical']
            phy['x'] = math.random(0, 1300)
            spawner:push(Prefab(star))
            plan:add(1, spawner)
        else
            spawner:push( Prefab(fun(math.random())))
            plan:add(cur_frame, spawner)
        end
    end

    return plan;
end
