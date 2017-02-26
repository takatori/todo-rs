#![feature(plugin, custom_derive, custom_attribute)]
// use compiler plugin
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;




mod task;
mod db;

use rocket::response::{Flash, Redirect};
use rocket_contrib::JSON;
use dotenv::dotenv;


use task::Task;

#[post("/todos", data = "<task>")]
fn new(task: JSON<Task>, conn: db::Conn) -> Flash<Redirect> {

    // consumes the JSON wrapper and returns the wrapped item.
    let todo = task.into_inner();

    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if todo.insert(&conn) {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Woops! The server failed.")
    }
}

#[put("/todos/<id>")]
fn toggle(id: i32, conn: db::Conn) -> JSON<&'static str> {

    if Task::toggle_with_id(id, &conn) {
        JSON("Toggled")
    } else {
        JSON("Couldn't toggle task.")
    }
}

#[delete("/todos/<id>")]
fn delete(id: i32, conn: db::Conn) -> JSON<&'static str> {
    if Task::delete_with_id(id, &conn) {
        JSON("Todo was deleted.")
    } else {
        JSON("Couldn't delete task.")
    }
}

#[get("/todos/<id>")]
fn get(id: i32, conn: db::Conn) -> JSON<Option<Task>> {
    JSON(Task::get(id, &conn))
}

#[get("/todos")]
fn list(conn: db::Conn) -> JSON<Vec<Task>> {
    JSON(Task::all(&conn))
}


fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/api/", routes![new, toggle, delete, get, list])
        .launch();
}
