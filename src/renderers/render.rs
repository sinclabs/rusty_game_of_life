use crate::universe::Universe;

pub trait Render {
    fn render(&mut self, universe: &mut Universe);
}
