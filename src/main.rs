#[macro_use]
extern crate rocket;

use tangy_lib::TangyLib;

#[get("/adv")]
fn adv() -> String {
    let mut t = TangyLib::init(&std::path::PathBuf::from("/var/lib/tang")).unwrap();
    t.adv()
}

#[post("/rec/<kid>", data="<data>")]
fn rec(kid: String, data: String) -> String {
    let mut t = TangyLib::init(&std::path::PathBuf::from("/var/lib/tang")).unwrap();

    t.rec(&kid, &data).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![adv, rec])
}
