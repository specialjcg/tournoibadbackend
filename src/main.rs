#[macro_use]
extern crate rocket;

use rocket::{Request, Response};
use rand::seq::SliceRandom;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;


#[derive(Debug, serde::Serialize, Clone)]
struct Task {
    name: String,
    completed: bool,
}

#[derive(Debug, serde::Serialize)]
struct TaskList {
    tasks: Vec<Task>,
}

#[get("/tasks")]
fn get_tasks() -> Json<TaskList> {
    // Create a list of tasks similar to the Java code
    let task_list = vec![
        Task { name: String::from("Les Monstres de Marshmallow"), completed: false },
        Task { name: String::from("Les Télégraphistes de la Galaxie"), completed: false },
        Task { name: String::from("Les Centaures Turbulents"), completed: false },
        Task { name: String::from("Les Slalomeurs de la Fantaisie"), completed: false },
        Task { name: String::from("Les Lutins Affamés"), completed: false },
        Task { name: String::from("Les Étrangers Flamboyants"), completed: false },
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
    ];

    // Shuffle the tasks randomly
    let mut rng = rand::thread_rng();
    let mut selected_tasks = task_list.choose_multiple(&mut rng, 5).cloned().collect::<Vec<Task>>();

    // Create and return the TaskList struct
    let task_list_response = TaskList { tasks: selected_tasks };
    Json(task_list_response)
}
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 4000)))
        .attach(Cors)
        .mount("/api", routes![get_tasks])
}




