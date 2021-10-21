use actix_web::*;

#[get("/api/pages/fortnite-game")]
pub async fn fortnite_game(app: crate::AppData) -> impl Responder {
    HttpResponse::Ok().json(app.files.fortnite_game.clone())
}

#[get("/api/pages/fortnite-game/{news}")]
pub async fn fortnite_game_(
    app: crate::AppData,
    news: web::Path<String>
) -> impl Responder {
    let news = news.into_inner();
    
    HttpResponse::Ok().json(
        app.files.fortnite_game.get(news).unwrap_or(
            &app.files.fortnite_game.clone()
        )
    )
}