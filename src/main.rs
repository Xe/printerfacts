use rand::prelude::*;
use rocket::{State, fs::FileServer};
use serde::Serialize;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    fact: Option<String>,
    path: Option<String>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
async fn index(fact_list: &State<pfacts::Facts>) -> Template {
    let ref facts = *fact_list;
    let i = thread_rng().gen::<usize>() % facts.len();

    Template::render(
        "index",
        &TemplateContext {
            title: "Printer Facts",
            fact: Some(facts[i].clone()),
            path: None,
            parent: "layout",
        },
    )
}

/// Returns an exciting fact about printers.
#[get("/fact")]
async fn fact(fact_list: &State<pfacts::Facts>) -> String {
    let ref facts = *fact_list;
    let i = thread_rng().gen::<usize>() % facts.len();

    facts[i].clone()
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> Template {
    Template::render(
        "error/404",
        &TemplateContext {
            title: "Not found",
            fact: None,
            path: Some(req.uri().path().to_string()),
            parent: "layout",
        },
    )
}

#[launch]
fn rocket() -> _ {
    let fact_list = pfacts::make();
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("public"))
        .manage(fact_list)
        .register("/", catchers![not_found])
        .mount("/", routes![index, fact])
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket_dyn_templates::Template;

    #[test]
    fn facts() {
        let rkt = rocket::build()
            .manage(pfacts::make())
            .mount("/", routes![super::fact]);
        let client = Client::tracked(rkt).expect("valid rocket");
        let response = client.get("/fact").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn page() {
        let rkt = rocket::build()
            .attach(Template::fairing())
            .manage(pfacts::make())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rkt).expect("valid rocket");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
