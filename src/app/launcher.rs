use actix_web::*;
use crate::models::launcher::*;

#[get("/form")]
pub async fn form() -> impl Responder {
    HttpResponse::Ok()
    .body(include_bytes!("../../resources/register.html").to_vec())
}

#[post("/login")]
pub async fn login(
    app: crate::AppData,
    body: web::Form<LoginForm>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    
    if body.login.is_some() && body.password.is_some() {
        let login = body.login.unwrap();
        let password = body.password.unwrap();
        
        let user = app.database.users.find_one(
            bson::doc! {
                "login": login
            },
            None
        ).await?.ok_or("ID Not Found")?;
        
        if bcrypt::verify(password, &user.password)? {
            let token = app.new_token(Some(user.id), false).await;
            
            return Ok(HttpResponse::Ok().json(LoginResponse {
                access_token: token.to_simple().to_string(),
                account_id: user.id.to_simple().to_string(),
                display_name: user.display_name,
                launcher_token: user.launcher_token.to_simple().to_string()
            }));
        } else {
            return Ok(HttpResponse::Unauthorized().body("Invalid Password"));
        }
    } else if body.launcher_token.is_some() {
        let launcher_token = body.launcher_token.unwrap();
        
        let user = app.database.users.find_one(
            bson::doc! {
                "launcher_token": launcher_token
            },
            None
        ).await?.ok_or("Invalid Launcher Token")?;
        
        let token = app.new_token(Some(user.id), false).await;
        
        return Ok(HttpResponse::Ok().json(LoginResponse {
            access_token: token.to_simple().to_string(),
            account_id: user.id.to_simple().to_string(),
            display_name: user.display_name,
            launcher_token: user.launcher_token.to_simple().to_string()
        }));
    }
    
    Ok(HttpResponse::BadRequest().into())
}

#[get("/exchange")]
pub async fn exchange(
    app: crate::AppData,
    req: HttpRequest
) -> impl Responder {
    let (_, token) = match app.validate(&req, None).await {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().into()
    };
    
    if let Some(user) = token.acc {
        let code = app.new_exchange(user).await;
        
        return HttpResponse::Ok().json(ExchangeResponse {
            code: code.to_simple().to_string()
        });
    }
    
    HttpResponse::Unauthorized().into()
}

#[post("/register")]
pub async fn register(
    app: crate::AppData,
    body: web::Form<RegisterForm>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    
    if let Some(captcha) = &app.captcha {
        let captcha_res = captcha.verify(body.captcha.ok_or("'captcha' Not Present")?).await?;
        
        if !captcha_res.success() {
            return Ok(HttpResponse::Unauthorized().body("Invalid Captcha"))
        }
    }
    
    app.database.new_user(body.login, body.display_name, body.password).await?;
    
    Ok(HttpResponse::Ok().into())
}