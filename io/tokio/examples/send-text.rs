extern crate tokio;
extern crate magic_wormhole_io_tokio;
use tokio::prelude::*;
use magic_wormhole_io_tokio::Wormhole;
// Can ws do hostname lookup? Use ip addr, not localhost, for now
const MAILBOX_SERVER: &'static str = "ws://127.0.0.1:4000/v1";
const APPID: &'static str = "lothar.com/wormhole/text-or-file-xfer";

fn main() {
    println!("starting");
    let w = Wormhole::new(APPID, MAILBOX_SERVER);
    //w.send(b"hello old friend"); // synchronous, queues for later
    //let w_future = w.writer.send(b"hello old friend");
    w.send(b"hello old friend".to_vec()); // Wormhole is Sink

    let c = w.allocate_code().and_then(|code| {
        println!("code is: {}", code);
        Ok(()) // ?
    });
    // or w.get_messages() -> Stream ?
    /*
    let r = w.get_message().and_then(|msg| {
        println!("ack is: {:?}", msg); // binary
        Ok(())
    });*/
    let r = w.for_each(|message| {
        println!("msg is: {:?}", message);
        Ok(())
    });
    // this is tokio::executor::spawn, and wants Future<(),()>, which sounds
    // like some sort of time-travelling robotic owl-eyed superhero.
    tokio::spawn(c.join(r.into_future()));

    // this is tokio::runtime::run, and also wants Future<(),()> . It drives
    // the future forward, but doesn't exit when it completes: it keeps
    // running until everything in the executor stops twitching. It must be
    // called outside the context of an executor (else it panics), in
    // contrast with tokio::spawn which *must* be called inside such a
    // context.
    tokio::run(w);

    // OTOH, DefaultExecutor::current().spawn(f) might be a way to schedule,
    // from outside the context, something to run once it's started.
}
