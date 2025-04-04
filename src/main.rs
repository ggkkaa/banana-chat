use std::cell::RefCell;
use std::io::{self, Write, Read, Error};
use std::net::TcpStream;
use std::path::Path;
use std::rc::Rc;
use std::thread;

use gtk::{prelude::*, Button, Entry};
use gtk::{glib, Application, ApplicationWindow, Builder, Window};
use gdk::Key;

const APP_ID: &str = "com.bananymous.chat";

fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("88.193.139.141:6969").unwrap();
    println!("Connected to the server!");

    let app = Application::builder().application_id(APP_ID).build();
    let gtk_stream = Rc::new(RefCell::new(stream.try_clone().unwrap()));
    app.connect_activate(move |app|  build_ui(app, gtk_stream.clone()));

    app.run();

    let mut out_stream = stream.try_clone().unwrap();
    let _read_thread = thread::spawn(move || {
        let mut buffer = [0; 1024]; 
        loop {
            match out_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed the connection.");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", message);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
                
            }
        }
    });


    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
        input.clear();
    }
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


    button.connect_clicked(move |_| {    
        let mut stream_borrow = stream.borrow_mut();
        stream_borrow.write_all(send_field.text().as_bytes()).unwrap();
    });

    window.set_application(Some(app));

    window.present();
}