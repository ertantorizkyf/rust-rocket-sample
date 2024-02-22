use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Student {
    pub id: Option<String>,
    pub name: String,
    pub major: String,
    pub admission_year: String,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StudentAlt {
    pub name: String,
    pub major: String,
    pub admission_year: String
}

pub struct AppState {
    pub student_db: Arc<Mutex<Vec<Student>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            student_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UpdateStudentSchema {
    pub name: Option<String>,
    pub major: Option<String>,
    pub admission_year: Option<String>,
}
