use handlers::student::{
    create_student_handler, 
    delete_student_handler, 
    edit_student_handler, 
    get_student_handler, 
    student_list_handler
};
use handlers::general::{
    env_impl,
    fibonacci,
    middleware_impl_test
};
use handlers::fs::{
    read_txt_file,
    write_txt_file,
    read_json_file,
    write_json_file,
    read_json_list_file
};
use handlers::curl::pokeapi;
use handlers::redis::{
    redis_get,
    redis_set,
    redis_get_hash,
    redis_set_hash
};
use handlers::database::{
    get_articles,
    insert_article
};
use handlers::auth::authorization;
use fairings::auth::JwtValidationFairing;
use helpers::catcher::not_found;

#[macro_use]
extern crate rocket;

mod handlers;
mod models;
mod responses;
mod helpers;
mod constants;
mod fairings;
mod tests;

#[launch]
fn rocket() -> _ {
    let student_app_data = models::student::AppState::init();
    rocket::build()
        .manage(student_app_data)
        .attach(JwtValidationFairing)
        .register("/", catchers![not_found])
        .mount(
            "/api/students",
            routes![
                create_student_handler,
                delete_student_handler,
                edit_student_handler,
                get_student_handler,
                student_list_handler
            ],
        )
        .mount(
            "/api/general",
            routes![
                env_impl,
                fibonacci,
                middleware_impl_test
            ],
        )
        .mount(
            "/api/fs",
            routes![
                read_txt_file,
                write_txt_file,
                read_json_file,
                write_json_file,
                read_json_list_file
            ],
        )
        .mount(
            "/api/curl",
            routes![
                pokeapi
            ],
        )
        .mount(
            "/api/redis",
            routes![
                redis_get,
                redis_set,
                redis_get_hash,
                redis_set_hash
            ],
        )
        .mount(
            "/api/database",
            routes![
                get_articles,
                insert_article
            ],
        )
        .mount(
            "/api/auth",
            routes![
                authorization
            ],
        )
}

#[cfg(test)]
#[path = "./tests/rocket_general_test.rs"]
mod rocket_general_test;
