use crate::models::Response;

pub mod store;

pub trait  Controller {
    fn call(&mut self,uri : &str)->Option<String>;
}