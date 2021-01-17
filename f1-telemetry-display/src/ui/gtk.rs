use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::fmt;
use crate::models::*;
use crate::ui::Ui;

pub struct GTKUi {
    app: gtk::Application,
}

impl Ui for GTKUi {
    fn new() -> Self {
        let app = gtk::Application::new(Some("org.acidrain.f1-telemetry-rs"), Default::default())
            .expect("Initialization failed...");

        app.connect_activate(|_| {});

        Self { app }
    }

    fn run(&mut self, stream: Stream) {
        self.app.connect_startup(move |app| {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            let stream = stream.clone();
            thread::spawn(move || loop {
                match stream.next() {
                    Ok(p) => match p {
                        Some(p) => {
                            let _ = tx.send(p);
                        }
                        None => {
                            thread::sleep(Duration::from_millis(500));
                        }
                    },
                    Err(_e) => {
                        error!("{:?}", _e);
                    }
                }
            });

            let game_state = Rc::new(RefCell::new(GameState::default()));
            let widgets = Rc::new(Widgets::new(&app));

            rx.attach(None, move |packet| {
                process_packet(&game_state, &widgets, &packet);

                glib::Continue(true)
            });
        });

        self.app.run(&Vec::new());
    }

    fn destroy(&self) {}
}

fn process_packet(game_state: &Rc<RefCell<GameState>>, widgets: &Rc<Widgets>, packet: &Packet) {
    game_state.borrow_mut().update(&packet);
    let ts = game_state.borrow().session_info.elapsed_time;
    widgets.button.set_label(&fmt::format_time_hms(ts));
    widgets.mwnd.show_all();
}

struct Widgets {
    mwnd: gtk::ApplicationWindow,
    button: gtk::Button,
}

impl Widgets {
    fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);

        window.set_title("F1 Telemetry");
        window.set_icon_name(Some("package-x-generic"));
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        let button = gtk::Button::with_label("Click me!");
        button.connect_clicked(|b| println!("Button clicked: {:?}", b));
        window.add(&button);

        window.show_all();

        Self {
            mwnd: window,
            button,
        }
    }
}
