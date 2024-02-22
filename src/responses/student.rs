use crate::models::student::Student;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct StudentData {
    pub student: Student,
}

#[derive(Serialize, Debug)]
pub struct StudentResponse {
    pub status: String,
    pub data: Student,
}

#[derive(Serialize, Debug)]
pub struct StudentListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<Student>,
}
