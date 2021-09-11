use iron::prelude::*;
use iron::Handler;
use iron::mime::Mime;
use iron::status;
use router::Router;
use serde_json::{Result, Value};
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::database::Database;
use crate::game::{Game, GameLoader};

pub struct API {
    address: String,
    database: Arc<Mutex<Database>>,
}

impl API {
    /**
     * Initializes the API
     */
    pub fn new(database: Arc<Mutex<Database>>, address: String) -> API {
        API {
            address,
            database
        }
    }

    /**
     * Launch an API instance
     * @param self
     */
    pub fn start(&mut self) {
        let mut router = Router::new();
        // Init routes
        let post_game_handler = PostGameHandler {
            database: self.database.clone()
        };
        let search_handler = SearchHandler {
            database: self.database.clone()
        };
        router.post("/game", post_game_handler, "game");
        router.post("/search/:variant", search_handler, "search");
        info!("start API endpoint at {}", self.address);
        // Start router
        Iron::new(router).http(&*self.address).unwrap();
    }
}

struct PostGameHandler {
    database: Arc<Mutex<Database>>,
}

impl Handler for PostGameHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let mut body = String::new();
        request.body.read_to_string(&mut body).unwrap();
        info!("POST /game/ {}", body);

        let v: Value = serde_json::from_str(&body).unwrap();
        self.database.lock().unwrap().add_game(&GameLoader::from_json(v));
        self.database.lock().unwrap().save();
        return Ok(Response::with((content_type, status::Ok, "{}")));
    }
}

struct SearchHandler {
    database: Arc<Mutex<Database>>,
}

impl Handler for SearchHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let mut body = String::new();
        request.body.read_to_string(&mut body).unwrap();
        let variant = request.extensions.get::<Router>().unwrap().find("variant").unwrap_or("");
        info!("POST /search/{} {}", variant, body);

        let v: Value = serde_json::from_str(&body).unwrap();
        let variant = GameLoader::variant_from_string(variant.to_string());
        return Ok(Response::with((content_type, status::Ok, self.database.lock().unwrap().search(variant, v["moves"].as_str().unwrap().to_string()))));
    }
}