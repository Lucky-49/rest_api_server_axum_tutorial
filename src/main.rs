pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Json, middleware, Router};
use axum::http::{Method, Uri};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

mod error; //Подключает файл error.rs

#[tokio::main] //Атрибут, указывающий, что функция main() будет использовать Tokio runtime
async fn main() -> Result<()> { //Объявление асинхронной функции main(), которая возвращает Result<()>. Это означает, что функция может выполняться асинхронно и может вернуть либо () в случае успеха, либо ошибку типа Result
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

// region: ---Маршруты сервера
    let route_test_server = Router::new().route("/test_server", get(handler_test_server));
    let route_index = Router::new().route("/", get(handler_index));

    // Обслуживание статических файлов из директории `game/app`
    let static_files = get_service(ServeDir::new("game/app"));

    let routes_all = Router::new()
        .merge(route_test_server)
        .merge(route_index)
        .nest_service("/static", static_files)
        .layer(middleware::map_response(main_response_mapper));
    // endregion: ---Маршруты сервера

    // region:   ---Start server
    let listener = TcpListener::bind("127.0.0.1:8080") //Создание TCP-слушателя на порту 8080 для прослушивания входящих соединений
        .await //Ожидание создания слушателя
        .unwrap(); //Этот вызов распаковывает результат операции, возвращая содержимое Result. В случае успешной привязки к порту, возвращается сам TcpListener. В случае ошибки, вызывается panic!, что может привести к завершению программы с выводом сообщения об ошибке

    info!("LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap(); //Запуск сервера Axum, который обрабатывает входящие соединения с помощью определенных маршрутов переменной routes_all
    //endregion: ---Start server

    Ok(())
}

async fn handler_index() -> Html<&'static str> {
    Html(include_str!("../game/app/index.html"))
}

async fn handler_test_server() -> impl IntoResponse {
    debug!("{:<12} - test_server", "HANDLER");

    Html("SERVER WORK")
}

async fn main_response_mapper(res: Response) -> Response {
    debug!("{:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}