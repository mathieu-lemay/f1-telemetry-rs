use crate::fmt;
use crate::models::{GameState, LapInfo};
use f1_telemetry::packet::generic::{ResultStatus, Team};
use gio::prelude::*;
use gtk::prelude::*;

pub(super) struct LapTimesView {
    _tree_view: gtk::TreeView,
    model: gtk::ListStore,
}

#[derive(Copy, Clone)]
enum Column {
    Position = 0,
    Name,
    CurrentLapTime,
    LastLapTime,
    BestLapTime,

    BackgroundColor,
}

impl LapTimesView {
    pub(super) fn new(parent: &gtk::ApplicationWindow) -> Self {
        let model = create_model();
        let tree_view = create_tree_view(&model);

        parent.add(&tree_view);

        Self {
            _tree_view: tree_view,
            model,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        self.model.clear();

        let col_indices = [
            Column::Position,
            Column::Name,
            Column::CurrentLapTime,
            Column::LastLapTime,
            Column::BestLapTime,
            Column::BackgroundColor,
        ]
        .iter()
        .map(|&c| c as u32)
        .collect::<Vec<u32>>();

        let mut lap_infos: Vec<(usize, &LapInfo)> = game_state
            .lap_infos
            .iter()
            .filter(|li| li.status != ResultStatus::Invalid)
            .enumerate()
            .collect();
        lap_infos.sort_by_key(|(_, li)| li.position);

        for (idx, li) in lap_infos {
            let participant = &game_state.participants[idx];

            let data: [&dyn ToValue; 6] = [
                &fmt::format_position(li.position, &li.status),
                &fmt::format_driver_name(&participant, game_state.session_info.is_online)
                    .to_string(),
                &fmt::milliseconds_to_hmsf(li.current_lap_time),
                &fmt::milliseconds_to_hmsf(li.last_lap_time),
                &fmt::milliseconds_to_hmsf(li.best_lap_time),
                &get_team_color(&participant.team),
            ];

            self.model.set(&self.model.append(), &col_indices, &data);
        }
    }
}

fn get_team_color(team: &Team) -> String {
    let color = match team {
        Team::Mercedes => "rgb(0, 53, 48)",
        Team::Ferrari => "rgb(56, 0, 0)",
        Team::RedBullRacing => "rgb(15, 0, 65)",
        Team::Williams => "rgb(0, 33, 65)",
        Team::RacingPoint => "rgb(62, 38, 51)",
        Team::Renault => "rgb(65, 62, 0)",
        Team::ToroRosso => "rgb(18, 40, 65)",
        Team::Haas => "rgb(30, 30, 30)",
        Team::McLaren => "rgb(65, 34, 0)",
        Team::AlfaRomeo => "rgb(40, 0, 0)",
        Team::AlphaTauri => "rgb(65, 65, 65)",
        Team::MyTeam => "rgb(30, 0, 65)",
        _ => "",
    };

    String::from(color)
}

fn create_model() -> gtk::ListStore {
    let col_types = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
    ];

    let model = gtk::ListStore::new(&col_types);

    let col_indices = [
        Column::Position,
        Column::Name,
        Column::CurrentLapTime,
        Column::LastLapTime,
        Column::BestLapTime,
        Column::BackgroundColor,
    ]
    .iter()
    .map(|&c| c as u32)
    .collect::<Vec<u32>>();

    let teams = [
        Team::Mercedes,
        Team::RedBullRacing,
        Team::McLaren,
        Team::RacingPoint,
        Team::Renault,
        Team::Ferrari,
        Team::AlphaTauri,
        Team::AlfaRomeo,
        Team::Haas,
        Team::Williams,
    ];
    for (i, t) in teams.iter().enumerate() {
        let data: [&dyn ToValue; 6] = [
            &format!("{}", i * 2 + 1),
            &format!("Player {}", i * 2),
            &fmt::milliseconds_to_hmsf(0),
            &fmt::milliseconds_to_hmsf(0),
            &fmt::milliseconds_to_hmsf(0),
            &get_team_color(t),
        ];

        model.set(&model.append(), &col_indices, &data);

        let data: [&dyn ToValue; 6] = [
            &format!("{}", i * 2 + 2),
            &format!("Player {}", i * 2 + 1),
            &fmt::milliseconds_to_hmsf(0),
            &fmt::milliseconds_to_hmsf(0),
            &fmt::milliseconds_to_hmsf(0),
            &get_team_color(t),
        ];

        model.set(&model.append(), &col_indices, &data);
    }

    model
}

fn create_tree_view(model: &gtk::ListStore) -> gtk::TreeView {
    let tree_view = gtk::TreeView::with_model(model);
    tree_view.set_widget_name("lap-times");
    tree_view.set_vexpand(true);

    tree_view.set_hover_selection(false);

    let selection = tree_view.get_selection();
    selection.set_select_function(Some(Box::new(|_, _, _, _| false)));

    add_lap_info_columns(&tree_view);

    tree_view
}

fn add_lap_info_columns(treeview: &gtk::TreeView) {
    add_column(treeview, Column::Position, "Position");
    add_column(treeview, Column::Name, "Player");
    add_column(treeview, Column::CurrentLapTime, "Current Lap");
    add_column(treeview, Column::LastLapTime, "Last Lap");
    add_column(treeview, Column::BestLapTime, "Best Lap");
}

fn add_column(treeview: &gtk::TreeView, column: Column, title: &str) {
    let renderer = gtk::CellRendererText::new();
    let col = gtk::TreeViewColumn::new();
    col.pack_start(&renderer, true);
    col.set_title(title);
    col.add_attribute(&renderer, "text", column as i32);
    col.add_attribute(&renderer, "background", Column::BackgroundColor as i32);
    treeview.append_column(&col);
}
