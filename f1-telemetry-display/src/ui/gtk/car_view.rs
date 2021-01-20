use crate::ui;
use gtk::{Orientation, Align, BoxExt};

pub struct CarView {
    pub(crate) container: gtk::Box,
}

impl CarView {
    pub fn new() -> Self {
        let car_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let car = gtk::ImageBuilder::new().file("car265-662.png").expand(true)
            .name("car")
            .build();
        car_box.pack_start(&car,false,false,0);
        return Self { container: car_box };
    }
}
