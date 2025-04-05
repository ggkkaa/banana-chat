/*
 * Copyright � 2025 ggkkaa
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \u201cSoftware\u201d), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::cell::RefCell;
use std::io::{self, Write, Read, Error};
use std::net::TcpStream;
use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use gtk::{prelude::*, Button, Entry, Label, ListBox};
use gtk::{glib, Application, ApplicationWindow, Builder, Window};
use gdk::Key;
use url::Url;

const APP_ID: &str = "com.bananymous.chat";

fn main() -> Result<(), Error> {
    let url = Url::parse("tcp://chat.bananymous.com:6969").expect("Invalid URL");

    let host = url.host_str().expect("No host in URL");
    let port = url.port().unwrap_or(6969);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).unwrap();
    stream.set_nonblocking(true)?;
    println!("Connected to the server!");

    let app = Application::builder().application_id(APP_ID).build();
    let gtk_stream = Rc::new(RefCell::new(stream.try_clone().unwrap()));
    app.connect_activate(move |app|  build_ui(app, gtk_stream.clone()));

    app.run();

    Result::Ok(())
}

fn build_ui(app: &Application, stream: Rc<RefCell<TcpStream>>) {
    let ui_file = Path::new("/home/luka/Documents/banana-chat/target/debug/asssets/ui/main.ui");
    let builder = Builder::from_file(ui_file);

    let window: Window = builder.object("window").expect("Failed to get window from UI file");
    window.set_decorated(true);
    window.set_default_width(800);
    window.set_default_height(600);

    let button: Button = builder.object("send_button").expect("Failed to get button from UI file");
    let send_field: Entry = builder.object("entry").expect("Failed to get entry");
    let chatlist: ListBox = builder.object("chatlist").expect("Failed to get chatlist from UI file");
    
    let join_msg = Label::new(Some("Welcome to Banana Chat!"));
    join_msg.set_xalign(0.0);

    chatlist.append(&join_msg);

    let event_controller = gtk::EventControllerKey::new();

    let event_send_field = send_field.clone();

    let stream_event = stream.clone();

    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            Key::Return => {
                let mut stream_event = stream_event.borrow_mut();

                stream_event.write_all(event_send_field.text().as_bytes()).unwrap();
            }
            _ => {}
        }
        glib::Propagation::Proceed
    });

    send_field.add_controller(event_controller);

    let stream_button = stream.clone();
    button.connect_clicked(move |_| {    
        let mut stream_button = stream_button.borrow_mut();
        stream_button.write_all(send_field.text().as_bytes()).unwrap();
        send_field.set_text("");
    });

    glib::timeout_add_local(Duration::from_millis(500), move || {
        let mut stream_timeout = stream.borrow_mut();

        let mut buf = [0u8; 1024];

        match stream_timeout.read(&mut buf) {
            Ok(0) => {
                println!("Server closed the connection.");
                return glib::ControlFlow::Break;
            }
            Ok(n) => {
                let text = String::from_utf8_lossy(&buf[..n]);
                let msg = Label::new(Some(&text));
                msg.set_xalign(0.0);
                chatlist.append(&msg);
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
            }
        }

        glib::ControlFlow::Continue
    });

    window.set_application(Some(app));

    window.present();
}