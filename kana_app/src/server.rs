//! Implement the internal server used to supply the UI webview.

use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};

/// Implements a basic HTTP server that return a single HTML page
/// based on the content set.
pub struct Server {
    // This is the content that will be served for any request.
    content: Arc<Mutex<String>>,

    // Port the server is listening to.
    port: u16,

    // This is used to send commands to the server.
    tx: Sender<Message>,

    // This channel is closed when the server shutdown is complete.
    closed: Arc<Mutex<RefCell<Receiver<bool>>>>,
}

impl Server {
    /// Returns the port number that the server is listening to.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Stops the server.
    pub fn stop(&self) {
        self.tx.send(Message::Stop).unwrap();
        let cell = self.closed.lock().unwrap();
        for _ in cell.borrow_mut().iter() {}
    }

    /// Set the content returned by the server.
    pub fn set_content(&self, content: &str) {
        let mut data = self.content.lock().unwrap();
        data.truncate(0);
        data.push_str(content);
    }
}

enum Message {
    Stop,
}

/// Starts a new [`Server`] listening to a random port.
pub fn start() -> Server {
    use rouille::Response;
    use rouille::Server;

    let (send_closed, recv_closed) = channel();

    let content = Arc::new(Mutex::new(String::from("Internal server")));

    // Creates the rouille server
    let internal = {
        let content = Arc::clone(&content);
        Server::new("localhost:0", move |_request| {
            let html = content.lock().unwrap();
            Response::html((*html).clone())
        })
        .unwrap()
    };

    let (tx, rx) = channel();

    let server = self::Server {
        content: Arc::clone(&content),
        port: internal.server_addr().port(),
        tx: tx,
        closed: Arc::new(Mutex::new(RefCell::new(recv_closed))),
    };

    // Server loop:

    let poll_delay = time::Duration::from_millis(1);
    thread::spawn(move || {
        loop {
            if let Ok(Message::Stop) = rx.try_recv() {
                break;
            }
            internal.poll();
            thread::sleep(poll_delay);
        }
        drop(send_closed);
    });

    server
}
