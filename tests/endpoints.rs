use actix_web::{test, App};
use parser::greet;

#[actix_web::test]
async fn get_hello() {
    let app = test::init_service(App::new().service(greet)).await;
    let req = test::TestRequest::get()
        .uri("/hello/test_name")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(test::read_body(resp).await, "Hello test_name!");
}
