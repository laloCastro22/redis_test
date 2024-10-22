mod controller;
mod service;
mod dao;
mod dto;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Cargar las rutas
    let routes = controller::controllers::user_routes();

    // Iniciar el servidor en el puerto 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
