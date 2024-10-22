use serde::Serialize;
#[derive(Serialize)]
pub(crate) struct ApiResponse<T> {
    pub(crate) result: T,
    pub(crate) success: bool,
    pub(crate) message: Vec<String>,
}

impl<T> ApiResponse<T> {
    // Constructor para crear una nueva respuesta genérica
    pub(crate) fn new(result: T, success: bool, message: Vec<String>) -> ApiResponse<T> {
        ApiResponse {
            result,
            success,
            message,
        }
    }
}