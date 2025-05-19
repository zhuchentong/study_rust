pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vec of Box<dyn Draw>
    // Trait Objects
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
