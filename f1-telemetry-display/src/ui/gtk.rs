extern crate cairo;

use std::cell::RefCell;
use std::rc::Rc;

use async_trait::async_trait;
use gio::prelude::*;
use gtk::prelude::*;

use f1_telemetry::packet::Packet;

use crate::models::*;
use crate::ui::Ui;

use self::car_view::CarView;
use self::events::EventsView;
use self::header::HeaderView;
use self::lap_times::LapTimesView;
use self::race_data_view::RaceDataView;
use self::style::BASE_STYLE;
use self::throttle_view::ThrottleView;
use self::tyre_temp_view::TyreTempView;

mod car;
mod car_view;
mod events;
mod header;
mod lap_times;
mod race_data_view;
mod style;
mod throttle_view;
mod tyre_temp;
mod tyre_temp_view;

pub(crate) struct GtkUi {
    app: gtk::Application,
}

unsafe impl Send for GtkUi {}

#[async_trait]
impl Ui for GtkUi {
    fn new() -> Self {
        let app = gtk::Application::new(Some("org.acidrain.f1-telemetry-rs"), Default::default())
            .expect("Initialization failed...");

        Self { app }
    }

    async fn run(&mut self) {
        self.app.connect_startup(move |_| {
            let provider = gtk::CssProvider::new();
            provider.load_from_path("custom.css").unwrap_or_default();
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            let provider = gtk::CssProvider::new();
            provider
                .load_from_data(BASE_STYLE.as_bytes())
                .expect("Failed to load CSS");
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        });

        self.app.connect_activate(move |app| {
            let game_state = RefCell::new(GameState::default());
            let widgets = Rc::new(Widgets::new(app));

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

            tokio::spawn(async move {
                while let Some(p) = crate::CHANNEL.rx.write().await.recv().await {
                    let _ = tx.send(p);
                }
            });

            rx.attach(None, move |packet| {
                process_packet(&game_state, &widgets, &packet);

                Continue(true)
            });
        });

        self.app.run(&Vec::new());
    }

    fn destroy(&self) {}
}

fn process_packet(game_state: &RefCell<GameState>, widgets: &Rc<Widgets>, packet: &Packet) {
    game_state.borrow_mut().update(packet);
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
            widgets.tyre_temp_view.update(&game_state);
        }
        Packet::CarStatus(_) => {
            widgets.car_view.update(&game_state);
            widgets.race_data_view.update(&game_state)
        }
        Packet::Event(_) => widgets.events_view.update(&game_state),
        _ => {}
    }
}

struct Widgets {
    header: HeaderView,
    lap_times_view: LapTimesView,
    throttle_view: ThrottleView,
    car_view: CarView,
    events_view: EventsView,
    tyre_temp_view: TyreTempView,
    race_data_view: RaceDataView,
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
        let events_view = EventsView::new();
        let tyre_temp_view = TyreTempView::new();
        let race_data_view = RaceDataView::new();

        let widgets_grid = gtk::GridBuilder::new()
            .row_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        widgets_grid.attach(lap_times_view.widget(), 0, 0, 1, 1);
        widgets_grid.attach(throttle_view.widget(), 0, 1, 1, 1);
        widgets_grid.attach(car_view.widget(), 1, 0, 1, 1);
        widgets_grid.attach(tyre_temp_view.widget(), 2, 0, 1, 1);
        widgets_grid.attach(race_data_view.widget(), 1, 1, 1, 1);

        let main_view_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .build();

        main_view_box.pack_start(header.widget(), false, false, 0);
        main_view_box.pack_start(&widgets_grid, false, false, 0);
        main_view_box.pack_start(events_view.widget(), false, false, 0);

        window.add(&main_view_box);

        window.show_all();

        Self {
            header,
            lap_times_view,
            throttle_view,
            car_view,
            events_view,
            tyre_temp_view,
            race_data_view,
        }
    }
}
