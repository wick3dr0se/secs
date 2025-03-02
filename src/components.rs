use crate::world::{Entity, World};

pub trait AttachComponents {
    fn attach_to_entity(self, world: &mut World, entity: Entity);
}

impl<C1: 'static + Send + Sync> AttachComponents for (C1,) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
    }
}

impl<C1: 'static + Send + Sync, C2: 'static + Send + Sync> AttachComponents for (C1, C2) {
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
    }
}

impl<C1: 'static + Send + Sync, C2: 'static + Send + Sync, C3: 'static + Send + Sync>
    AttachComponents for (C1, C2, C3)
{
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
    }
}

impl<
        C1: 'static + Send + Sync,
        C2: 'static + Send + Sync,
        C3: 'static + Send + Sync,
        C4: 'static + Send + Sync,
    > AttachComponents for (C1, C2, C3, C4)
{
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
    }
}

impl<
        C1: 'static + Send + Sync,
        C2: 'static + Send + Sync,
        C3: 'static + Send + Sync,
        C4: 'static + Send + Sync,
        C5: 'static + Send + Sync,
    > AttachComponents for (C1, C2, C3, C4, C5)
{
    fn attach_to_entity(self, world: &mut World, entity: Entity) {
        world.attach_component(entity, self.0);
        world.attach_component(entity, self.1);
        world.attach_component(entity, self.2);
        world.attach_component(entity, self.3);
        world.attach_component(entity, self.4);
    }
}
