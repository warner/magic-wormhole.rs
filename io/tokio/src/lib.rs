extern crate bytes;
extern crate magic_wormhole_core;
extern crate futures;
#[macro_use]
extern serde_json;
use magic_wormhole_core::WormholeCore;
use futures::{Stream, Sink};
use bytes::IntoBuf;
use serde_json::Value;

enum Mood {
    Happy,
    Lonely,
}

enum Error {
    Generic,
}

/*
struct Reader {
}
impl Stream<[u8], Error> for Reader {
}

struct Writer {
}
impl Sink<[u8], Error> for Writer {
}
*/

pub struct Wormhole {
    core: WormholeCore,
    //pub reader: Stream<[u8], Error>,
    // maybe bytes::Bytes? or Box<&[u8]>? or Box<bytes::Buf> ?
    //pub writer: Sink<[u8], Error>,
}

impl Sink<IntoBuf, Error> for Wormhole {
    ...
}

impl Stream<IntoBuf, Error> for Wormhole {
    ...
}

impl Wormhole {
    pub fn new(appid: &str, relay_url: &str) -> Wormhole {
        let c = WormholeCore::new(appid, relay_url);
        Wormhole { core: c }
    }

    pub fn allocate_code(&mut self) -> Future<String, Error> {
        futures::future::ok("4-purple-sausages".to_string())
    }

    pub fn set_code(&mut self, code: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn input_code(&mut self) -> CodeInput { // is a sink/stream
        notimplemented!()
    }

    // note: (maybe?) no got_code, everything is handled with Futures

    pub fn get_verifier(&mut self) -> Future<String, Error> {
        futures::future::ok("fake verifier".to_string())
    }

    pub fn get_versions(&mut self) -> Future<Value, Error> {
        futures::future::ok(json!({"fake": "version"}))
    }

    pub fn close(&mut self) -> Future<Mood, Error> {
        // we're not happy until you're happy. so we lie.
        futures::future::ok(Mood::Happy)
    }


}


