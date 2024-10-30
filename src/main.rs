use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use sqlx::PgPool;

mod db;
mod schema;

use crate::schema::{Context, create_schema, Schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(pool: Data<PgPool>, st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let context = Context { pool: pool.get_ref().clone() };
    let user = data.execute(&st, &(context)).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  log::info!("starting HTTP server on port 8080");
  log::info!("GraphiQL playground: http://localhost:8080/graphiql");

  // Create Juniper schema
  let schema = Arc::new(create_schema());
  // Create the database pool
  let pool = db::create_pool().await.expect("Failed to create pool");

  // Start HTTP server
  HttpServer::new(move || {
      App::new()
          .app_data(Data::new(pool.clone()))  // share pool with each request
          .app_data(Data::from(schema.clone()))  // sahre schema with each request
          .service(graphql)
          .service(graphql_playground)
          // the graphiql UI requires CORS to be enabled
          .wrap(Cors::permissive())
          .wrap(middleware::Logger::default())
  })
  .workers(5)
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}