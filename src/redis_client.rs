// use once_cell::sync::OnceCell;
// use redis::{Client, Connection};
// use std::env;

// static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

// pub fn connect_redis() -> Connection {
//     match REDIS_CLIENT.get() {
//         None => {
//             let redis_url = env::var("REDIS_URL").unwrap_or("redis://localhost/0".to_string());
//             let client = Client::open(redis_url).unwrap();
//             let connect = client.get_connection().unwrap();
//             REDIS_CLIENT.set(client).unwrap();
//             connect
//         }
//         Some(client) => client.get_connection().unwrap(),
//     }
// }
