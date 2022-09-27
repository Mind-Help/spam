use std::net::IpAddr;
use std::str::FromStr;

use lettre::message::header::ContentType;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use tokio_postgres::{Client, Config, Error, NoTls};
use warp::Filter;

static TEMPLATE: &str = include_str!("../template.html");

#[tokio::main]
async fn main() -> Result<(), Error> {
	let mailer = SmtpTransport::relay(env!("SMTP_HOST"))
		.unwrap()
		.credentials(Credentials::new(
			env!("SMTP_USERNAME").to_string(),
			env!("SMTP_PASSWD").to_string(),
		))
		.tls(Tls::Required(
			TlsParameters::new(env!("DOMAIN").to_string()).unwrap(),
		))
		.authentication(vec![Mechanism::Login])
		.port(
			env!("SMTP_PORT")
				.parse()
				.unwrap_or_else(|_| panic!("$SMTP_PORT could not be parsed to integer")),
		)
		.build();

	let client = get_db_client().await?;
	let data: Vec<(String, String)> = client
		.query(r#"SELECT "User".name, "User".email FROM "User";"#, &[])
		.await?
		.iter()
		.map(|row| {
			(
				row.get::<usize, &str>(0).to_string(),
				row.get::<usize, &str>(1).to_string(),
			)
		})
		.collect();

	let mailer_clone = mailer.clone();
	let path = warp::path!("send_mail")
		.map(move || {
			data.iter()
				.for_each(|(name, email)| send_mail(name, email, &mailer_clone));
			"ok"
		})
		.or(warp::path!("teste").map(move || {
			send_mail("Augusto", "augustomp@concordiasl.com.br", &mailer);
			"ok"
		}))
		.with(warp::cors().allow_any_origin());

	warp::serve(path)
		.run((
			IpAddr::from_str("::0").unwrap(),
			env!("PORT")
				.parse()
				.unwrap_or_else(|_| panic!("$SERVER_PORT could not be parsed to integer")),
		))
		.await;

	Ok(())
}

async fn get_db_client() -> Result<Client, Error> {
	let config = Config::from_str(env!("DATABASE_URL")).unwrap();
	let (client, connection) = config.connect(NoTls).await?;

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}
	});

	Ok(client)
}

fn send_mail(name: &str, email: &str, mailer: &SmtpTransport) {
	let template = TEMPLATE.replace("${nome}", name);

	let message = Message::builder()
		.from(
			format!("Mind Help <{}>", env!("OUR_EMAIL"))
				.parse()
				.unwrap(),
		)
		.to(format!("{name} <{email}>").parse().unwrap())
		.subject("newsletter")
		.singlepart(
			SinglePart::builder()
				.header(ContentType::TEXT_HTML)
				.body(template),
		)
		.unwrap();

	match mailer.send(&message) {
		Ok(_) => println!("Email sent successfully!"),
		Err(e) => panic!("Could not send email: {e:?}"),
	}
}
