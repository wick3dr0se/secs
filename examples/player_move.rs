use secs::{World, Component};

struct Position {
    x: i32,
    y: i32
}

impl Component for Position {}

fn main() {
    let mut world = World::default();
    let player = world.spawn();

    world.attach(player, Position { x: 0, y: 0 });

    for (_entity, (pos,)) in world.query_mut::<(&mut Position,)>() {
        pos.x += 1;
        pos.y += 1;
        
        println!("Player position: ({}, {})", pos.x, pos.y);
    }
}