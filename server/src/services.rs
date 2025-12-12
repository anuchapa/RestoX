use crate::DbConnector;

pub mod store;

pub trait  Service {
    fn new(connector: DbConnector) -> Self;
}