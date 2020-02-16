#![feature(proc_macro_hygiene, decl_macro)]

use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
use rand::prelude::*;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

mod facts;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    fact: Option<String>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index(fact_list: State<facts::Facts>) -> Template {
    let ref facts = *fact_list;
    let i = thread_rng().gen::<usize>() % facts.len();

    Template::render(
        "index",
        &TemplateContext {
            title: "Printer Facts",
            fact: Some(facts[i].clone()),
            parent: "layout",
        },
    )
}

#[get("/fact")]
fn fact(fact_list: State<facts::Facts>) -> String {
    let ref facts = *fact_list;
    let i = thread_rng().gen::<usize>() % facts.len();

    format!("{}", facts[i])
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
        .mount("/", routes![index, fact])
        .launch();
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::*;

    #[test]
    fn facts() {
        let rkt = rocket::ignite()
            .manage(super::facts::make())
            .mount("/", routes![super::fact]);
        let client = Client::new(rkt).expect("valid rocket");
        let response = client.get("/fact").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
