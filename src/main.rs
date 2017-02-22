#![feature(plugin, custom_derive, custom_attribute)]
// use compiler plugin
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate dotenv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;

mod task;
mod db;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::JSON;
use dotenv::dotenv;


use task::Task;

#[derive(Debug, Serialize, Deserialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    tasks: Vec<Task>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
            task: Task::all(conn),
        }
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            tasks: Task::all(conn),
        }
    }
}

#[post("/", data = "<task>")]
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

#[put("/<id>")]
fn toggle(id: i32, conn: db::Conn) -> Result<Redirect, JSON> {
    if Task::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(JSON(&Context::err(&conn, "Couldn't toggle task.")))
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Conn) -> Result<Flash<Redirect>, JSON> {
    if Task::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
    } else {
        Err(JSON(&Context::err(&conn, "Couldn't delete task.")))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: db::Conn) -> JSON {
    &match msg {
        Some(ref msg) => JSON(Context::raw(&conn, Some((msg.name(), msg.msg())))),
        None => JSON(Context::raw(&conn, None)),
    }
}


fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index, static_files::all])
        .mount("/todo/", routes![new, toggle, delete])
        .lounch();
}
