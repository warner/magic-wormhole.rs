extern crate magic_wormhole_core;
extern crate futures;
#[macro_use]
extern crate serde_json;
use magic_wormhole_core::WormholeCore;
use futures::prelude::*;
use serde_json::Value;

pub enum Mood {
    Happy,
    Lonely,
}

pub enum Error {
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
    #[allow(dead_code)]
    core: WormholeCore,
    //pub reader: Stream<[u8], Error>,
    // maybe bytes::Bytes? or Box<&[u8]>? or Box<bytes::Buf> ?
    //pub writer: Sink<[u8], Error>,
}

// for now, send/receive bytes::Bytes, but I'd like to make this accept
// anything that's IntoBuf instead. I don't know how.. would that make it
// Wormhole<T: IntoBuf> and let the app decide what buf-ish type they want to
// use once, at compile time?

// actually I can't even make Bytes work. Just use Vec<u8>.

impl Sink for Wormhole {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(&mut self, _item: Vec<u8>) -> StartSend<Vec<u8>, Error> {
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        unimplemented!()
    }

}

impl Stream for Wormhole {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Vec<u8>>, Error> {
        unimplemented!()
    }
}

// as a Future, we yield nothing until closed
impl Future for Wormhole {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        unimplemented!()
    }
}

pub struct CodeInput {
}

impl Wormhole {
    pub fn new(appid: &str, relay_url: &str) -> Wormhole {
        let c = WormholeCore::new(appid, relay_url);
        Wormhole { core: c }
    }

    pub fn allocate_code(&mut self) -> Box<Future<Item=String, Error=Error>> {
        Box::new(futures::future::ok("4-purple-sausages".to_string()))
    }

    pub fn set_code(&mut self, _code: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn input_code(&mut self) -> CodeInput { // is a sink/stream
        unimplemented!()
    }

    // note: (maybe?) no got_code, everything is handled with Futures

    pub fn get_verifier(&mut self) -> Box<Future<Item=String, Error=Error>> {
        Box::new(futures::future::ok("fake verifier".to_string()))
    }

    pub fn get_versions(&mut self) -> Box<Future<Item=Value, Error=Error>> {
        Box::new(futures::future::ok(json!({"fake": "version"})))
    }

    pub fn close(&mut self) -> Box<Future<Item=Mood, Error=Error>> {
        // we're not happy until you're happy. so we lie.
        Box::new(futures::future::ok(Mood::Happy))
    }


}


