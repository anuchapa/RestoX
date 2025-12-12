use crate::{controllers::Controller, models::{Response, store::StoreListResponse}, services::store::{StoreService,TStoreService}};

const LIST_URI: &str = "/store/list";
pub struct StoreController{
    service:StoreService
}

impl StoreController {
    pub fn new(service:StoreService) -> Self{
        StoreController { service }
    }
}

impl Controller for StoreController {
    fn call(&mut self,uri : &str) ->Option<String>{
        println!("{}",uri);
        let response = match uri {
            LIST_URI => {
                let res:Vec<StoreListResponse>= self.service.list();
                Some(serde_json::to_string(&res).unwrap())
            },
            other => {
                println!("unknown method: {}", other);
                None
            },
        };
        response
        
    }
}