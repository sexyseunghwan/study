use crate::common::*;

/*
    Function that initializes global environment variables.
*/
// pub fn init_comp_env() {

//     // Set compilation environment
//     dotenv().ok();

//     // 1. SSH key configuration settings
//     set_env_var(&SSH_ENV, "SSH_ENV");

//     // 2. RDB Environment Settings
//     set_env_var(&DATABASE_URL, "DATABASE_URL");
    
// }


// /*
//     Function that initializes a particular global environment variable.
// */
// fn set_env_var(env_var: &Lazy<RwLock<String>>, env_name: &str) {

//     let mut env_lock = match env_var.write() {
//         Ok(env_lock) => env_lock,
//         Err(e) => {
//             error!("{:?}", e);
//             panic!("{:?}", e);
//         }
//     };

//     let env_val = env::var(env_name).expect("Compile type must be set");
//     *env_lock = env_val.clone();
// }


// /*
//     Function that returns global environment variables.
// */
// pub fn get_env_var(env_var: &Lazy<RwLock<String>>) -> Result<String, anyhow::Error> {

//     let env_lock = env_var.read().unwrap();
//     let init_var = env_lock.clone();

//     Ok(init_var)
// }