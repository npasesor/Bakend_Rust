#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Probando el server de rocket",
        },
    )
}
#[get("/abaut")]
fn abaut() -> &'static str {
    "profile!"
}
#[get("/")]
fn profile() -> &'static str {
    "profile!"
}

#[post("/")]
fn createprofile() -> &'static str {
    "create profile"
}

#[put("/")]
fn updateprofile() -> &'static str {
    "update profile"
}

#[delete("/")]
fn deleteprofile() -> &'static str {
    "eliminar"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, abaut])
        .mount(
            "/profile",
            routes![profile, createprofile, updateprofile, deleteprofile],
        )
        .attach(Template::fairing())
}
