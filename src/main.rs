use handlers::student::{
    create_student_handler, 
    delete_student_handler, 
    edit_student_handler, 
    get_student_handler, 
    student_list_handler
};
use handlers::general::{
    env_impl,
    fibonacci
};
use handlers::fs::{
    read_txt_file,
    write_txt_file,
    read_json_file,
    write_json_file,
    read_json_list_file
};
use handlers::curl::pokeapi;

#[macro_use]
extern crate rocket;

mod handlers;
mod models;
mod responses;
mod helpers;

#[launch]
fn rocket() -> _ {
    let student_app_data = models::student::AppState::init();
    rocket::build()
        .manage(student_app_data)
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
                fibonacci
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
}
