//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

mod vec3;
mod ray;
mod camera;
mod hittable;
mod hitrecord;
mod hittable_list;
mod material;
mod utils;
mod render;

#[macro_use]
extern crate serde;

use hittable_list::HittableListWithTile;


/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "backend_name";

/// The name of a second backend associated with this service.
const OTHER_BACKEND_NAME: &str = "other_backend_name";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Make any desired changes to the client request.
    req.set_header(header::HOST, "example.com");

    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD | &Method::POST => (),

        // Accept PURGE requests; it does not matter to which backend they are sent.
        m if m == "PURGE" => return Ok(req.send(BACKEND_NAME)?),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_str("This method is not allowed\n"))
        }
    };

    // Pattern match on the path.
    match req.get_path() {

		"/render" => {
			println!("/render is currenty only called from the native Rust client");
			// let (t,d) = render::do_render();
			Ok(Response::from_status(StatusCode::OK))
				// .with_content_type(mime::IMAGE_JPEG)
				// .with_body(d))
		}
		"/rendertile" => {
			let b = req.into_body();
			let s = b.into_string();
			let input : HittableListWithTile = serde_json::from_str(&s).unwrap();
			let res = render::render_tile(&input.h, input.i,input.j, input.dimi, input.dimj, input.width, input.height);
			let res_json = serde_json::to_string(&res).unwrap();
			Ok(Response::from_status(StatusCode::OK)
				.with_body(res_json))
				// .with_content_type(mime::IMAGE_JPEG)
				// .with_body(d))
		}
        // If request is to the `/` path, send a default response.
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body("<iframe src='https://developer.fastly.com/compute-welcome' style='border:0; position: absolute; top: 0; left: 0; width: 100%; height: 100%'></iframe>\n")),

        // If request is to the `/backend` path, send to a named backend.
        "/backend" => {
            // Request handling logic could go here...  E.g., send the request to an origin backend
            // and then cache the response for one minute.
            req.set_ttl(60);
            Ok(req.send(BACKEND_NAME)?)
        }

        // If request is to a path starting with `/other/`...
        path if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_pass(true);
            Ok(req.send(OTHER_BACKEND_NAME)?)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_str("The page you requested could not be found\n")),
    }
}
