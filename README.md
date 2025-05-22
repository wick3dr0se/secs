# secs - Shit Entity Component System
**secs** is an ECS for people who want something with the bare minimum. No complex features, no unnecessary abstractions — just the essentials, no strings attached (but you could do that)

If you’re tired of ECS libraries that make you feel like you’re writing a thesis on systems design, **secs** is here to give you what you need and nothing more

## Why secs?
You could use something like **hecs**, **specs** or **bevy** for your ECS needs, but why chase frills when you could have uncomplicated **secs**? It's simple, lightweight and may (or may not) get the job done. No promises. If you need more, you can always try something else

***secs** — without the fluff*

## Features
- **Entity Management**: Entities are ID's, right?
- **Component Storage**: Components are stored in sparse sets — sounds fancy but archetypes were too much work
- **Multiple Mutable Queries**: You can mutate many components, probably..
- **Scheduling**: **secs** has a minimal scheduler that stays out of your way. Need more control? Run systems manually.
- **Resources**: Resources can easily be passed around to any (scheduled) system

## Getting Started
Get **secs**

```bash
cargo add secs
```

Example: How it’s probably supposed to work
```rust
use secs::World;

let mut world = World::default();
world.spawn((Component1 { /* your data */ }, Component2));

world.query(|entity, c1: &Component1, c2: &mut Component2| {
    // maybe get mixed mutability components
})
```

See more examples in [examples/](examples/)

## Contributing
Want to make **secs** less shitty? Contributions are much appreciated
