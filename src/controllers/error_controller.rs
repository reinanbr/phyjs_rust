use rocket::Request;
use rocket_dyn_templates::{Template, context};


#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    println!("Handling 404 for URI: {}", req.uri());

    Template::render(
        "error/_404_",
        context! {
            title:"404 (Not Found)",
            uri: req.uri().to_string() 
        },
    )
}
