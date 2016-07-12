extern crate ws;

use std::sync::mpsc;

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

use CanaryEvent;

pub struct Client {
    pub out: Sender,
    pub tx: mpsc::Sender<CanaryEvent>,
    rx: mpsc::Receiver<CanaryEvent>
}

impl Client {
    pub fn new(out: Sender) -> Client {
        let (local_tx, local_rx) = mpsc::channel();

        Client {
            out: out,
            tx: local_tx,
            rx: local_rx
        }
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket")

        // loop {
        //     let to_broadcast = self.rx.recv().unwrap();
        //     // self.out.send(format!("{:?}", to_broadcast));
        //     self.out.send("broadcasted message");
        // }

        // Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        Ok(())
    }
}
