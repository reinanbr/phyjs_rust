use rocket_dyn_templates::{Template, context};


#[get("/hello/<name>")]
pub fn hello_get(name: &str) -> Template{
    Template::render("tera/pages/home", context! { user_name: name })
}
