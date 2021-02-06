#include <iostream>

#include "librespot_cxx.h"

int main(int argc, char **argv)
{
	auto core = rs::tokio::core::core();

	auto cfg = rs::core::config::session_config();
	auto cred = rs::core::authentication::with_password(argv[1], argv[2]);

	std::cout << "connecting..." << std::endl;
	auto session = rs::core::session::connect(cfg, cred, core);

	return 0;
}