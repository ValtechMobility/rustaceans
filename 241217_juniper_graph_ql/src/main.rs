mod graphql_schema;

use axum::{
    http::Response,
    response::IntoResponse,
    routing::get,
    routing::{on, MethodFilter},
    Extension, Json, Router,
};
use graphql_schema::{create_schema, Schema};
use juniper::graphiql;
use juniper::http::GraphQLRequest;
use juniper_axum::{extract::JuniperRequest, graphiql, playground, response::JuniperResponse};
use std::sync::Arc;

async fn reply() -> Result<String, String> {
    Ok("Hello Valtech".to_string())
}

async fn get_heroes(
    Extension(schema): Extension<Arc<Schema>>,
    Json(request): Json<GraphQLRequest>,
) -> Result<(), String> {
    let res = request.execute(&schema, &());
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res).unwrap()).unwrap();
    Ok(())
}

async fn custom_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&schema, &()).await)
}

#[tokio::main]
async fn main() {
    println!("Starting server!");

    let schema = Arc::new(create_schema());

    let app = Router::new()
        .route("/", get(reply))
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql),
        )
        .route("/heroes", get(get_heroes))
        .route("/graphiql", get(graphiql("/graphql", "/heroes")))
        .layer(Extension(Arc::new(schema)));

    println!("Running server on 127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
