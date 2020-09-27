extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::error::Error;
use std::io::Write;
use std::rc::Rc;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use gtk::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    )?;

    let file = std::fs::File::create("bla.log")?;
    let file = Rc::new(RefCell::new(file));

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("My Bla App");
        window.set_default_size(350, 70);

        let file = file.clone();
        let button = Button::new_with_label("Click me!");
        button.connect_clicked(move |_| {
            if let Err(e) = file.borrow_mut().write_all(b"clicked!\n") {
                eprintln!("{}", e)
            }
        });
        window.add(&button);

        window.show_all();
    });

    app.run(&[]);
    Ok(())
}
