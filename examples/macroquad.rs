use macroquad::{prelude::*, rand, ui::root_ui};
use secs::World;

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
    width: f32,
    height: f32,
}

struct Score {
    value: i32,
}

enum Shape {
    Square,
    Circle,
}

fn move_system(world: &World, game_state: &mut GameState) {
    if game_state.paused {
        return;
    }

    world.query(|_entity, pos: &mut Position, vel: &mut Velocity| {
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
    });
}

fn collision_system(world: &World, _: &mut GameState) {
    world.query(
        |_, player_center: &Position, player: &mut Sprite, player_score: &mut Score| {
            world.query(|_, powerup_center: &Position, powerup: &mut Powerup| {
                if powerup.active
                    && (powerup_center.x - player_center.x).abs()
                        < (powerup.width * 0.5) + (player.width * 0.5)
                    && (powerup_center.y - player_center.y).abs()
                        < (powerup.height * 0.5) + (player.height * 0.5)
                {
                    powerup.active = false;

                    player.shape = match player.shape {
                        Shape::Square => Shape::Circle,
                        Shape::Circle => Shape::Square,
                    };

                    player_score.value += 1;
                    player.width += 3.;
                    player.height += 3.;
                }
            });
        },
    )
}

fn render_system(world: &World, game_state: &mut GameState) {
    if game_state.paused {
        let text = "PAUSED";
        let font_size = 100.;
        let text_width = measure_text(text, None, font_size as u16, 1.).width;
        let (x, y) = ((screen_width() - text_width) / 2., screen_height() / 2.);

        draw_text(text, x, y, font_size, RED);

        return;
    }

    world.query(|_, pos: &Position, sprite: &Sprite| match sprite.shape {
        Shape::Square => draw_rectangle(
            pos.x - (sprite.width * 0.5),
            pos.y - (sprite.width * 0.5),
            sprite.width,
            sprite.height,
            ORANGE,
        ),
        Shape::Circle => draw_circle(pos.x, pos.y, sprite.width * 0.5, PURPLE),
    });

    world.query(|_, powerup: &Powerup, pos: &Position| {
        if powerup.active {
            draw_rectangle(
                pos.x - (powerup.width * 0.5),
                pos.y - (powerup.width * 0.5),
                powerup.width,
                powerup.height,
                RED,
            );
        }
    });

    world.query(|_, score: &Score| {
        root_ui().label(None, &format!("Player Score: {}", score.value));
    });
}

#[macroquad::main("secs_macroquad")]
async fn main() {
    let world = World::default();

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

        world.spawn((
            Powerup {
                active: true,
                width: 15.,
                height: 15.,
            },
            Position { x, y },
        ));
    }

    let scheduler = secs::Scheduler::default();

    let mut game_state = GameState { paused: false };

    scheduler.register(move_system);
    scheduler.register(collision_system);

    scheduler.register(render_system);

    loop {
        clear_background(SKYBLUE);

        if is_key_pressed(KeyCode::P) {
            game_state.paused = !game_state.paused;
        }

        // run all parallel and sequential systems
        scheduler.run(&world, &mut game_state);

        next_frame().await;
    }
}
