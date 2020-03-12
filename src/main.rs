#![feature(proc_macro_hygiene, decl_macro)]

use rand::prelude::*;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

#[macro_use]
extern crate serde_derive;

mod facts;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    fact: Option<String>,
    path: Option<String>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[openapi(skip)]
#[get("/")]
fn index(fact_list: State<facts::Facts>) -> Template {
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
#[openapi]
#[get("/fact")]
fn fact(fact_list: State<facts::Facts>) -> String {
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

fn main() {
    let prometheus = rocket_prometheus::PrometheusMetrics::new();
    let fact_list = facts::make();
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("public"))
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .manage(fact_list)
        .register(catchers![not_found])
        .mount("/", routes_with_openapi![index, fact])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../openapi.json".to_owned()),
                urls: None,
            }),
        )
        .launch();
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::*;
    use rocket_contrib::templates::Template;

    #[test]
    fn facts() {
        let rkt = rocket::ignite()
            .manage(super::facts::make())
            .mount("/", routes![super::fact]);
        let client = Client::new(rkt).expect("valid rocket");
        let response = client.get("/fact").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn page() {
        let rkt = rocket::ignite()
            .attach(Template::fairing())
            .manage(super::facts::make())
            .mount("/", routes![super::index]);
        let client = Client::new(rkt).expect("valid rocket");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
