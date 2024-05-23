use crate::common::*;

use crate::util_mod::jwt_utils::*;
use crate::util_mod::cookies_utils::*;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    // Polling if the service is ready to process the request.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    
    // Middleware execution function.
    fn call(&self, req: ServiceRequest) -> Self::Future {

        println!("1");

        // Include "user_seq_num" in the request.
        let user_seq_num = match get_cookie(&req, "user_seq_num").parse::<i32>() {
            Ok(user_seq_num) => user_seq_num,
            Err(_) => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorBadRequest("Error occurred while converting the variable 'user_seq_num' to type i32."))
                });
            }
        };
        
        println!("2");
        println!("{}", user_seq_num);

        // Exception handling if "user_seq_num" is not present within the client Cookie.
        if user_seq_num == -1 {
                        
            return Box::pin(async move {
                Err(actix_web::error::ErrorBadRequest("Invalid user sequence number"))
            });
        }
        
        let user_seq_num_move = 6;
        
        // Look up "access_token", "refresh_token" in client COOKIES.
        let access_token = get_cookie(&req, "access_token");
        let refresh_token = get_cookie(&req, "refresh_token");
        
        println!("{}",access_token);
        println!("{}",refresh_token);

        println!("3");

        // Exception handling if "access_token","refresh_token" is not present within the client Cookie.
        if access_token == String::from("-1") || refresh_token == String::from("-1") {
            
            return Box::pin(async move {
                Err(actix_web::error::ErrorBadRequest("Invalid access_token / refresh_token"))
            });
            
        }

        println!("4");
        //req.extensions_mut().insert(user_seq_num);

        let fut = self.service.call(req);
        
        Box::pin(async move {
            
            println!("5");

            match check_user_jwt_infos(&access_token, &refresh_token, user_seq_num_move).await {
                
                Ok(token) => {
                    println!("6");       
                    let mut res = fut.await?;
                    let token_valid_yn = *token.token_is_valid();
                    let re_access_token = token.access_token().clone().unwrap_or(String::from(""));
                    
                    if token_valid_yn == false {
                        
                        let cookie = 
                            set_cookie_per_mins(String::from("access_token"), re_access_token, 10);
                        
                        let seq_cookie = set_cookie_per_days(String::from("user_seq_num"), user_seq_num_move.to_string(), 30);

                        res.response_mut().add_cookie(&cookie)?;
                        res.response_mut().add_cookie(&seq_cookie)?;
                    }
                    
                    
                    Ok(res)
                },
                Err(e) => {
                    println!("7");    
                    errors(&e).await;
                    Err(e) 
                }
            }
           
        })


    }
}