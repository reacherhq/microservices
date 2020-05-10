// Reacher
// Copyright (C) 2018-2020 Amaury Martiny

// Reacher is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Reacher is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Reacher.  If not, see <http://www.gnu.org/licenses/>.

mod handlers;
mod saasify_secret;

use saasify_secret::check_saasify_secret;
use std::{env, net::IpAddr};
use warp::Filter;

/// Create all the endpoints of our API.
fn create_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// POST /check_email
	warp::path("check_email")
		.and(warp::post())
		// FIXME We should be able to just use warp::header::exact, and remove
		// completely `./saasify_secret.rs`.
		// https://github.com/seanmonstar/warp/issues/503
		.and(check_saasify_secret())
		// When accepting a body, we want a JSON body (and to reject huge
		// payloads)...
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::json())
		.and_then(handlers::check_email)
		// View access logs by setting `RUST_LOG=reacher`.
		.with(warp::log("reacher"))
}

/// Run a HTTP server using warp.
///
/// # Panics
///
/// If at least one of the environment variables:
/// - RCH_HTTP_HOST
/// - RCH_PROXY_HOST
/// - RCH_PROXY_PORT
/// is malformed, then the program will panic.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	env_logger::init();

	// Use an empty string if we don't have any env variable for sentry. Sentry
	// will just silently ignore.
	let sentry = sentry::init(env::var("RCH_SENTRY_DSN").unwrap_or_else(|_| "".into()));
	// Sentry will also catch panics.
	sentry::integrations::panic::register_panic_handler();
	if sentry.is_enabled() {
		log::info!(target: "reacher", "Sentry is successfully set up.")
	}

	let api = create_api();

	log::info!(target: "reacher", "Server is listening on :8080.");

	// Since we're running the HTTP server inside a Docker container, we
	// use 0.0.0.0. The port is 8080 as per Fly documentation.
	warp::serve(api)
		.run((
			env::var("RCH_HTTP_HOST")
				.unwrap_or_else(|_| "127.0.0.1".into())
				.parse::<IpAddr>()
				.expect("RCH_HTTP_HOST is malformed."),
			8080,
		))
		.await;
	Ok(())
}

#[cfg(test)]
mod tests {
	use warp::http::StatusCode;
	use warp::test::request;

	use super::create_api;

	#[tokio::test]
	async fn test_saasify_secret() {
		let resp = request()
			.path("/check_email")
			.method("POST")
			.reply(&create_api())
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
		assert_eq!(
			resp.body(),
			"Missing request header \"x-saasify-secret\"".as_bytes()
		);
	}
}
