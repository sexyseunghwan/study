// use crate::common::*;

// use crate::util_mod::jwt_utils::check_user_jwt_infos;

// #[macro_export]
// macro_rules! with_auth_async {

//     use crate::common::*;

//     use crate::util_mod::jwt_utils::check_user_jwt_infos;

//     ($handler:expr) => {
//         |req: actix_web::HttpRequest| {
//             Box::pin(async move {
//                 // 여기에 인증 로직 구현, 예를 들어:
//                 match check_user_jwt_infos(req).await {
//                     Ok((user_seq, re_token)) => {
//                         // 인증 성공, 핸들러 함수 실행
//                         $handler(req).await
//                     },
//                     Err(e) => {
//                         // 인증 실패, 에러 반환
//                         Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
//                     }
//                 }
//             })
//         }
//     };
// }
