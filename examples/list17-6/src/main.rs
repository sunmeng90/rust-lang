fn main() {}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    // This restricts to a list of components all of type Button or all of type TextField
    pub components: Vec<T>, 
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
