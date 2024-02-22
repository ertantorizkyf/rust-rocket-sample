use crate::{
    models::general::GeneralBody,
    responses::general::GeneralResponse
};

use std::{
    fs,
    path::Path
};
use rocket::{
    get,
    http::Status,
    response::status::Custom,
    serde::json::{
        from_str,
        to_pretty_string,
        Json,
        Value
    }
};

#[get("/txt")]
pub async fn read_txt_file() -> Result<Json<GeneralResponse>, Status> {
    let file_path = Path::new("./assets/sample.txt");
    let contents = fs::read_to_string(file_path)
        .expect("LogRocket: error reading file");

    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: contents,
    };

    Ok(Json(json_response))
}

#[post("/txt", data = "<body>")]
pub async fn write_txt_file(
    body: Json<GeneralBody>
) -> Result<Json<GeneralResponse>, Custom<Json<GeneralResponse>>> {
    let file_path = Path::new("./assets/sample.txt");
    let content = &body.content;
    fs::write(file_path, content)
        .expect("LogRocket: error writing to file");
    
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: "Txt file manipulated".to_string(),
    };

    Ok(Json(json_response))
}

#[get("/json")]
pub async fn read_json_file() -> Result<Json<GeneralResponse>, Status> {
    let file_path = Path::new("./assets/sample.json");
    let content = {
        let file_content = fs::read_to_string(file_path)
            .expect("LogRocket: error reading file");
        from_str::<Value>(&file_content)
            .expect("LogRocket: error serializing to JSON")
    };
    
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: to_pretty_string(&content).expect("LogRocket: error parsing to JSON"),
    };

    Ok(Json(json_response))
}

#[post("/json", data="<body>")]
pub async fn write_json_file(
    body: Json<GeneralBody>
) -> Result<Json<GeneralResponse>, Status> {
    let file_path = Path::new("./assets/sample.json");
    let content = GeneralBody {
        content: body.content.to_owned()
    };
    fs::write(
        file_path,
        to_pretty_string(&content).expect("LogRocket: error parsing to JSON")
    )
    .expect("LogRocket: error writing to file");
    
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: to_pretty_string(&content).expect("LogRocket: error parsing to JSON"),
    };

    Ok(Json(json_response))
}

#[get("/json-list")]
pub async fn read_json_list_file() -> Result<Json<GeneralResponse>, Status> {
    let file_path = Path::new("./assets/sample-list.json");
    let contents = {
        let file_content = fs::read_to_string(file_path)
            .expect("LogRocket: error reading file");
        from_str::<Value>(&file_content)
            .expect("LogRocket: error serializing to JSON")
    };
    
    let json_response = GeneralResponse {
        status: "success".to_string(),
        message: to_pretty_string(&contents).expect("LogRocket: error parsing to JSON"),
    };

    Ok(Json(json_response))
}
