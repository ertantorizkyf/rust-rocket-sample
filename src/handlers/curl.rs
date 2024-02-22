use crate::{
    models::pokemon::{
        Pokemon, 
        PokeapiList
    },
    responses::pokemon::PokemonListResponse
};

use curl::easy::Easy;
use std::env;
use rocket::{
    get,
    http::Status,
    serde::json::{
        from_slice,
        Json
    }
};

#[get("/pokeapi?<page>&<limit>")]
pub fn pokeapi(
    page: Option<usize>,
    limit: Option<usize>
) -> Result<Json<PokemonListResponse>, Status> {
    let limit = limit.unwrap_or(20);
    let page = (page.unwrap_or(1) - 1) * limit;
    let pokeapi_base_url = match env::var_os("POKEAPI_BASE_URL") {
        Some(v) => v.into_string().unwrap(),
        None => "https://pokeapi.co/api/v2/".to_string()
    };
    let pokeapi_full_url = format!("{}pokemon?offset={}", pokeapi_base_url, page);
    print!("full url: {}", pokeapi_full_url);

    let mut pokeapi_response = Vec::new();
    let mut easy = Easy::new();
    easy.url(pokeapi_full_url.as_str()).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        pokeapi_response.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let final_resp: PokeapiList = match from_slice(&pokeapi_response) {
        Ok(data) => {
            data
        },
        Err(_e) => {
            PokeapiList {
                count: 0,
                next: None,
                previous: None,
                results: Vec::<Pokemon>::with_capacity(0)
            }
        }
    
    };

    let json_response = PokemonListResponse {
        status: "success".to_string(),
        results: final_resp.results.len(),
        data: final_resp.results
    };

    Ok(Json(json_response))
}
