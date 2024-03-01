use actix_web::{get, patch, post, web, App, HttpResponse, HttpServer, Responder};

use hostname::get;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas avaliable!")    
}

#[get("")]
async fn index() -> impl Responder {
    let hostname = get().expect("Failed to get hostname");
    HttpResponse::Ok().body(format!("hostname: {}!", hostname.to_string_lossy()))    
}

#[post("/buypizza")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying a pizza!")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a pizza!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(web::scope("/apirust-v1")
            .service(index))
            // .service(get_pizzas)
            // .service(buy_pizza)
            // .service(update_pizza)
            // .service(index)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}
