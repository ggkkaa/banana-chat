use std::cell::RefCell;
use std::io::{self, Write, Read, Error};
use std::net::TcpStream;
use std::rc::Rc;
use std::thread;

use gtk::{prelude::*, Button, Entry, Box};
use gtk::{glib, Application, ApplicationWindow};


const APP_ID: &str = "org.banan.chat";

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
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Banana Chat")
        .build();

        let hbox = Box::new(gtk::Orientation::Horizontal, 10);
        
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Message ..."));
        let stream_clone = stream.clone();

    let entry_clone = entry.clone();
        let button = Button::with_label("Send");
        button.connect_clicked(move |_| {
            let text = entry_clone.text();
            let mut stream = stream_clone.borrow_mut();
            if let Err(err) = stream.write_all(text.as_bytes()) {
                eprintln!("Failed to send message: {}", err);
            }
            entry_clone.set_text("");
        });

        hbox.append(&entry);
        hbox.append(&button);

        window.set_child(Some(&hbox));

    // Present window
    window.present();
}