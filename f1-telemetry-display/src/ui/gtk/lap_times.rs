use crate::fmt;
use crate::models::GameState;
use f1_telemetry::packet::generic::{ResultStatus, Team};
use gio::prelude::*;
use gtk::prelude::*;

pub(super) struct LapTimesView {
    _tree_view: gtk::TreeView,
    model: gtk::ListStore,
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

        let col_indices = [0, 1, 2, 3];
        for (idx, li) in game_state
            .lap_infos
            .iter()
            .filter(|li| li.status != ResultStatus::Invalid)
            .enumerate()
        {
            let participant = &game_state.participants[idx];

            let pos = fmt::format_position(li.position, &li.status);
            let name = fmt::format_driver_name(&participant, game_state.session_info.is_online);
            let lap_time = fmt::milliseconds_to_msf(li.current_lap_time);
            let color = get_team_color(&participant.team);

            let data: [&dyn ToValue; 4] = [&pos, &name.to_string(), &lap_time, &color];

            self.model.set(&self.model.append(), &col_indices, &data);
        }
    }
}

fn get_team_color(team: &Team) -> String {
    let color = match team {
        Team::Mercedes => "rgb(0, 210, 190)",
        Team::Ferrari => "rgb(220, 0, 0)",
        Team::RedBullRacing => "rgb(60, 0, 255)",
        Team::Williams => "rgb(0, 128, 255)",
        Team::RacingPoint => "rgb(245, 150, 200)",
        Team::Renault => "rgb(255, 245, 0)",
        Team::ToroRosso => "rgb(70, 155, 255)",
        Team::Haas => "rgb(119, 119, 119)",
        Team::McLaren => "rgb(255, 135, 0)",
        Team::AlfaRomeo => "rgb(155, 0, 0)",
        Team::AlphaTauri => "rgb(255, 255, 255)",
        Team::MyTeam => "rgb(118, 0, 218)",
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
    ];

    let model = gtk::ListStore::new(&col_types);

    let col_indices = [0, 1, 2, 3];
    for i in 0..20 {
        let data: [&dyn ToValue; 4] = [
            &format!("{}", i + 1),
            &format!("Player {}", i),
            &fmt::milliseconds_to_hmsf(0),
            if i % 2 == 0 { &"#323232" } else { &"#484848" },
        ];

        model.set(&model.append(), &col_indices, &data);
    }

    model
}

fn create_tree_view(model: &gtk::ListStore) -> gtk::TreeView {
    let tree_view = gtk::TreeView::with_model(model);
    tree_view.set_widget_name("foo");
    tree_view.set_vexpand(true);

    tree_view.set_hover_selection(false);

    let selection = tree_view.get_selection();
    selection.set_select_function(Some(Box::new(|_, _, _, _| false)));

    add_columns(model, &tree_view);

    tree_view
}

fn add_columns(_model: &gtk::ListStore, treeview: &gtk::TreeView) {
    // Column for fixed toggles
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Position");
        column.add_attribute(&renderer, "text", 0);
        column.add_attribute(&renderer, "background", 3);
        // column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        // column.set_fixed_width(50);
        treeview.append_column(&column);
    }

    // Column for bug numbers
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Player");
        column.add_attribute(&renderer, "text", 1);
        column.add_attribute(&renderer, "background", 3);
        // column.set_sort_column_id(1);
        treeview.append_column(&column);
    }

    // Column for severities
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Lap Time");
        column.add_attribute(&renderer, "text", 2);
        column.add_attribute(&renderer, "background", 3);
        // column.set_sort_column_id(2);
        treeview.append_column(&column);
    }

    // // Column for description
    // {
    //     let renderer = gtk::CellRendererText::new();
    //     let column = gtk::TreeViewColumn::new();
    //     column.pack_start(&renderer, true);
    //     column.set_title("Description");
    //     column.add_attribute(&renderer, "text", Columns::Description as i32);
    //     column.set_sort_column_id(Columns::Description as i32);
    //     treeview.append_column(&column);
    // }
    //
    // // Column for spinner
    // {
    //     let renderer = gtk::CellRendererSpinner::new();
    //     let column = gtk::TreeViewColumn::new();
    //     column.pack_start(&renderer, true);
    //     column.set_title("Spinning");
    //     column.add_attribute(&renderer, "pulse", Columns::Pulse as i32);
    //     column.add_attribute(&renderer, "active", Columns::Active as i32);
    //     treeview.append_column(&column);
    // }
    //
    // // Column for symbolic icon
    // {
    //     let renderer = gtk::CellRendererPixbuf::new();
    //     let column = gtk::TreeViewColumn::new();
    //     column.pack_start(&renderer, true);
    //     column.set_title("Symbolic icon");
    //     column.add_attribute(&renderer, "icon-name", Columns::Icon as i32);
    //     column.add_attribute(&renderer, "sensitive", Columns::Sensitive as i32);
    //     column.set_sort_column_id(Columns::Icon as i32);
    //     treeview.append_column(&column);
    // }
}
