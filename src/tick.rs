pub trait Tick {
    fn tick(&mut self);
}

pub trait TryTick {
    type Error;

    fn try_tick(&mut self) -> Result<(), Self::Error>;
}

pub trait TickWithResource<Resource> {
    fn tick_with_resource(&mut self, resource: Resource);
}

pub trait TryTickWithResource<Resource> {
    type Error;

    fn try_tick_with_resource(&mut self, resource: Resource) -> Result<(), Self::Error>;
}
