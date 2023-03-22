
#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rusqlite::Connection;

#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    item: String
}

#[derive(Serialize)]
struct StatusMessage {
    message: String
}

#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<ToDoList>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to db"));
        }
    };

    let mut statement = match db_connection
        .prepare("SELECT id, item FROM todo_list;") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into())
    };
        

    let results = statement.query_map([], |row| {
        Ok(ToDoItem{
            id: row.get(0)?,
            item: row.get(1)?
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<ToDoItem>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList{ items })),
                Err(_) => Err("Could not collect items".into())
            }
        },
        Err(_) => Err("Failed to fetch todo items".into())
    }
}

#[post("/todo", format = "json", data = "<item>")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to db"));
        }
    };

    let mut statement = match db_connection
        .prepare("INSERT INTO todo_list (id, item) VALUES (NULL, $1);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into())
    };

    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { message: format!("{} rows inserted", rows_affected) })),
        Err(_) => Err("Failed to insert todo items".into())
    }
}

#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to db"));
        }
    };

    let mut statement = match db_connection
        .prepare("DELETE FROM todo_list WHERE id = $1;") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into())
    };

    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { message: format!("{} rows deleted", rows_affected) })),
        Err(_) => Err("Failed to insert todo items".into())
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();
    
        db_connection.execute(
            "CREATE TABLE IF NOT EXISTS todo_list (
                id INTEGER PRIMARY KEY,
                item VARCHAR(64) NOT NULL
            );",
            []
        ).unwrap();
    }

    rocket::build().mount("/", routes![index, fetch_all_todo_items, add_todo_item, delete_todo_item])
}