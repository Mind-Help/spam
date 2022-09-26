use std::net::IpAddr;
use std::str::FromStr;

use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};
use tokio_postgres::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let mailer = SmtpTransport::relay(env!("SMTP_HOST"))
		.unwrap()
		.credentials(Credentials::new(
			env!("SMTP_USERNAME").to_string(),
			env!("SMTP_PASSWD").to_string(),
		))
		.authentication(vec![Mechanism::Login])
		.port(
			env!("SMTP_PORT")
				.parse()
				.unwrap_or_else(|_| panic!("$SMTP_PORT could not be parsed to integer")),
		)
		.build();

	warp::serve(warp::path!("send_mail").map(move || {
		send_mail("", &mailer);
		"ok"
	}))
	.run((
		IpAddr::from_str("::0").unwrap(),
		env!("PORT")
			.parse()
			.unwrap_or_else(|_| panic!("$SERVER_PORT could not be parsed to integer")),
	))
	.await;

	Ok(())

	/*let client = get_db_client().await?;

	client
		.query(r#"SELECT "User".email FROM "User";"#, &[])
		.await?
		.iter()
		.for_each(|row| send_mail(row.get::<usize, &str>(0), &mailer));*/
}

/*async fn get_db_client() -> Result<Client, Error> {
	let config = Config::from_str(env!("DATABASE_URL")).unwrap();
	let (client, connection) = config.connect(NoTls).await?;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}
	});

	Ok(client)
}
*/

fn send_mail(_email: &str, mailer: &SmtpTransport) {
	let message = Message::builder()
		.from(
			format!("Mind Help <{}>", env!("OUR_EMAIL"))
				.parse()
				.unwrap(),
		)
		.to("Hei <augustomp@concordiasl.com.br>".parse().unwrap())
		.subject("Happy new year")
		.body(String::from("Be happy!"))
		.unwrap();

	match mailer.send(&message) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {:?}", e),
	}
}
