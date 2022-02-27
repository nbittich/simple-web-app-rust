use axum::{
    extract::Form,
    response::Html,
    routing::{get, post, MethodRouter},
    Router,
};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use simple_web_app_rust::{schema::users::dsl::*, user::*};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::net::SocketAddr;
#[derive(Serialize)]
pub struct Page {
    title: String,
    content: String,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    setup_tracing();

    let env = include_html();
    let db = simple_web_app_rust::db::get_db();

    let addr = std::env::var("ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8080"));

    let socket_addr: SocketAddr = addr.parse().expect("unable to parse socket address");

    let app = Router::new()
        .merge(root(env.clone()))
        .merge(subscribe(env.clone(), db.clone()))
        .merge(users_list(env.clone(), db.clone()))
        .merge(next(env.clone()));

    tracing::info!("listening on {:?}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument(skip_all)]
fn root(env: Environment<'static>) -> Router {
    route(
        "/",
        get(|| async move {
            tracing::info!("Home route entered!");
            let template = env.get_template("home.html").unwrap();
            let page = Page {
                title: "Some title".into(),
                content: "Lorum Ipsum".into(),
                message: None,
            };
            let html = template.render(context!(page)).unwrap();
            Html(html)
        }),
    )
}

#[tracing::instrument(skip_all)]
fn subscribe(env: Environment<'static>, db: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    route(
        "/subscribe",
        post(|Form(input): Form<Input>| async move {
            tracing::info!("Subscribe route entered!");
            let db_connection = db.get().unwrap();
            let new_user = UserNew {
                email: input.email,
                password: input.password,
                date_created: format!("{}", chrono::Local::now().naive_local()),
            };
            insert_into(users)
                .values(&new_user)
                .execute(&db_connection)
                .expect("Error");
            let result: User = users.order(id.desc()).first(&db_connection).unwrap();
            let msg = format!(
                "User with email {} and id {} inserted",
                result.email, result.id
            );

            let template = env.get_template("home.html").unwrap();
            let page = Page {
                title: "Some title".into(),
                content: "Lorum Ipsum".into(),
                message: Some(msg),
            };
            let html = template.render(context!(page)).unwrap();
            Html(html)
        }),
    )
}

#[tracing::instrument(skip_all)]
fn next(env: Environment<'static>) -> Router {
    route(
        "/next",
        get(|| async move {
            tracing::info!("Next route entered!");
            let template = env.get_template("next.html").unwrap();
            let page = Page {
                title: "Subscribe page".into(),
                content: "Subscribe to our great website".into(),
                message: None,
            };
            let html = template.render(context!(page)).unwrap();
            Html(html)
        }),
    )
}

#[tracing::instrument(skip_all)]
fn users_list(env: Environment<'static>, db: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    route(
        "/users",
        get(|| async move {
            tracing::info!("Users list route entered!");
            let db_connection = db.get().unwrap();
            let all_users = users.load::<User>(&db_connection).unwrap();
            let template = env.get_template("table.html").unwrap();
            let page = Page {
                title: "List of users".into(),
                content: "a place to see all users".into(),
                message: None,
            };
            let html = template
                .render(context! {
                    page => page,
                    users => all_users
                })
                .unwrap();
            Html(html)
        }),
    )
}

fn include_html<'a>() -> Environment<'a> {
    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("./html/layout.html"))
        .unwrap();
    env.add_template("home.html", include_str!("./html/home.html"))
        .unwrap();
    env.add_template("next.html", include_str!("./html/next.html"))
        .unwrap();
    env.add_template("table.html", include_str!("./html/table.html"))
        .unwrap();
    env
}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}