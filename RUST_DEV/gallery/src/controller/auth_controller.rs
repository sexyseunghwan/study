use crate::common::*;

use crate::middleware::*;

use crate::dicontainer::DIContainer;
use crate::dto::login_dtos::*;

use crate::services::auth_services::*;

use crate::util_mod::cookies_utils::*;




/*
    Route Login Controller configure
*/
pub fn config(cfg: &mut web::ServiceConfig) {
    
    /*
        Routes specific services into urls through the "route()" function.
        - Jwt-related middleware execution required
    */
    let auth_scope = web::scope("/login")
        .wrap(JwtMiddleware)
        .route("/user", web::post().to(login))
        .route("/checkuser", web::post().to(check_user))
        .route("/userinfos", web::get().to(get_user_infos));
        
    
    /*
        No need to run jwt related middleware
    */
    let login_scope = web::scope("/logins")
        .route("/main", web::post().to(main_user_login))
        .route("/joinuser", web::post().to(join_user))
        .route("/test", web::post().to(testing))
        .route("/mainInjection", web::post().to(main_user_login_injection));
        
        
    
    cfg.service(auth_scope);
    cfg.service(login_scope);
}


/*
    test
*/
async fn get_user_infos(di_container: web::Data<DIContainer>, req: HttpRequest) -> impl Responder {

    let auth_service = di_container.auth_service().as_ref();
    let user_seq = get_cookie_from_http(&req, "user_seq_num");
    
    println!("{}", user_seq);
    
    let user_infos = match auth_service.get_user_infos(&user_seq).await {
        Ok(user_infos) => user_infos,
        Err(e) => {
            errors(&e).await;
            return HttpResponse::Unauthorized().finish();
        }
    };
    
    println!("{:?}", user_infos);

    HttpResponse::Ok().json(user_infos)
}


/*
    TEST Handler
*/
async fn testing(di_container: web::Data<DIContainer>) -> impl Responder {

    let auth_service = di_container.auth_service().as_ref();

    // let mut redis_conn: redis::cluster_async::ClusterConnection = get_global_redis_conn().await.unwrap();
    
    // let result: Option<String> = redis_conn.get("jwt_user::2").await.unwrap();

    // println!("{:?}", result);

    HttpResponse::Ok().finish()
}


/*
    test
*/
async fn main_user_login_injection(di_container: web::Data<DIContainer>, login_request: web::Json<UserLoginInput>) -> impl Responder {
    
    let auth_service = di_container.auth_service().as_ref();

    match auth_service.login_user_injection(login_request.user_id(), login_request.user_pw()).await {
        Ok(user_data) => HttpResponse::Ok().json(user_data), 
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::Unauthorized().finish()
        },
    }
}



/*
    Main Login Handler
    - Issue jwt to the client.
*/
async fn main_user_login(di_container: web::Data<DIContainer>, login_request: web::Json<UserLoginInput>) -> impl Responder {
    
    let auth_service = di_container.auth_service().as_ref();
    
    // 1. Check user information. - user_id, user_pw
    let user_seq = match auth_service.verify_user_info(login_request.user_id(), login_request.user_pw()).await {
        Ok(user_seq) => {
            user_seq
        },
        Err(e) => {
            errors(&e).await;
            return HttpResponse::Unauthorized().finish();
        }
    };
    
    // 2. Issuing JWT to the client.
    let tokens = match auth_service.set_user_jwt_token(user_seq).await {
        Ok(tokens) => tokens,
        Err(e) => {
            errors(&e).await;
            return HttpResponse::Unauthorized().finish();
        }
    };

    let access_token_cookie = tokens.0;
    let refresh_token_cookie = tokens.1;
    let user_seq_num = tokens.2;

    // 3. Save JWT to cookies.
    HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .cookie(user_seq_num)
        .finish()

}


/*
    Function that verifies the user's ID and password
*/
async fn login(di_container: web::Data<DIContainer>, login_request: web::Json<UserLoginInput>, http_req: HttpRequest) -> impl Responder {
    
    let auth_service = di_container.auth_service().as_ref();

    HttpResponse::Ok()
    // match auth_service.login_user(login_request.user_id(), login_request.user_pw()).await {
    //     Ok(token) => HttpResponse::Ok().json(token), 
    //     Err(e) => {
    //         error!("{:?}", e);
    //         HttpResponse::Unauthorized().finish()
    //     },
    // }
}


/*
    Function to sign up for a new user.
*/
async fn join_user(di_container: web::Data<DIContainer>, login_request: web::Json<UserLoginInput>) -> impl Responder {

    let auth_service = di_container.auth_service().as_ref();

    match auth_service.join_membership(login_request.user_id(), login_request.user_pw()).await {
        Ok(_) => HttpResponse::Ok().json("success"), 
        Err(e) => {
            errors(&e).await;
            HttpResponse::Unauthorized().finish()
        }
    }
}


/*
    TEST
*/
async fn check_user(di_container: web::Data<DIContainer>, req: HttpRequest) -> impl Responder {
    
    let auth_service = di_container.auth_service().as_ref();
    
    let access_token = get_cookie_from_http(&req, "access_token");
    let refresh_token = get_cookie_from_http(&req, "refresh_token");
    let user_seq_num = get_cookie_from_http(&req, "user_seq_num");


    HttpResponse::Ok()

    // let token = match body.get("jwt").and_then(Value::as_str) {
    //     Some(token) => {
    //         println!("{:?}",token);
    //         token
    //     },
    //     None => {
    //         return HttpResponse::Unauthorized().finish()
    //     }
    // };

    // match auth_service.verify_user_token(token).await {
    //     Ok(res) => HttpResponse::Ok().json(res), 
    //     Err(e) => {
    //         error!("{:?}", e);
    //         HttpResponse::Unauthorized().finish()
    //     }
    // }
}


