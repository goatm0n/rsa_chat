use warp::Filter;


#[tokio::main]
async fn main() {
    let db = models::blank_db();
    let api = filters::messages(db);
    let routes = api.with(warp::log("messages"));
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}

mod filters {
    use super::handlers;
    use super::models::{Db, Message};
    use warp::Filter;

    pub fn messages(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        messages_list(db.clone())
            .or(messages_create(db.clone()))
    }

    /// GET /messages 
    pub fn messages_list(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("messages")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::list_messages)
    }

    /// POST /messages with json body
    pub fn messages_create(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("messages")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_message)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (Message,), Error = warp::Rejection> + Clone {
        // when accepting a body, we want a json body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

}

mod handlers {
    use super::models::{Db, Message};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn list_messages(db: Db) -> Result<impl warp::Reply, Infallible> {
        let messages = db.lock().await;
        let messages: Vec<Message> = messages
            .clone()
            .into_iter()
            .collect();
        Ok(warp::reply::json(&messages))
    }

    pub async fn create_message(message: Message, db: Db) -> Result<impl warp::Reply, Infallible> {
        let mut vec = db.lock().await;

        vec.push(message);

        Ok(StatusCode::CREATED)
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Message>>>;

    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }
    
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Message {
        pub text: String,
    }

}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use super::filters;
    use super::models::{blank_db, Message};

    #[tokio::test]
    async fn test_post() {
        let db = blank_db();
        let api = filters::messages(db);

        let resp = request()
            .method("POST")
            .path("/messages")
            .json(&message1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    fn message1() -> Message {
        Message {
            text: "This is the message".into(),
        }
    }

}








