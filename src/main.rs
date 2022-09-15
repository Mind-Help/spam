use std::str::FromStr;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tokio_postgres::{Config, Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
	let config = Config::from_str(env!("DATABASE_URL")).unwrap();
	let (client, connection) = config.connect(NoTls).await?;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}
	});

	let emails = client
		.query(r#"SELECT "User".email FROM "User";"#, &[])
		.await?;

	emails
		.iter()
		.for_each(|email| println!("{}", email.get::<usize, &str>(0)));

	Ok(())
}

fn send_mail(email: &str) {
	let message = Message::builder()
		.from(
			format!("Mind Help <{}>", env!("OUR_EMAIL"))
				.parse()
				.unwrap(),
		)
		.reply_to(format!("Olá <{}>", email).parse().unwrap())
		.to("Hei <augustomp@concordiasl.com.br>".parse().unwrap())
		.subject("Happy new year")
		.body(String::from("Be happy!"))
		.unwrap();

	let creds = Credentials::new(
		env!("SMTP_USERNAME").to_string(),
		env!("SMTP_PASSWD").to_string(),
	);

	let mailer = SmtpTransport::relay(env!("SMTP_HOST"))
		.unwrap()
		.credentials(creds)
		.build();

	match mailer.send(&message) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {:?}", e),
	}
}
