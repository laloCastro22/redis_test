pub async fn sentido_juicio_amparo_jn_get_list(  pager: Query<PagerQuery<BasicCatalogFilter, CatalogSort>>, req:Request) -> Response  {



    //T: Tail, C: Enum
    log::info!("{}", req.address());
    let pager= pager.into_inner();
    let mut query = SentidoJuicioAmparoJuicio::query();


    let mut all_filters : Vec<CatalogFilter> = vec![];

    all_filters.push(CatalogFilter::ByActivo(true));
    all_filters.push(CatalogFilter::ByFechaInicio(Utc::now().date_naive()));

    if let Some(filters) = pager.filters {
        let mut front_filters: Vec<CatalogFilter> = filters.into_iter()
                                                            .filter_map(|f| f.into_filter())
                                                            .collect(); //parseo los filtros permitidos
        all_filters.append(&mut front_filters);
    }
    query = query.filters(all_filters.clone()).sort(vec![pager.sort.unwrap_or(CatalogSort::ByNombreAsc)]);

    match query.execute().await {
        Ok(elements) => {
            match serde_json::to_string(&ResultOperation{
                result: elements.iter()
                    .map(|sentido| (sentido.id.as_option().unwrap() , sentido.nombre.clone() ))
                    .collect::<IndexMap<i32, String>>(),
                messages: vec![],
                success: true
            }) {
                Ok(content) => {
                    Response::ok().header("Content-Type", "application/json").body(content)
                },
                Err(e) => {
                    log::error!("{}", e);
                    Response::internal_server_error()
                }
            }

        },
        Err(e) => {
            log::error!("{}", e);
            Response::internal_server_error()
        }
    }

}
