
use rocket_dyn_templates::{context, Template};


#[get("/")]
pub fn home_get() -> Template{
    Template::render("index",context!{title:"Página Inicial"})
}
