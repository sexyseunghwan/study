use crate::common::*;

use crate::repositories::pic_repository::*;


#[async_trait]

pub trait TPicService {
    async fn get_best_seen_pic_infos(&self, pic_cnt: i32) -> Result<(), anyhow::Error>;
    
    
    // test
    async fn set_seen_pic_infos(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    async fn get_seen_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error>;

    async fn set_like_pic_infos(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    async fn get_like_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error>;

    
}

#[derive(Debug)]
pub struct PicService<T: TPicRepository + 'static> {
    pub repository: Arc<T>,
}

#[async_trait]
impl<T: TPicRepository + Sync + Send> TPicService for PicService<T> { 
    
    /*
        
    */
    async fn get_best_seen_pic_infos(&self, pic_cnt: i32) -> Result<(), anyhow::Error> {

        //let pic_vector = self.repository.find_best_seen_pic(pic_cnt).await?;

        Ok(())
    }

    /* ================================================ */
    

    /*
        It stores the corresponding user information and the information of the clicked photo.
    */
    async fn set_seen_pic_infos(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error> {

        // When a specific user clicks on a specific photo, the information is stored in Redis.
        let _ = self.repository.set_zadd_pic("seenPhoto", user_seq_int, pic_seq).await?;

        // Increase the click count of the photo and store the data in Redis.
        let _ = self.repository.set_zincrby_pic("photoSeenCounts" ,pic_seq).await?;
        
        Ok(())
    }


    /*
        Function that determines how many times a particular photo post has been clicked.
    */
    async fn get_seen_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error> {

        let _ = self.repository.get_zscore_pic("photoSeenCounts", pic_seq).await?;

        Ok(())
    }
    
    
    /*
        It stores the corresponding user information and the information of the clicked photo.
    */
    async fn set_like_pic_infos(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error> {

        let like_yn = self.repository.get_zscore_pic(&format!("likePhoto:{}",pic_seq), user_seq_int).await?;

        if like_yn == -1 {
            
            // When a specific user clicks on a specific photo, the information is stored in Redis.
            let _ = self.repository.set_zadd_pic("likePhoto", user_seq_int, pic_seq).await?;

            // Increase the click count of the photo and store the data in Redis.
            let _ = self.repository.set_zincrby_pic("photoLikeCounts" ,pic_seq).await?;
        } 

        Ok(())
    }


    /*
        Function that determines how many times a particular photo post has been clicked.
    */
    async fn get_like_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error> {

        let _ = self.repository.get_zscore_pic("photoLikeCounts", pic_seq).await?;

        Ok(())
    }

    /*
        Function that determines whether a particular user has clicked on the PHOTO.
    */
    // async fn get_seen_pic_infos(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error> {

    //     let _ = self.repository.get_seen_pic(user_seq_int, pic_seq).await?;

    //     Ok(())

    // }



}