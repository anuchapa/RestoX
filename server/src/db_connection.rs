
use postgres::{Client, NoTls};

pub struct DbConnector {
    pub client: Client,
}

impl DbConnector {
    pub fn new(params : &str) -> DbConnector {
        DbConnector { client: Client::connect(params, NoTls).unwrap() }
    }
}



// pub fn connect_db_test(){
//     let mut db_connection = Client::connect("host=localhost user=restox_user password=090770 dbname=restox_db", NoTls).unwrap();
//     let tables = db_connection.query("SELECT * FROM stores;", &[]).unwrap();
//     for table in tables {
//         let id : i32 = table.get(0);
//         let name: &str = table.get(1);
//         println!("{} {}", id ,name);
//     }
// }

