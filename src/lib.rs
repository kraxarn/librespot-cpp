#[cxx::bridge]
mod ffi {
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
				   credentials: &Box<Credentials>) -> Result<Box<Session>>;
	}

	#[namespace = "librespot::playback::audio_backend"]
	extern "Rust" {
		fn backends() -> Vec<String>;
	}
}

//region librespot_core

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
		   credentials: &Box<Credentials>) -> Result<Box<Session>, String>
{
	let config = (**session_config).0.clone();
	let credential = (**credentials).0.clone();

	let join_handle = std::thread::spawn(move || {
		let mut core = tokio_core::reactor::Core::new().unwrap();
		let handle = core.handle();

		core.run(librespot_core::session::Session::connect(
			config, credential, None, handle)).unwrap()
	});

	match join_handle.join() {
		Ok(s) => Ok(Box::new(Session(s))),
		Err(e) => Err(e.downcast_ref::<String>()
			.unwrap_or(&"Unknown Error".to_string()).to_string()),
	}
}

//endregion

//endregion

//region librespot_playback

//region librespot::playback::audio_backend

fn backends() -> std::vec::Vec<String> {
	librespot_playback::audio_backend::BACKENDS
		.iter()
		.map(|backend| backend.0.to_string())
		.collect()
}

//endregion

//endregion