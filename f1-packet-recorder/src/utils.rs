use rusqlite::Connection;
use std::sync::mpsc::{channel, Receiver};

pub(crate) fn ctrl_c_channel() -> anyhow::Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = channel();
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
pub(crate) fn get_database_connection(file: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(file)?;

    Ok(conn)
}
