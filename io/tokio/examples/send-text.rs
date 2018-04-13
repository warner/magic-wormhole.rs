extern crate magic_wormhole_io_tokio;
use magic_wormhole_io_tokio::Wormhole;
// Can ws do hostname lookup? Use ip addr, not localhost, for now
const MAILBOX_SERVER: &'static str = "ws://127.0.0.1:4000/v1";
const APPID: &'static str = "lothar.com/wormhole/text-or-file-xfer";

fn main() {
    println!("starting");
    let w = Wormhole::new(APPID, MAILBOX_SERVER);
    //w.send(b"hello old friend"); // synchronous, queues for later
    //let w_future = w.writer.send(b"hello old friend");
    w.send(b"hello old friend"); // Wormhole is Sink

    let c = w.allocate_code().and_then(|code| {
        println!("code is: {}", code);
        Ok(()) // ?
    });
    // or w.get_messages() -> Stream ?
    let r = w.get_message().and_then(|msg| {
        println!("ack is: {:?}", msg); // binary
        Ok(())
    });
    tokio::spawn(c.join(r));

    tokio::run(w);
        
}
