use f1_telemetry::Stream;
use gio::prelude::*;
use gtk::prelude::*;

use std::thread;
use std::time::Duration;

use crate::models::*;
use crate::ui::Ui;

pub struct GTKUi {
    _app: gtk::Application,
    _game_state: GameState,
}

impl Ui for GTKUi {
    fn new(stream: Stream) -> Self {
        let application =
            gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
                .expect("Initialization failed...");

        application.connect_activate(|app| {
            println!("CONNECT ACTIVATE");
            build_ui(app);
        });

        application.connect_startup(move |_app| {
            println!("CONNECT STARTUP");
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            let stream = stream.clone();

            thread::spawn(move || {
                loop {
                    match stream.next() {
                        Ok(p) => match p {
                            Some(p) => {
                                println!("Real shit");
                                let _ = tx.send(Some(p));
                                //game_state.update(&p);
                                //ui.render(&game_state, &p);
                            }
                            None => {
                                println!("I sleep");
                                let _ = tx.send(None);
                                thread::sleep(Duration::from_millis(500));
                            }
                        },
                        Err(_e) => {
                            error!("{:?}", _e);
                        }
                    }
                }
            });

            rx.attach(None, move |val| {
                println!("RX: {:?}", val);
                glib::Continue(true)
            });
        });

        application.run(&Vec::new());

        Self {
            _app: application,
            _game_state: GameState::default(),
        }
    }

    fn run(&mut self) {}

    fn destroy(&self) {}
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");

    button.connect_clicked(|b| println!("Button clicked: {:?}", b));

    window.add(&button);

    window.show_all();
}
