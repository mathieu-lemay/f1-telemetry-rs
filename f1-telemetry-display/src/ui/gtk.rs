use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::models::*;
use crate::ui::gtk::car_view::CarView;
use crate::ui::gtk::header::HeaderView;
use crate::ui::gtk::lap_times::LapTimesView;
use crate::ui::gtk::throttle_view::ThrottleView;
use crate::ui::Ui;

mod car;
mod car_view;
mod header;
mod lap_times;
mod throttle_view;

extern crate cairo;

pub struct GTKUi {
    app: gtk::Application,
}

const STYLE: &str = "
#session_name {
    font-weight: bold;
}

#lap-times {
    font-weight: bold;
}

.pedal_input label {
    font-weight: bold;
}

.pedal_input block {
    border-style: none;
    border-top-left-radius: 5px;
    border-top-right-radius: 5px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
}

.pedal_input trough {
    border-style: none;
    margin: 5px 0px;
}

#throttle .filled {
    background-color: #00A000;
}

#brake .filled {
    background-color: #A00000;
}

#gear {
    font-size: 3em;
}
";

impl Ui for GTKUi {
    fn new() -> Self {
        let app = gtk::Application::new(Some("org.acidrain.f1-telemetry-rs"), Default::default())
            .expect("Initialization failed...");

        app.connect_activate(|_| {});

        Self { app }
    }

    fn run(&mut self, stream: Stream) {
        self.app.connect_startup(move |app| {
            let provider = gtk::CssProvider::new();
            provider.load_from_path("custom.css").unwrap_or_default();
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            let provider = gtk::CssProvider::new();
            provider
                .load_from_data(STYLE.as_bytes())
                .expect("Failed to load CSS");
            // // We give the CssProvided to the default screen so the CSS rules we added
            // // can be applied to our window.
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            let stream = stream.clone();
            thread::spawn(move || loop {
                match stream.next() {
                    Ok(p) => match p {
                        Some(p) => {
                            let _ = tx.send(p);
                        }
                        None => {
                            thread::sleep(Duration::from_millis(5));
                        }
                    },
                    Err(_e) => {
                        error!("{:?}", _e);
                    }
                }
            });

            let game_state = RefCell::new(GameState::default());
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

fn process_packet(game_state: &RefCell<GameState>, widgets: &Rc<Widgets>, packet: &Packet) {
    game_state.borrow_mut().update(&packet);
    let game_state = game_state.borrow();

    match packet {
        Packet::Session(_) => {
            widgets.header.update(&game_state);
        }
        Packet::Participants(_) => widgets.lap_times_view.set_participants(&game_state),
        Packet::Lap(_) => {
            widgets.header.update(&game_state);
            widgets.lap_times_view.update(&game_state);
        }
        Packet::CarTelemetry(_) => {
            widgets.throttle_view.update(&game_state);
        }
        Packet::CarStatus(_) => widgets.car_view.update(&game_state),
        _ => {}
    }
}

struct Widgets {
    _mwnd: gtk::ApplicationWindow,
    header: HeaderView,
    lap_times_view: LapTimesView,
    throttle_view: ThrottleView,
    car_view: CarView,
}

impl Widgets {
    fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);

        window.set_title("F1 Telemetry");
        window.set_icon_name(Some("application-default-icon"));
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);

        let header = HeaderView::new();

        let lap_times_view = LapTimesView::new();
        let throttle_view = ThrottleView::new();
        let car_view = CarView::new();

        let widgets_grid = gtk::GridBuilder::new()
            .row_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        widgets_grid.attach(&lap_times_view.tree_view, 0, 0, 1, 1);
        widgets_grid.attach(&throttle_view.container, 0, 1, 1, 1);
        widgets_grid.attach(&car_view.container, 1, 0, 1, 1);

        let main_view_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .build();

        main_view_box.pack_start(&header.container, false, false, 0);
        main_view_box.pack_start(&widgets_grid, false, false, 0);

        window.add(&main_view_box);

        window.show_all();

        Self {
            _mwnd: window,
            header,
            lap_times_view,
            throttle_view,
            car_view,
        }
    }
}
