use warp::Filter;
use crate::service::service_user as user_service;
use crate::dto::dto_responses::ApiResponse;
use warp::Rejection;  // Asegúrate de importar esto para el manejo de errores

// Handler para la ruta de usuarios
pub async fn get_users_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let users = user_service::obten_usuario_tipo();

    // Retorna un Ok con la respuesta JSON
    Ok(warp::reply::json(&users.unwrap()))
}

// Definir la ruta usando `.and_then()` que maneja errores
pub fn user_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users").and(warp::get()).and_then(get_users_handler)
}
