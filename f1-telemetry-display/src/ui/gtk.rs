use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::models::*;
use crate::ui::gtk::lap_times::LapTimesView;
use crate::ui::Ui;

mod lap_times;

pub struct GTKUi {
    app: gtk::Application,
}

const STYLE: &str = "
#lap-times {
    font-weight: bold;
}";

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

            let mut session_uid: Option<u64> = None;
            let game_state = Rc::new(RefCell::new(GameState::default()));
            let widgets = Rc::new(Widgets::new(&app));

            rx.attach(None, move |packet| {
                process_packet(&session_uid, &game_state, &widgets, &packet);
                session_uid = game_state.borrow().session_uid;

                glib::Continue(true)
            });
        });

        self.app.run(&Vec::new());
    }

    fn destroy(&self) {}
}

fn process_packet(
    _session_uid: &Option<u64>,
    game_state: &Rc<RefCell<GameState>>,
    widgets: &Rc<Widgets>,
    packet: &Packet,
) {
    game_state.borrow_mut().update(&packet);
    let game_state = game_state.borrow();

    if let Packet::Lap(_) = packet {
        widgets.lap_times_view.update(&game_state);
    }
}

struct Widgets {
    _mwnd: gtk::ApplicationWindow,
    lap_times_view: LapTimesView,
}

impl Widgets {
    fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);

        window.set_title("F1 Telemetry");
        window.set_icon_name(Some("application-default-icon"));
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);

        let lap_times_view = LapTimesView::new(&window);

        window.show_all();

        Self {
            _mwnd: window,
            lap_times_view,
        }
    }
}
