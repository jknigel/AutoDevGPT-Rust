use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_task(&self, task_id: u64) -> Option<&Task> {
        return self.tasks.get(&task_id)
    }

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete_task(&mut self, task_id: u64) {
        self.tasks.remove(&task_id);
    }

    fn update_task(&mut self, updated_task: Task) {
        self.tasks.insert(updated_task.id, updated_task);
    }

    //USER DATA RELATED FUNCTIONS

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }

    /*fn get_user_by_id(&self, user_id: u64) -> Option<&User> {
        self.users.get(&user_id)
    }*/

    //DATABASE SAVING
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data= serde_json::to_string(&self)?;
        let mut file:fs::File = fs::File::create(filename)?;
        file.write_all(data.as_bytes())?;
        return Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data:String = fs::read_to_string(filename)?;
        let db:Database = serde_json::from_str(&data)?;
        return Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
    //http_client: HttpClient,
}

async fn create_task(app_state: web::Data<AppState>, new_task: web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_task(new_task.into_inner());
    let _ = db.save_to_file("database.json");
    return HttpResponse::Ok().finish()
}

async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_task(id.into_inner()) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks: Vec<&Task> = db.get_all_tasks();
    return HttpResponse::Ok().json(tasks)
}

async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_task(task.into_inner());
    let _ = db.save_to_file("database.json");
    return HttpResponse::Ok().finish()
}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete_task(id.into_inner());
    let _ = db.save_to_file("database.json");
    return HttpResponse::Ok().finish()
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file("database.json");
    return HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db:std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            return HttpResponse::Ok().body("Logged in successfully!")
        },
        _ => return HttpResponse::BadRequest().body("Invalid username or password!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::load_from_file("database.json") {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(AppState {
        db: Mutex::new(db),
        //http_client: HttpClient::new(),
    });

    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_origin_fn(|origin, _req_head|{
                origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(data.clone())
            .wrap(cors)
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(read_all_tasks))
            .route("/tasks", web::put().to(update_task))
            .route("/tasks/{id}", web::get().to(read_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run().await
}