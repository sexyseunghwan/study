use crate::common::*;

use crate::repositories::pic_repository::*;
use crate::services::auth_services::*;

use crate::repositories::auth_repository::*;
use crate::services::pic_service::*;

/*

*/
#[derive(Debug, Getters, Setters)]
#[getset(get = "pub")]
pub struct DIContainer {
    
    // 1. Auth Object
    auth_service: Arc<AuthService<AuthRepository>>,
    
    // 2. Pic Object
    pic_service: Arc<PicService<PicRepository>>
    
}

impl DIContainer {

    /*
        Dependent objects to be injected can be added directly below.
    */
    pub fn new(db_datas: web::Data<DbState>) -> Self {
        
        // 1. Auth Object
        let auth_repository:Arc<AuthRepository> = Arc::new(AuthRepository::new(&db_datas));
        let auth_service = Arc::new(AuthService {repository : auth_repository});

        // 2. Pic Object
        let pic_repository:Arc<PicRepository> = Arc::new(PicRepository::new(&db_datas));
        let pic_service = Arc::new(PicService {repository : pic_repository});
        
        DIContainer {
            auth_service,
            pic_service
        }
    }
    
}