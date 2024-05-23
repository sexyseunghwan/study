use crate::common::*;

use crate::dto::test_dtos::*;

/*
    Route configure
*/
pub fn config(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/test")
        .route("/get", web::get().to(get_test))
        .route("/posttext", web::post().to(post_text))
        .route("/postjson", web::post().to(post_json))
    );
}

/*
    GET TEST
*/
async fn get_test(query: web::Query<QueryInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("Received: {}, {}", query.param1(), query.param2()))
}

/*
    POST TEST
*/
async fn post_json(info: web::Json<QueryInfo>) -> impl Responder {
    let response = format!("Received: {}, {}", info.param1(), info.param2());

    HttpResponse::Ok().body(response)
}


/*

*/
async fn post_text(body: String) -> impl Responder {

    println!("Received text: {}", body);

    HttpResponse::Ok().body(format!("Received text: {:?}", body))

}