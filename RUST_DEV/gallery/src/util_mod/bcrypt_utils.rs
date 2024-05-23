use crate::common::*;



/*

*/
pub async fn encryption_by_hash(password: &str) -> Result<String, anyhow::Error> {

    let password_move = password.to_string();

    let pass_by_hashed = task::spawn_blocking(move|| {

            hash(password_move, DEFAULT_COST)

        }).await?
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?; 

    Ok(pass_by_hashed)
}

/*

*/
pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, anyhow::Error> {
    
    let password_move = password.to_string();
    let hashed_password_move = hashed_password.to_string();
    
    let res = task::spawn_blocking(move|| {

        verify(password_move, &hashed_password_move)
        
    }).await?
    .map_err(|e| anyhow!("Verification failed: {}", e))?; 

    Ok(res)
}

/*

*/
// pub fn check_hash_comparison(input_psw: &str, from_db_pw: &str) -> Result<bool, anyhow::Error> {


//     match hash(input_psw, DEFAULT_COST) {
//         Ok(hashed) => {
//             println!("Hashed password: {}", hashed);
            
//             // 저장된 해시와 입력받은 비밀번호가 일치하는지 검증합니다.
//             match verify(from_db_pw, &hashed) {
//                 Ok(matched) => {
//                     if matched {
//                         println!("Password match!");
//                     } else {
//                         println!("Password does not match!");
//                     }
//                 },
//                 Err(e) => println!("Verify error: {}", e),
//             }
//         },
//         Err(e) => println!("Hash error: {}", e),
//     }


//     Ok(true)
// }