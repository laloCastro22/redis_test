use std::collections::HashMap;
use crate::dto::{dto_responses::ApiResponse};
extern crate redis;
use redis::{Commands, Connection, RedisResult};

fn conecta_redis() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

pub fn busca_por_filtro() -> Result<ApiResponse<HashMap<i32, String>>, redis::RedisError> {

    let mut con = conecta_redis()?;

    let _: () = redis::cmd("SELECT").arg(5).query(&mut con)?;

    // Obtener todos los elementos de la lista "tipos_de_persona"
    let tipos: Vec<String> = con.lrange("mylist", 0, -1)?;

    // Crear un HashMap que contenga los elementos filtrados con sus índices
    let mut tipos_filtrados: HashMap<i32, String> = HashMap::new();

    // Iterar sobre la lista con índices y filtrar por el valor del filtro
    for (i, tipo) in tipos.into_iter().enumerate() {
        if tipo.contains("valor") {
            tipos_filtrados.insert(i as i32, tipo);
        }
    }

    // Devolver la respuesta
    Ok(ApiResponse::new(
        tipos_filtrados,
        true,
        vec![String::from("Tipos de persona filtrados correctamente")],
    ))
}