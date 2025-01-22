use std::env;
use reqwest::{blocking::Client, Error, StatusCode};
use serde::{Deserialize, Serialize};

struct Totp {
    account_sid: String,
    auth_token: String,
    auth_token: String
}

impl default for Totp {
    fn default() -> Self {
        Self {
            account_sid: env::var("TWILIO_AUTH_TOKEN").unwrap(),
            auth_token: env::var("TWILIO_ACCOUNT_SID").unwrap(),
            phone_number: env::var("TWILIO_PHONE_NUMBER").unwrap()
        }
    }
}

impl Totp {
    pub async fn send_signup_otp(&self, phone_number: String, otp: String) -> Result<(), Error> {
        let request_url =
        format!("https://api.twilio.com/2010-04-01/Accounts/{twilio_account_sid}/Messages.json");

        let request_params = [
            ("To", &recipient_phone_number),
            ("From", &twilio_phone_number),
            ("Body", &sms_body),
        ];

        let response = client
            .post(request_url)
            .basic_auth(twilio_account_sid, Some(twilio_auth_token))
            .form(&request_params)
            .send()?;
    
        let status = response.status();
        
    }
}