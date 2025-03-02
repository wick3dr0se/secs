use macroquad::{prelude::*, rand, ui::root_ui};
use secs::prelude::{ExecutionMode, World};

struct GameState {
    paused: bool,
}

struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

struct Sprite {
    shape: Shape,
    width: f32,
    height: f32,
}

struct Powerup {
    active: bool,
}

struct Score {
    value: i32,
}

enum Shape {
    Square,
    Circle,
}

fn move_system(world: &World) {
    if let Some(game_state) = world.get_resource::<GameState>() {
        if game_state.paused {
            return;
        }
    }

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
    for (_, (player_pos, player_spr, player_score)) in
        world.query::<(&Position, &mut Sprite, &mut Score)>()
    {
        for (_, (powerup_pos, powerup)) in world.query::<(&Position, &mut Powerup)>() {
            if powerup.active
                && (powerup_pos.x - player_pos.x).abs() < player_spr.width
                && (powerup_pos.y - player_pos.y).abs() < player_spr.height
            {
                powerup.active = false;

                player_spr.shape = match player_spr.shape {
                    Shape::Square => Shape::Circle,
                    Shape::Circle => Shape::Square,
                };

                player_score.value += 1;
                player_spr.width += 3.;
                player_spr.height += 3.;
            }
        }
    }
}

fn render_system(world: &World) {
    if let Some(game_state) = world.get_resource::<GameState>() {
        if game_state.paused {
            let text = "PAUSED";
            let font_size = 100.;
            let text_width = measure_text(text, None, font_size as u16, 1.).width;
            let (x, y) = ((screen_width() - text_width) / 2., screen_height() / 2.);

            draw_text(text, x, y, font_size, RED);

            return;
        }
    }

    for (_, (pos, sprite)) in world.query::<(&Position, &Sprite)>() {
        match sprite.shape {
            Shape::Square => draw_rectangle(pos.x, pos.y, sprite.width, sprite.height, ORANGE),
            Shape::Circle => draw_circle(
                pos.x + sprite.width / 2.,
                pos.y + sprite.height / 2.,
                sprite.width / 2.,
                PURPLE,
            ),
        }
    }

    for (_, (powerup, pos)) in world.query::<(&Powerup, &Position)>() {
        if powerup.active {
            draw_rectangle(pos.x, pos.y, 15., 15., RED);
        }
    }

    for (_, (score,)) in world.query::<(&Score,)>() {
        root_ui().label(None, &format!("Player Score: {}", score.value));
    }
}

#[macroquad::main("secs_macroquad")]
async fn main() {
    let mut world = World::default();

    world.spawn((
        Position { x: 100., y: 100. },
        Velocity { x: 0., y: 0. },
        Sprite {
            shape: Shape::Circle,
            width: 20.,
            height: 20.,
        },
        Score { value: 0 },
    ));

    for _ in 0..50 {
        let x = rand::gen_range(0., screen_width());
        let y = rand::gen_range(0., screen_height());

        world.spawn((Powerup { active: true }, Position { x, y }));
    }

    world.add_resource(GameState { paused: false });

    // macroquad is single threaded so any systems executing its code cannot be run in parallel
    world.add_system(move_system, ExecutionMode::Serial);
    #[cfg(feature = "multithreaded")]
    {
        world.add_system(collision_system, ExecutionMode::Parallel);
    }
    #[cfg(not(feature = "multithreaded"))]
    {
        world.add_system(collision_system, ExecutionMode::Serial);
    }
    world.add_system(render_system, ExecutionMode::Serial);

    loop {
        clear_background(SKYBLUE);

        if is_key_pressed(KeyCode::P) {
            if let Some(game_state) = world.get_resource_mut::<GameState>() {
                game_state.paused = !game_state.paused;
            }
        }

        // run all parallel and sequential systems
        world.run_systems();

        next_frame().await;
    }
}
