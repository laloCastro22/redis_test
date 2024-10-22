use std::collections::HashMap;
use redis::RedisError;
use crate::dao::user_dao;
use crate::dto::dto_responses::ApiResponse;

// Definir los servicios relacionados con usuarios
pub fn obten_usuario_tipo() -> Result<ApiResponse<HashMap<i32, String>>, RedisError> {
    user_dao::busca_por_filtro()
}
