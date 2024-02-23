use crate::models::pokemon::Pokemon;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PokemonListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<Pokemon>,
}
