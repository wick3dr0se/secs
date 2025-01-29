use secs::{World, Component};

struct Position {
    x: i32,
    y: i32
}

struct Velocity {
    x: i32,
    y: i32
}

impl Component for Position {}
impl Component for Velocity {}

fn main() {
    let mut world = World::default();
    let player = world.spawn();

    world.attach(player, Position { x: 0, y: 0 });
    world.attach(player, Velocity { x: 0, y: 0 });

    for (entity,(pos, vel)) in world.query::<(&mut Position, &mut Velocity)>() {
        if player == entity { // only entity in world currently
            vel.x = 1;
            vel.y = -1;
            pos.x += vel.x;
            pos.y += vel.y;
    
            println!("Player position: ({}, {})", pos.x, pos.y);
        }
    }
}