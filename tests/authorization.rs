mod common;

#[cfg(test)]
mod tests {
    
    use reqwest::Client;
    use serde_json::{json, Value};
    use crate::common;
    /* 
    #[tokio::test]
    async fn test_view_user() {
        let client = Client::new();
        let user = common::user_create_common(&client).await;
        let id = user["id"].as_i64().expect("No ID found") as i32;

        let view_response = client
            .get(&format!("{}/users/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to view user");

        assert!(view_response.status().is_success());

        let viewed_rustacean: Value = view_response.json().await.expect("Invalid response");
        assert_eq!(viewed_rustacean["id"].as_i64().expect("No ID found") as i32, id);
    }
    */

    #[tokio::test]
    async fn test_send_email() { 
        use lettre::message::header::ContentType;
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};

        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("charles.ng.x@protonmail.com".parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new("Charles.Ng.X@gmail.com".to_owned(), "your_google_app_password".to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }   
}