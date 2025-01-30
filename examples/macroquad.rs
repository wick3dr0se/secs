use macroquad::{prelude::*, rand, ui::root_ui};
use secs::prelude::{ExecutionMode, World};

struct Position {
    x: f32,
    y: f32
}

struct Velocity {
    x: f32,
    y: f32
}

struct Sprite {
    shape: Shape
}

struct Powerup {
    active: bool
}

struct Score {
    value: i32
}

enum Shape {
    Square,
    Circle
}

fn move_system(world: &World) {
    for (_entity, (pos, vel)) in world.query::<(&mut Position, &mut Velocity)>() {
        vel.x = 0.;
        vel.y = 0.;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            vel.x = 2.;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            vel.x = -2.;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            vel.y = 2.;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            vel.y = -2.;
        }

        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn collision_system(world: &World) {
    for (_entity, (player_pos, player_sprite, player_score)) in world.query::<(&Position, &mut Sprite, &mut Score)>() {
        for (_entity, (powerup_pos, powerup)) in world.query::<(&Position, &mut Powerup)>() {
            if powerup.active && (powerup_pos.x - player_pos.x).abs() < 20. && (powerup_pos.y - player_pos.y).abs() < 20. {
                powerup.active = false;

                player_sprite.shape = match player_sprite.shape {
                    Shape::Square => Shape::Circle,
                    Shape::Circle => Shape::Square,
                };

                player_score.value += 1;                
            }
        }
    }
}

fn render_system(world: &World) {
    for (_, (pos, sprite)) in world.query::<(&Position, &Sprite)>() {
        match sprite.shape {
            Shape::Square => draw_rectangle(pos.x, pos.y, 20., 20., ORANGE),
            Shape::Circle => draw_circle(pos.x + 10., pos.y + 10., 10., PURPLE),
        }
    }

    for (_, (powerup, pos)) in world.query::<(&Powerup, &Position)>() {
        if powerup.active {
            draw_rectangle(pos.x, pos.y, 10., 10., RED);
        }
    }

    for (_, (score,)) in world.query::<(&Score,)>() {
        root_ui().label(None, &format!("Player Score: {}", score.value));
    }
}

#[macroquad::main("secs_macroquad")]
async fn main() {
    let mut world = World::default();

    let player = world.spawn();
    world.attach(player, Position { x: 100., y: 100. });
    world.attach(player, Velocity { x: 0., y: 0. });
    world.attach(player, Sprite { shape: Shape::Square });
    world.attach(player, Score { value: 0 });

    for _ in 0..25 {
        let powerup = world.spawn();
        let x = rand::gen_range(0., screen_width());
        let y = rand::gen_range(0., screen_height());

        world.attach(powerup, Position { x, y });
        world.attach(powerup, Powerup { active: true });
    }

    // macroquad is single threaded so any systems executng it's code cannot be run in parallel
    world.add_system(move_system, ExecutionMode::Serial);
    world.add_system(collision_system, ExecutionMode::Parallel);
    world.add_system(render_system, ExecutionMode::Serial);

    loop {
        clear_background(SKYBLUE);

        // run all parallel and sequential systems
        world.run_systems();

        next_frame().await;
    }
}