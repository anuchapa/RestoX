use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct StoreListResponse{
    pub id : i32,
    pub name : String
}

impl Response for Vec<StoreListResponse>  {
    
}