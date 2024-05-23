use crate::common::*;

use crate::middleware::*;

use crate::dicontainer::DIContainer;

use crate::services::pic_service::*;

use crate::dto::pic_dtos::*;


//.wrap(JwtMiddleware)
// .route("/bestSeen", web::get().to(screen_best_seen))
// .route("/bestLike", web::get().to(screen_best_like))
// .route("/seenTest", web::post().to(seen_test))


/* 
    Route Photo Controller configure
*/
pub fn config(cfg: &mut web::ServiceConfig) {
    
    /*
        Routes specific services into urls through the "route()" function.
        - Jwt-related middleware execution required
    */
    let pic_scope = web::scope("/screen")
        .wrap(JwtMiddleware)
        .route("/likeTest", web::post().to(like_test));
    
    cfg.service(pic_scope);
}


/*
    Test Controller
*/
async fn seen_test(di_container: web::Data<DIContainer>, json_request: web::Json<BestSeenInput>) -> impl Responder {
    
    let pic_service = di_container.pic_service().as_ref();
    
    // When a user presses a specific photo, Save the information that the user clicked on the PHOTO in REDIS.
    let _ = match pic_service.set_seen_pic_infos(*json_request.user_seq(), *json_request.pic_seq()).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}",err);
            return HttpResponse::Unauthorized().finish();
        } 
    };
    
    // It checks how many times a particular photo post has been clicked.
    let _ = match pic_service.get_seen_pic_count(*json_request.pic_seq()).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}",err);
            return HttpResponse::Unauthorized().finish();
        } 
    };

    HttpResponse::Ok().json("") 
}


/*
    Test Controller
*/
async fn like_test(di_container: web::Data<DIContainer>, json_request: web::Json<BestSeenInput>) -> impl Responder {

    let pic_service = di_container.pic_service().as_ref();
    
    let _ = match pic_service.set_like_pic_infos(*json_request.user_seq(), *json_request.pic_seq()).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}",err);
            return HttpResponse::Unauthorized().finish();
        } 
    };
    

    let _ = match pic_service.get_like_pic_count(*json_request.pic_seq()).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}",err);
            return HttpResponse::Unauthorized().finish();
        } 
    };


    HttpResponse::Ok().json("Like Success")

}


// Function that determines whether a particular user has clicked on a particular photo.
// let _ = match pic_service.get_seen_pic_infos(*json_request.user_seq(), *json_request.pic_seq()).await {
//     Ok(_) => (),
//     Err(err) => {
//         error!("{:?}",err);
//         return HttpResponse::Unauthorized().finish();
//     } 
// };

/*

*/
async fn screen_best_seen(di_container: web::Data<DIContainer>) -> impl Responder {
    
    let pic_service = di_container.pic_service().as_ref();
     
    // 래디스에서 사람들이 가장 많이 본 사진 게시물을 가져와준다.
    // let best_seen = auth_service.get_best_seen_pic_infos().await {

    // }

    HttpResponse::Ok()

}

/*

*/
async fn screen_best_like(di_container: web::Data<DIContainer>, req: HttpRequest) -> impl Responder {

    let pic_service = di_container.pic_service().as_ref();
    
    // 래디스에서 사람들이 가장 많이 좋아요 누른 사진 게시물을 가져온다.

    
    HttpResponse::Ok()
}