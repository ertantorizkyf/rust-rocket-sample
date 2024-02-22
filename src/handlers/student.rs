use crate::{
    models::student::{AppState, Student, UpdateStudentSchema},
    responses::general::GeneralResponse,
    responses::student::{StudentResponse, StudentListResponse}
};
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State,
};
use uuid::Uuid;

#[get("/?<page>&<limit>")]
pub async fn student_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<StudentListResponse>, Status> {
    let vec = data.student_db.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let students: Vec<Student> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = StudentListResponse {
        status: "success".to_string(),
        results: students.len(),
        data: students,
    };

    Ok(Json(json_response))
}

#[post("/", data = "<body>")]
pub async fn create_student_handler(
    mut body: Json<Student>,
    data: &State<AppState>,
) -> Result<Json<StudentResponse>, Custom<Json<GeneralResponse>>> {
    let mut vec = data.student_db.lock().unwrap();

    for student in vec.iter() {
        if student.name == body.name && student.major == body.major {
            let error_response = GeneralResponse {
                status: "fail".to_string(),
                message: format!("'{}' is already admitted in {} major", student.name, student.major),
            };

            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let student = body.to_owned();

    vec.push(body.into_inner());

    let json_response = StudentResponse {
        status: "success".to_string(),
        data: student.into_inner(),
    };

    Ok(Json(json_response))
}

#[get("/<id>")]
pub async fn get_student_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Json<StudentResponse>, Custom<Json<GeneralResponse>>> {
    let vec = data.student_db.lock().unwrap();

    for student in vec.iter() {
        if student.id == Some(id.to_owned()) {
            let json_response = StudentResponse {
                status: "success".to_string(),
                data: student.clone(),
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = GeneralResponse {
        status: "fail".to_string(),
        message: format!("Student with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}

#[patch("/<id>", data = "<body>")]
pub async fn edit_student_handler(
    id: String,
    body: Json<UpdateStudentSchema>,
    data: &State<AppState>,
) -> Result<Json<StudentResponse>, Custom<Json<GeneralResponse>>> {
    let mut vec = data.student_db.lock().unwrap();

    for student in vec.iter_mut() {
        if student.id == Some(id.clone()) {
            let datetime = Utc::now();
            let name = body.name.to_owned().unwrap_or(student.name.to_owned());
            let major = body.major.to_owned().unwrap_or(student.major.to_owned());
            let admission_year = body.admission_year.to_owned().unwrap_or(student.admission_year.to_owned());
            
            let payload = Student {
                id: student.id.to_owned(),
                name: if !name.is_empty() {
                    name
                } else {
                    student.name.to_owned()
                },
                major: if !major.is_empty() {
                    major
                } else {
                    student.major.to_owned()
                },
                admission_year: if !admission_year.is_empty() {
                    admission_year
                } else {
                    student.admission_year.to_owned()
                },
                createdAt: student.createdAt,
                updatedAt: Some(datetime),
            };
            *student = payload;

            let json_response = StudentResponse {
                status: "success".to_string(),
                data: student.clone(),
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = GeneralResponse {
        status: "fail".to_string(),
        message: format!("Student with ID: {} not found", id),
    };

    Err(Custom(Status::NotFound, Json(error_response)))
}

#[delete("/<id>")]
pub async fn delete_student_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Status, Custom<Json<GeneralResponse>>> {
    let mut vec = data.student_db.lock().unwrap();

    for student in vec.iter_mut() {
        if student.id == Some(id.clone()) {
            vec.retain(|student| student.id != Some(id.to_owned()));

            return Ok(Status::NoContent);
        }
    }

    let error_response = GeneralResponse {
        status: "fail".to_string(),
        message: format!("Student with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}
