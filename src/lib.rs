#[cxx::bridge]
mod ffi {
	#[namespace = "rs::tokio::core"]
	extern "Rust" {
		type Core;

		fn core() -> Box<Core>;
	}

	#[namespace = "rs::core::config"]
	extern "Rust" {
		type SessionConfig;

		fn session_config() -> Box<SessionConfig>;
	}

	#[namespace = "rs::core::authentication"]
	extern "Rust" {
		type Credentials;

		fn with_password(username: String, password: String) -> Box<Credentials>;
	}

	#[namespace = "rs::core::session"]
	extern "Rust" {
		type Session;

		fn connect(session_config: &Box<SessionConfig>,
				   credentials: &Box<Credentials>,
				   mut core: &mut Box<Core>) -> Box<Session>;
	}
}

//region tokio_core::reactor::Core

pub struct Core(tokio_core::reactor::Core);

fn core() -> Box<Core> {
	Box::new(Core(tokio_core::reactor::Core::new().unwrap()))
}

//endregion

//region librespot_core::config::SessionConfig

pub struct SessionConfig(librespot_core::config::SessionConfig);

fn session_config() -> Box<SessionConfig> {
	Box::new(SessionConfig(librespot_core::config::SessionConfig::default()))
}

//endregion

//region librespot_core::authentication::Credentials

pub struct Credentials(librespot_core::authentication::Credentials);

fn with_password(username: String, password: String) -> Box<Credentials> {
	Box::new(Credentials(librespot_core::authentication::Credentials::with_password(username, password)))
}

//endregion

//region librespot_core::session::Session

pub struct Session(librespot_core::session::Session);

fn connect(session_config: &Box<SessionConfig>,
		   credentials: &Box<Credentials>,
		   core: &mut Box<Core>) -> Box<Session>
{
	let session = librespot_core::session::Session
	::connect((**session_config).0.clone(), (**credentials).0.clone(),
			  None, (*core).0.handle());

	Box::new(Session((*core).0.run(session).unwrap()))
}

//endregion