use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::{json, Value};

pub fn build_router() -> Router {
    let r = Router::new()
        .route("/health", get(health))
        // .route("/query-rooms", get(query_rooms))
        // .route("/create-room", post(create_room))
        // .route("/check-join", post(check_join))
        // .route("/update-status", post(update_status))
        ;

    r
}

fn response_result<T>(code: i32, result: T) -> Json<Value>
where
    T: Serialize,
{
    let v = json!({"code": code, "result": result});
    Json(v)
}

fn response_success() -> Json<Value> {
    response_result(1, "success".to_string())
}

fn response_failed() -> Json<Value> {
    response_result(0, "failed".to_string())
}

async fn health() -> impl IntoResponse {
    Json("Health")
}

// async fn query_rooms() -> impl IntoResponse {
//     let db = DB.lock().await;
//     let v = json!(*db);
//     response_result(1, v)
// }

// async fn create_room(Json(body): Json<Value>) -> impl IntoResponse {
//     let mut room_map = STATE_MAP.lock().await;
//     let r_name = body.get("room").unwrap().as_str().unwrap();
//     if room_map.contains_key(r_name) {
//         return response_result(0, format!("Room: {} has been existed", r_name));
//     }

//     let gm = body.get("gm").unwrap().as_str().unwrap();

//     let room_info = RoomInfo {
//         use_number: 0,
//         max_number: body.get("maxNumber").unwrap().as_i64().unwrap() as i32,
//         status: 0,
//         width: body.get("worldWidth").unwrap().as_i64().unwrap() as i32,
//         players: Vec::new(),
//         gm: gm.to_string(),
//     };

//     db::save_room_info(r_name, &room_info).await;

//     let s = State::from_info(r_name.to_string(), &room_info);
//     let s = Arc::new(Mutex::new(s));

//     tokio::task::spawn(run_state(s.clone()));
//     room_map.insert(r_name.to_string(), s.clone());

//     response_failed()
// }

// async fn check_join(Json(body): Json<Value>) -> impl IntoResponse {
//     let room_map = STATE_MAP.lock().await;
//     let room_name = body.get("room").unwrap().as_str().unwrap();

//     match room_map.get(room_name) {
//         None => response_failed(),
//         Some(room) => {
//             let name = body.get("name").unwrap().as_str().unwrap();
//             let result = room.lock().await.can_join(name);
//             response_result(result, result)
//         }
//     }
// }

// async fn update_status(Json(body): Json<Value>) -> impl IntoResponse {
//     let room_map = STATE_MAP.lock().await;
//     let room_name = body.get("room").unwrap().as_str().unwrap();

//     if let Some(r) = room_map.get(room_name) {
//         let mut room = r.lock().await;

//         let gm = body.get("gm").unwrap().as_str().unwrap();

//         let status = body.get("status").unwrap().as_i64().unwrap();
//         room.status = status as i32;
//     }

//     response_success()
// }
