use gio::prelude::*;
use gtk::prelude::*;
use gtk::{SortColumn, SortType, Widget};

use f1_telemetry::packet::generic::Team;

use crate::fmt;
use crate::fmt::AsMinuteTimeString;
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
                &fmt::format_driver_name(participant, game_state.session_info.is_online)
                    .to_string(),
                &li.current_lap_time.as_minute_time_string(),
                &li.last_lap_time.as_minute_time_string(),
                &li.best_lap_time.as_minute_time_string(),
                &get_team_color(game_state.year, &participant.team),
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
                &li.current_lap_time.as_minute_time_string(),
                &li.last_lap_time.as_minute_time_string(),
                &li.best_lap_time.as_minute_time_string(),
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

fn get_team_color(year: u16, team: &Team) -> &'static str {
    match (year, team) {
        (2019, Team::Mercedes) => "rgb(0, 210, 190)",
        (2019, Team::Ferrari) => "rgb(220, 0, 0)",
        (2019, Team::RedBullRacing) => "rgb(30, 65, 255)",
        (2019, Team::Renault) => "rgb(255, 245, 0)",
        (2019, Team::Haas) => "rgb(240, 215, 135)",
        (2019, Team::RacingPoint) => "rgb(245, 150, 200)",
        (2019, Team::ToroRosso) => "rgb(70, 155, 255)",
        (2019, Team::McLaren) => "rgb(255, 135, 0)",
        (2019, Team::AlfaRomeo) => "rgb(155, 0, 0)",
        (2019, Team::Williams) => "rgb(255, 255, 255)",

        (2020, Team::Mercedes) => "rgb(0, 210, 190)",
        (2020, Team::Ferrari) => "rgb(192, 0, 0)",
        (2020, Team::RedBullRacing) => "rgb(6, 0, 239)",
        (2020, Team::Renault) => "rgb(255, 245, 0)",
        (2020, Team::Haas) => "rgb(120, 120, 120)",
        (2020, Team::RacingPoint) => "rgb(245, 150, 200)",
        (2020, Team::AlphaTauri) => "rgb(200, 200, 200)",
        (2020, Team::McLaren) => "rgb(255, 135, 0)",
        (2020, Team::AlfaRomeo) => "rgb(150, 0, 0)",
        (2020, Team::Williams) => "rgb(0, 130, 250)",

        (2021, Team::Mercedes) => "rgb(0, 210, 90)",
        (2021, Team::Ferrari) => "rgb(220, 0, 0)",
        (2021, Team::RedBullRacing) => "rgb(6, 0, 239)",
        (2021, Team::Alpine) => "rgb(0, 144, 255)",
        (2021, Team::Haas) => "rgb(255, 255, 255)",
        (2021, Team::AstonMartin) => "rgb(0, 111, 98)",
        (2021, Team::AlphaTauri) => "rgb(43, 69, 98)",
        (2021, Team::McLaren) => "rgb(255, 135, 0)",
        (2021, Team::AlfaRomeo) => "rgb(144, 0, 0)",
        (2021, Team::Williams) => "rgb(0, 90, 255)",

        (2022, Team::Mercedes) => "rgb(108, 211, 191)",
        (2022, Team::RedBullRacing) => "rgb(30, 91, 198)",
        (2022, Team::Ferrari) => "rgb(237, 28, 36)",
        (2022, Team::McLaren) => "rgb(245, 128, 32)",
        (2022, Team::Alpine) => "rgb(34, 147, 209)",
        (2022, Team::AlphaTauri) => "rgb(78, 124, 155)",
        (2022, Team::AstonMartin) => "rgb(45, 130, 109)",
        (2022, Team::Williams) => "rgb(55, 190, 221)",
        (2022, Team::AlfaRomeo) => "rgb(172, 32, 57)",
        (2022, Team::Haas) => "rgb(182, 186, 189)",

        (_, Team::MyTeam) => "rgb(0, 150, 0)",
        _ => "rgb(128, 128, 128)",
    }
}

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

    for i in 0..20 {
        let data: [&dyn ToValue; 7] = [
            &format!("{}", i + 1),
            &format!("Player {}", i + 1),
            &0.as_minute_time_string(),
            &0.as_minute_time_string(),
            &0.as_minute_time_string(),
            &get_team_color(0, &Team::Unknown),
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
