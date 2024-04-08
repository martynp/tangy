#[macro_use]
extern crate rocket;

use std::sync::RwLock;

use clap::Parser;
use rocket::http::{ContentType, Status};
use rocket::State;
use tangy_lib::{KeySource, TangyLib};

struct TangState {
    pub state: RwLock<TangyLib>,
}

#[get("/adv")]
fn adv(tangy_state: &State<TangState>) -> (Status, (ContentType, Option<String>)) {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(None) {
        Ok(a) => {
            return (
                Status::Ok,
                (ContentType::new("application", "jose+json"), Some(a)),
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return (
                Status::NotFound,
                (ContentType::new("application", "jose+json"), None),
            );
        }
        Err(_) => {
            return (
                Status::InternalServerError,
                (ContentType::new("application", "jose+json"), None),
            );
        }
    }
}

#[get("/adv/<skid>")]
fn adv_kid(skid: &str, tangy_state: &State<TangState>) -> (Status, (ContentType, Option<String>)) {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(Some(skid)) {
        Ok(a) => {
            return (
                Status::Ok,
                (ContentType::new("application", "jose+json"), Some(a)),
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return (
                Status::NotFound,
                (ContentType::new("application", "jose+json"), None),
            );
        }
        Err(_) => {
            return (
                Status::InternalServerError,
                (ContentType::new("application", "jose+json"), None),
            );
        }
    }
}

#[post("/rec/<kid>", data = "<data>")]
fn rec(kid: &str, data: &str, tangy_state: &State<TangState>) -> (Status, (ContentType, String)) {
    let tangy = tangy_state.state.read().unwrap();

    (
        Status::Ok,
        (
            ContentType::new("application", "jwk+json"),
            tangy.rec(&kid, &data).unwrap(),
        ),
    )
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// location of certificates database
    #[arg(short, long, default_value = "db")]
    dir: std::path::PathBuf,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();

    let tangy_state = TangState {
        state: RwLock::new(TangyLib::init(KeySource::LocalDir(&args.dir)).unwrap()),
    };

    let figment = rocket::Config::figment().merge(("port", args.port));

    rocket::custom(figment)
        .manage(tangy_state)
        .mount("/", routes![adv, adv_kid, rec])
}
