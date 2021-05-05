use async_trait::async_trait;

use crate::ui::{gtk::GtkUi, nc::NcursesUi};

mod gtk;
mod nc;

#[async_trait]
pub trait Ui {
    fn new() -> Self
    where
        Self: Sized;
    async fn run(&mut self, host: String, port: u16);
    fn destroy(&self);
}

pub fn get_ui(ui: &str) -> Box<dyn Ui> {
    match ui {
        "gtk" => Box::new(GtkUi::new()),
        "ncurses" => Box::new(NcursesUi::new()),
        _ => panic!("Invalid ui: {}", ui),
    }
}

// fn start_stream<F>(host: &str, port: u16, send_fn: &'static F)
// where
//     F: Fn(Packet) -> () + Send + Sync + 'static,
// {
//     let host = host.to_string();
//
//     tokio::spawn(async move {
//         let stream = Stream::new(format!("{}:{}", host, port))
//             .await
//             .expect("Unable to bind socket");
//
//         loop {
//             match stream.next().await {
//                 Ok(p) => {
//                     let _ = send_fn(p);
//                 }
//                 Err(_e) => {
//                     error!("{:?}", _e);
//                 }
//             }
//         }
//     });
// }
