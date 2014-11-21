extern crate events;

use any::Any;

pub trait Component<'a> {
    fn initialize(&mut self, &'a mut Cog);
    fn handle_event(&mut self, &events::Event);
    fn close(&mut self);
}

pub struct Cog<'a> {
    pub children: Vec<Box<Cog<'a>>>,
    pub components: Vec<Box<Any>>,
}
