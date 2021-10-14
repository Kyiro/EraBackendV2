use actix_web::*;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicError {
    error_code: String,
    
    error_message: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    message_vars: Option<Vec<String>>,
    
    numeric_error_code: i32,
    
    originating_service: String,
    
    intent: String,
    
    #[serde(rename = "error_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl EpicError {
    pub fn new() -> Self {
        Self {
            error_code: String::new(),
            error_message: String::new(),
            message_vars: None,
            numeric_error_code: 0,
            originating_service: String::new(),
            intent: String::from("prod"),
            error_description: None,
            error: None,
        }
    }
    
    pub fn unauthorized(req: &HttpRequest) -> Self {
        let url = req.path().to_string();
        
        Self::new()
            .error_code("errors.com.epicgames.common.authentication.authentication_failed")
            .numeric_error_code(1032)
            .originating_service("fortnite")
            .error_message(&("Authentication failed for ".to_owned() + &url))
            .message_vars(vec![url])
            .clone()
    }
    
    pub fn permission(perm: String, perm_type: String) -> Self {
        Self::new()
            .error_code("errors.com.epicgames.common.missing_permission")
            .numeric_error_code(1023)
            .originating_service("fortnite")
            .error_message(&format!(
                "Sorry your login does not posses the permissions '{} {}' needed to perform the requested operation",
                perm,
                perm_type
            ))
            .message_vars(vec![
                perm,
                perm_type
            ])
            .clone()
    }
    
    pub fn req_method() -> Self {
        Self::new()
            .error_code("errors.com.epicgames.common.method_not_allowed")
            .numeric_error_code(1009)
            .originating_service("fortnite")
            .error_message("Sorry the resource you were trying to access cannot be accessed with the HTTP method you used.")
            .clone()
    }
    
    pub fn not_found() -> Self {
        Self::new()
            .error_code("errors.com.epicgames.common.not_found")
            .numeric_error_code(1004)
            .originating_service("fortnite")
            .error_message("Sorry the resource you were trying to find could not be found")
            .clone()
    }
    
    pub fn error_code(&mut self, msg: &str) -> &mut Self {
        self.error_code = String::from(msg);
        self
    }

    pub fn error_message(&mut self, msg: &str) -> &mut Self {
        self.error_message = String::from(msg);
        self
    }

    pub fn message_vars(&mut self, msg: Vec<String>) -> &mut Self {
        self.message_vars = Some(msg);
        self
    }

    pub fn numeric_error_code(&mut self, msg: i32) -> &mut Self {
        self.numeric_error_code = msg;
        self
    }

    pub fn originating_service(&mut self, msg: &str) -> &mut Self {
        self.originating_service = String::from(msg);
        self
    }

    pub fn intent(&mut self, msg: &str) -> &mut Self {
        self.intent = String::from(msg);
        self
    }

    pub fn error_description(&mut self, msg: &str) -> &mut Self {
        self.error_description = Some(String::from(msg));
        self
    }

    pub fn error(&mut self, msg: &str) -> &mut Self {
        self.error = Some(String::from(msg));
        self
    }
}