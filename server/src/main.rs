use server::{DbConnector, SimpleServer, controllers::store::StoreController, services::{Service, store::StoreService}};
fn main() {
    let connector = DbConnector::new("host=localhost user=restox_user password=090770 dbname=restox_db");
    let store_service = StoreService::new(connector);
    let store_controller = Box::new(StoreController::new(store_service));
    let mut server = SimpleServer::new("7878");
    server.controllers.push(store_controller);
    server.start_server();
}





