/**
Send: Indica que T debe ser seguro para transferirse entre hilos.
      En Rust, Send es un marcador que asegura que un tipo puede ser enviado a otro hilo de ejecución de manera segura.
Sync: Indica que T debe ser seguro para compartir entre múltiples hilos.
      En Rust, Sync es un marcador que asegura que un tipo puede ser compartido entre múltiples hilos de ejecución de manera segura.
'static: Indica que T debe tener una duración estática.
          En Rust, 'static es un tiempo de vida especial que significa que el objeto al que se aplica tiene una duración estática.
**/

pub async fn get_list<T, A, B>(pager: Query<PagerQuery<A, B>>, req: Request, ) -> Response
where
    T: CatalogItem + Send + Sync + 'static,
    A: CatalogFilter + Into<FilterType> + Send,
    B: CatalogSort + Send,
{
    log::info!("{}", req.address());

    let pager = pager.into_inner();

    let mut query = T::query();


    let mut all_filters: Vec<A> = vec![];
    all_filters.push(A::by_activo(true));
    all_filters.push(A::by_fecha_inicio(Utc::now().date_naive()));


    if let Some(filters) = pager.filters {
        let mut front_filters: Vec<A> = filters
            .into_iter()
            .filter_map(|f| f.into_filter())
            .collect();
        all_filters.append(&mut front_filters);
    }

    query = query.filters(all_filters.clone()).sort(vec![pager.sort.unwrap_or_else(B::ByNombreAsc)]);

    match query.execute().await {
        Ok(elements) => {
            match serde_json::to_string(&ResultOperation {
                result: elements
                    .iter()
                    .map(|item| (item.id().as_option().unwrap(), item.nombre().clone()))
                    .collect::<IndexMap<i32, String>>(),
                messages: vec![],
                success: true,
            }) {
                Ok(content) => {
                    Response::ok().header("Content-Type", "application/json").body(content)
                }
                Err(e) => {
                    log::error!("{}", e);
                    Response::internal_server_error()
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
            Response::internal_server_error()
        }
    }
}
