use crate::world::{Entity, World};

use std::any::Any;

pub trait AttachComponents {
    fn attach_to_entity(self, world: &World, entity: Entity);
}

impl<C1: Any> AttachComponents for (C1,) {
    #[track_caller]
    fn attach_to_entity(self, world: &World, entity: Entity) {
        world.attach_component(entity, self.0);
    }
}

impl<C1: Any, C2: Any> AttachComponents for (C1, C2) {
    #[track_caller]
    fn attach_to_entity(self, world: &World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
    }
}

impl<C1: Any, C2: Any, C3: Any> AttachComponents for (C1, C2, C3) {
    #[track_caller]
    fn attach_to_entity(self, world: &World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
    }
}

impl<C1: Any, C2: Any, C3: Any, C4: Any> AttachComponents for (C1, C2, C3, C4) {
    #[track_caller]
    fn attach_to_entity(self, world: &World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
    }
}

impl<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any> AttachComponents for (C1, C2, C3, C4, C5) {
    #[track_caller]
    fn attach_to_entity(self, world: &World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
        world.attach_component(entity, self.4);
    }
}
