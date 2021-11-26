mod entities;

use std::collections::{HashMap, BTreeMap};
use std::path::Path;
use std::time::SystemTime;
use rusqlite::{Connection};

pub type SQLError = rusqlite::Error;

pub struct TaskGroup {
    name: String,
    tasks: Vec<(String, Vec<Task>)>,
}

pub struct Task {
    name: String,
    description: Option<String>,
    actions: BTreeMap<SystemTime, Action>,
    annexes: HashMap<String, std::path::Path>,
}

pub struct Action {
    state: State,
    name: String,
    description: Option<String>,
}

#[derive(FromPrimitive,ToPrimitive,PartialOrd)]
enum State {
    CANCELLED = 0,
    OPEN = 1,
    DONE = 2,
}

const VERSION: str = "0.1";

pub fn get_db(path: &str) -> Result<&Connection, SQLError> {
    let conn = Connection::open(path)?;
    match conn.query_row("SELECT value FROM configuration WHERE name = ?",["version"],|row| row.get(0) {
        Ok(VERSION) => &conn,
        _ => migrate(conn)
    }
}

fn migrate(conn: &Connection) -> Result<&Connection, SQLError> {]
    conn.execute("CREATE TABLE IF NOT EXISTS configuration (name TEXT PRIMARY KEY, value TEXT NOT NULL)", [])?;
    // TODO: create a migration schema from old versions (since this is the first one, i will pass)
    conn.execute_batch("BEGIN;
    CREATE TABLE IF NOT EXISTS TaskGroup (
        id      INTEGET PRIMARY KEY,
        name    TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS Task (
        id      INTEGER PRIMARY KEY,
        group   INTEGER NOT NULL,
        name    TEXT NOT NULL,
        desc    TEXT
    ) CONSTRAINT fk_group_task FOREIGN KEY (group)
      REFERENCES TaskGroup (id);
    CREATE TABLE IF NOT EXISTS Annex (
        id      INTEGER PRIMARY KEY,
        task    INTEGER NOT NULL,
        path    TEXT NOT NULL,
    ) CONSTRAINT fk_task_annex FOREIGN_KEY (task)
      REFERENCES Task (id);
    CREATE TABLE IF NOT EXISTS Action ()
    COMMIT;")?;
    conn
}