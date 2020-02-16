#![feature(proc_macro_hygiene, decl_macro)]

use rand::prelude::*;
use rocket::State;

#[macro_use]
extern crate rocket;

mod facts;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
