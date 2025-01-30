use secs::World;

struct Position {
    x: i32,
    y: i32
}

struct Velocity {
    x: i32,
    y: i32
}

fn move_system(world: &World) {
    // iterate over entities with these components
    for (_entity,(pos, vel)) in world.query::<(&mut Position, &mut Velocity)>() {
        vel.x = 1;
        vel.y = -1;
        pos.x += vel.x;
        pos.y += vel.y;
    
        println!("Entity position: ({}, {})", pos.x, pos.y);
    }
}

fn main() {
    let mut world = World::default();
    let entity = world.spawn();

    world.attach(entity, Position { x: 0, y: 0 });
    world.attach(entity, Velocity { x: 0, y: 0 });

    world.add_system(move_system);

    // run all systems sequentially
    world.run_systems();
    // run them again but in parallel
    world.run_systems_par();
}