#[macro_use]
extern crate rocket;

use std::string::ToString;
use rocket::{Data, Request, Response};
use rand::seq::SliceRandom;
use rocket::data::FromData;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::futures::AsyncReadExt;
use rocket::http::{Header, Status};
use rocket::serde::json::{Json, serde_json};
use rocket::yansi::Paint;
use rocket_cors::{AllowedOrigins, CorsOptions, Method};
use serde::__private::de::IdentifierDeserializer;
use std::sync::{Arc, Mutex, Once};

#[derive(Debug, serde::Serialize, Clone,serde::Deserialize,PartialEq)]
struct Task {
    name: String,
    completed: bool,
}

#[derive(Debug, serde::Serialize,Clone,PartialEq)]
struct TaskList {
    tasks: Vec<Task>,
}

fn create_task_list() -> TaskList {
    TaskList {
        tasks: vec![
            Task { name: String::from("Les Monstres de Marshmallow"), completed: false },
            Task { name: String::from("Les Télégraphistes de la Galaxie"), completed: false },
            Task { name: String::from("Les Centaures Turbulents"), completed: false },
            Task { name: String::from("Les Slalomeurs de la Fantaisie"), completed: false },
            Task { name: String::from("Les Lutins Affamés"), completed: false },
            Task { name: String::from("Les Strangers Flamboyants"), completed: false },
            Task { name: String::from("Les Cosaques de l’Ouest"), completed: false },
            Task { name: String::from("Les Épéistes du Royaume"), completed: false },
            Task { name: String::from("Les Patineurs de la Révolution"), completed: false },
            Task { name: String::from("Les Gladiateurs du Temps"), completed: false },
            Task { name: String::from("Les Grenouilles de l’Écume"), completed: false },
            Task { name: String::from("Les Vagabonds de l’Espace"), completed: false },
            Task { name: String::from("Les Cœurs Héroïques"), completed: false },
            Task { name: String::from("Les Bandits de l’Aurore"), completed: false },
            Task { name: String::from("Les Mousquetaires des Ténèbres"), completed: false },
            Task { name: String::from("Les Éclaireurs de l’Avenir"), completed: false },
            Task { name: String::from("Les Chevaliers de l’Hiver"), completed: false },
            Task { name: String::from("Les Fous Volants de la Nuit"), completed: false },
            Task { name: String::from("Les Sorciers de l’Orient"), completed: false },
            Task { name: String::from("Les Hiérarques de la Paix"), completed: false },
            Task { name: String::from("Les Cavaliers du Vent"), completed: false },
            Task { name: String::from("Les Cavaliers du Destin"), completed: false },
        ],
    }
}

type SharedState = Arc<Mutex<TaskList>>;

// Function to create and return a closure that stores a value
fn get_shared_state() -> &'static SharedState {
    // Initialize the shared state if it doesn't exist yet
    static mut SHARED_STATE: Option<SharedState> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            SHARED_STATE = Some(Arc::new(Mutex::new(TaskList { tasks: Vec::new() })));
        });

        SHARED_STATE.as_ref().unwrap()
    }
}
#[get("/teams")]
fn get_teams() -> Json<TaskList> {
    // Create a list of tasks similar to the Java code


    // Shuffle the tasks randomly
    let mut rng = rand::thread_rng();
    let mut selected_tasks = create_task_list().tasks.choose_multiple(&mut rng, 5).cloned().collect::<Vec<Task>>();

    // Create and return the TaskList struct
    let task_list_response = TaskList { tasks: selected_tasks };
    Json(task_list_response)
}
#[get("/teamsChoose")]
fn get_teams_choose() -> Json<TaskList> {
    // Create a list of tasks similar to the Java code


    // Shuffle the tasks randomly
    let shared_state = get_shared_state();
    let mut state_guard = shared_state.lock().unwrap();
    // Create and return the TaskList struct
    Json(state_guard.to_owned())
}
#[post("/teamspost", format = "text", data = "<user_input>")]
fn hello_post(user_input: &str) -> Result<String, Status> {
    println!("TaskList: {:?}", user_input);
    let task_list_result = json_to_tasklist(user_input);

    match task_list_result {
        Ok(task_list) => {
            let shared_state = get_shared_state();
            let mut state_guard = shared_state.lock().unwrap();
            state_guard.tasks = task_list.tasks;

            println!("TaskList: {:?}", state_guard);
        }
        Err(e) => {
            println!("Error parsing JSON: {}", e);
        }
    }
    Ok("Data received successfully".to_string())

}
fn cors() -> rocket_cors::Cors {
    // Define allowed origins
    let allowed_origins = AllowedOrigins::all(); // Allow requests from any origin

    // Create CORS options
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post, rocket::http::Method::Options] // Add other methods as needed
            .into_iter()
            .map(From::from)
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("Error while building CORS");

    cors
}

#[launch]
fn rocket() -> _ {



    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 4000)))
        .mount("/api", routes![get_teams,hello_post,get_teams_choose])
        .attach(cors())


}
impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{{ name: {}, completed: {} }}", self.name, self.completed)
    }
}
fn json_to_tasklist(json_str: &str) -> Result<TaskList, serde_json::Error> {
    // Parse the JSON string into a vector of Task objects
    let tasks: Vec<Task> = serde_json::from_str(json_str)?;

    // Create a TaskList from the vector of Task objects
    let task_list = TaskList { tasks };

    Ok(task_list)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let teststring=Task { name: String::from("Les Monstres de Marshmallow"), completed: false };
        assert_eq!(hello_post(teststring.to_string()), Ok("Data received successfully".to_string()));
    }
    #[test]
    fn test_transform_to_task_list() {
        // Define teststring as a string literal with a static lifetime
        let json_str = r#"[{ "name": "Les Monstres de Marshmallow", "completed": false },{ "name": "Les Monstres de Marshmallow", "completed": false }]"#;


        // Extract the inner values for comparison
        let expected_result = vec![
            Task { name: String::from("Les Monstres de Marshmallow"), completed: false },
            Task { name: String::from("Les Monstres de Marshmallow"), completed: false }
        ];
        let task_list_result = json_to_tasklist(json_str);

        match task_list_result {
            Ok(task_list) => {
                println!("TaskList: {:?}", task_list);
                assert_eq!(task_list, TaskList { tasks: expected_result });//, "Expected: {:?}, Actual: {:?}", expected_result);
            }
            Err(e) => {
                println!("Error parsing JSON: {}", e);
            }
        }

        // Assert that the inner values match

    }


}


