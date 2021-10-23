use actix_web::*;
use hcaptcha::*;

pub struct Client {
    pub client: HcaptchaClient,
    secret: String
}

impl Client {
    pub fn new(secret: String) -> Self {
        let client = HcaptchaClient::new();
        
        Self {
            client,
            secret
        }
    }
    
    pub async fn verify(&self, captcha: String) -> Result<HcaptchaResponse, HcaptchaError> {
        let captcha = HcaptchaCaptcha::new(&captcha)?;
        let request = HcaptchaRequest::new(&self.secret, captcha)?;
        
        self.client.verify_client_response(request).await
    }
}