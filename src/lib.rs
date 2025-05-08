wit_bindgen::generate!({
    world: "wasi:http/proxy@0.2.0",
    generate_all
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types as wasi_http_types;

struct Component;

impl Guest for Component {
    fn handle(
        request: wasi_http_types::IncomingRequest,
        response_out: wasi_http_types::ResponseOutparam,
    ) -> () {
        let headers = [(
            wasi_http_types::FieldKey::from("Content-Type"),
            wasi_http_types::FieldValue::from("text/html"),
        )];
        let headers = wasi_http_types::Headers::from_list(&headers).unwrap();
        let response = wasi_http_types::OutgoingResponse::new(headers);
        response.set_status_code(200).unwrap();

        let body = response.body().unwrap();
        let stream = body.write().unwrap();
        stream.blocking_write_and_flush(b"Hello, world!").unwrap();
        drop(stream);

        wasi_http_types::OutgoingBody::finish(body, None).unwrap();
        wasi_http_types::ResponseOutparam::set(response_out, Ok(response));
    }
}

export!(Component);
