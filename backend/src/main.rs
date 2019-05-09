#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Mutex;
mod cors;

struct AppState {
    count: usize,
}

#[derive(Serialize, Copy, Clone)]
enum Status {
    Ok,
    Error,
}

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    status: Status,
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T) -> Self {
        ApiResponse {
            status: Status::Ok,
            data,
        }
    }
}

#[get("/count")]
fn count(state: State<Mutex<AppState>>) -> Option<Json<ApiResponse<usize>>> {
    let lock = state.lock().ok()?;
    Some(Json(ApiResponse::ok(lock.count)))
}

#[get("/increment")]
fn increment(state: State<Mutex<AppState>>) -> Option<Json<ApiResponse<usize>>> {
    let mut lock = state.lock().ok()?;
    lock.count = lock.count.saturating_add(1);
    Some(Json(ApiResponse::ok(lock.count)))
}

#[catch(404)]
fn not_found() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse {
        status: Status::Error,
        data: "Resource not found",
    })
}

fn rocket(state: Mutex<AppState>) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![count, increment])
        .register(catchers![not_found])
        .manage(state)
        .attach(cors::AllowOrigin())
}

fn main() {
    let state = Mutex::new(AppState { count: 0 });
    rocket(state).launch();
}
