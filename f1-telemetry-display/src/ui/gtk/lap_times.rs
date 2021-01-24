use gio::prelude::*;
use gtk::prelude::*;
use gtk::{SortColumn, SortType, Widget};

use f1_telemetry::packet::generic::Team;

use crate::fmt;
use crate::models::GameState;

const COLUMN_DEFAULT_WIDTH: i32 = 100;

#[derive(Copy, Clone)]
enum Column {
    Position = 0,
    Name,
    CurrentLapTime,
    LastLapTime,
    BestLapTime,

    BackgroundColor,
    LastLapColor,
    BestLapColor,
    TruePosition,
}

enum FastestLapType {
    None,
    Slow,
    PersonalBest,
    SessionBest,
}

impl FastestLapType {
    fn from(lap_time: u32, personal_best: u32, session_best: u32) -> Self {
        if lap_time == 0 {
            Self::None
        } else if lap_time <= session_best {
            Self::SessionBest
        } else if lap_time <= personal_best {
            Self::PersonalBest
        } else {
            Self::Slow
        }
    }

    fn color(&self) -> &'static str {
        match self {
            Self::Slow => "#dca3a3",
            Self::PersonalBest => "#7f9f7f",
            Self::SessionBest => "#b06698",
            _ => "#bebebe",
        }
    }
}

pub(super) struct LapTimesView {
    tree_view: gtk::TreeView,
    model: gtk::TreeStore,
}

impl LapTimesView {
    pub(super) fn new() -> Self {
        let model = create_model();
        let tree_view = create_tree_view(&model);

        Self { tree_view, model }
    }

    pub(super) fn set_participants(&self, game_state: &GameState) {
        self.model.clear();

        let col_indices = [
            Column::Position,
            Column::Name,
            Column::CurrentLapTime,
            Column::LastLapTime,
            Column::BestLapTime,
            Column::BackgroundColor,
            Column::TruePosition,
        ]
        .iter()
        .map(|&c| c as u32)
        .collect::<Vec<u32>>();

        for (participant, li) in game_state.get_valid_lap_info() {
            let data: [&dyn ToValue; 7] = [
                &fmt::format_position(li.position, &li.status),
                &fmt::format_driver_name(&participant, game_state.session_info.is_online)
                    .to_string(),
                &fmt::milliseconds_to_msf(li.current_lap_time),
                &fmt::milliseconds_to_msf(li.last_lap_time),
                &fmt::milliseconds_to_msf(li.best_lap_time),
                &get_team_color(&participant.team),
                &li.position,
            ];

            self.model
                .set(&self.model.append(None), &col_indices, &data);
        }
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.tree_view
    }

    pub(super) fn update(&self, game_state: &GameState) {
        let iter = match self.model.get_iter_first() {
            Some(i) => i,
            None => return,
        };

        let col_indices = [
            Column::Position,
            Column::CurrentLapTime,
            Column::LastLapTime,
            Column::BestLapTime,
            Column::LastLapColor,
            Column::BestLapColor,
            Column::TruePosition,
        ]
        .iter()
        .map(|&c| c as u32)
        .collect::<Vec<u32>>();

        for (_, li) in game_state.get_valid_lap_info() {
            let data: [&dyn ToValue; 7] = [
                &fmt::format_position(li.position, &li.status),
                &fmt::milliseconds_to_msf(li.current_lap_time),
                &fmt::milliseconds_to_msf(li.last_lap_time),
                &fmt::milliseconds_to_msf(li.best_lap_time),
                &FastestLapType::from(
                    li.last_lap_time,
                    li.best_lap_time,
                    game_state.session_best_times.lap,
                )
                .color(),
                &FastestLapType::from(
                    li.last_lap_time,
                    li.best_lap_time,
                    game_state.session_best_times.lap,
                )
                .color(),
                &li.position,
            ];

            self.model.set(&iter, &col_indices, &data);
            self.model.iter_next(&iter);
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

// pub fn get_cairo_team_color(team:&Team) -> (f64,f64,f64) {
//     let color = match team {
//         Team::Mercedes => (0, 53, 48),
//         Team::Ferrari => (56, 0, 0),
//         Team::RedBullRacing => (15, 0, 65),
//         Team::Williams => (0, 33, 65),
//         Team::RacingPoint => (62, 38, 51),
//         Team::Renault => (65, 62, 0),
//         Team::ToroRosso => (18, 40, 65),
//         Team::Haas => (30, 30, 30),
//         Team::McLaren => (65, 34, 0),
//         Team::AlfaRomeo => (40, 0, 0),
//         Team::AlphaTauri => (65, 65, 65),
//         Team::MyTeam => (30, 0, 65),
//         _ => (255, 255, 255)
//     };
//     let r = color.0 as f64 / 255.0;
//     let g = color.1 as f64 / 255.0;
//     let b = color.2 as f64 / 255.0;
//     (r,g,b)
// }

fn create_model() -> gtk::TreeStore {
    let col_types = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::I8,
    ];

    let model = gtk::TreeStore::new(&col_types);

    let col_indices = [
        Column::Position,
        Column::Name,
        Column::CurrentLapTime,
        Column::LastLapTime,
        Column::BestLapTime,
        Column::BackgroundColor,
        Column::TruePosition,
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
        let data: [&dyn ToValue; 7] = [
            &format!("{}", i * 2 + 1),
            &format!("Player {}", i * 2),
            &fmt::milliseconds_to_msf(0),
            &fmt::milliseconds_to_msf(0),
            &fmt::milliseconds_to_msf(0),
            &get_team_color(t),
            &1,
        ];

        model.set(&model.append(None), &col_indices, &data);

        let data: [&dyn ToValue; 7] = [
            &format!("{}", i * 2 + 2),
            &format!("Player {}", i * 2 + 1),
            &fmt::milliseconds_to_msf(0),
            &fmt::milliseconds_to_msf(0),
            &fmt::milliseconds_to_msf(0),
            &get_team_color(t),
            &1,
        ];

        model.set(&model.append(None), &col_indices, &data);
    }

    model
}

fn create_tree_view(model: &gtk::TreeStore) -> gtk::TreeView {
    let sortable_store = gtk::TreeModelSort::new(model);
    sortable_store.set_sort_column_id(
        SortColumn::Index(Column::TruePosition as u32),
        SortType::Ascending,
    );
    let tree_view = gtk::TreeView::with_model(&sortable_store);
    tree_view.set_widget_name("lap-times");
    tree_view.set_vexpand(true);

    tree_view.set_hover_selection(false);

    let selection = tree_view.get_selection();
    selection.set_select_function(Some(Box::new(|_, _, _, _| false)));

    add_lap_info_columns(&tree_view);

    tree_view
}

fn add_lap_info_columns(treeview: &gtk::TreeView) {
    add_column(treeview, Column::Position, "Position", Some(80), None);
    add_column(treeview, Column::Name, "Player", Some(150), None);
    add_column(treeview, Column::CurrentLapTime, "Current Lap", None, None);
    add_column(
        treeview,
        Column::LastLapTime,
        "Last Lap",
        None,
        Some(Column::LastLapColor),
    );
    add_column(
        treeview,
        Column::BestLapTime,
        "Best Lap",
        None,
        Some(Column::BestLapColor),
    );
}

fn add_column(
    treeview: &gtk::TreeView,
    column: Column,
    title: &str,
    width: Option<i32>,
    foreground_color_column: Option<Column>,
) {
    let renderer = gtk::CellRendererText::new();
    let col = gtk::TreeViewColumn::new();
    col.pack_start(&renderer, true);
    col.set_title(title);
    col.set_fixed_width(width.unwrap_or(COLUMN_DEFAULT_WIDTH));
    col.add_attribute(&renderer, "text", column as i32);
    col.add_attribute(&renderer, "background", Column::BackgroundColor as i32);

    if let Some(c) = foreground_color_column {
        col.add_attribute(&renderer, "foreground", c as i32);
    }

    treeview.append_column(&col);
}
