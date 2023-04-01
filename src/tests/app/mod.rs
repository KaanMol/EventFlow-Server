use crate::{
    app::{routes::ping, AppState},
    handlers::{self},
};

use actix_web::{
    http::{self, header::ContentType},
    test,
    web::{self, Bytes},
    App,
};

mod users;

#[actix_web::test]
async fn test_ping_ok() {
    let app = test::init_service(App::new().service(crate::app::routes::ping)).await;
    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_and_read_body(&app, req).await;

    assert_eq!(resp, Bytes::from_static(b"pong"));
}
