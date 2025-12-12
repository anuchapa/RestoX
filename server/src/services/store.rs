use crate::{DbConnector, models::{Response, store::StoreListResponse}, services::Service};



pub trait TStoreService{
    fn create(&mut self,name : &str);
    fn rename(&mut self,id:usize , new_name:&str);
    fn delete(&mut self,id:usize);
    fn list(&mut self)->Vec<StoreListResponse>;
}

pub struct StoreService{
    connector:DbConnector
}

impl Service for StoreService {
    fn new(connector: DbConnector) -> Self{
        StoreService{connector}
    }
}

impl TStoreService for StoreService {
    
    fn create(&mut self,name : &str) {
        
    }

    fn rename(&mut self,id:usize , new_name:&str) {
        
    }

    fn delete(&mut self,id:usize) {
        
    }

    fn list(&mut self) ->Vec<StoreListResponse> {
        println!("List functoin.");
        let tables =  self.connector.client.query("SELECT * FROM stores;", &[]).unwrap();
        let mut store_list:Vec<StoreListResponse> = Vec::new();
        for table in tables {
            let id : i32 = table.get(0);
            let name: String = table.get(1);
            store_list.push(StoreListResponse { id: id, name: name });
            // println!("{} {}", id ,name);
        }
        store_list
    }
}

