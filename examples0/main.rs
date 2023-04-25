#![windows_subsystem = "windows"]

use gtk::prelude::*;
use gtk::Orientation;
use gtk::{glib, Application, ApplicationWindow, Button};
use std::cell::Cell;
use std::rc::Rc;
use gtk::glib::clone;

const APP_ID: &str = "MemoryManagement_Sample";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(application:&Application) {

    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    //let number_copy = number.clone();
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("MemoryManager Sample")
        .child(&gtk_box)
        .build();
    // let button = Button::builder().label("Press Me!").margin_top(12).margin_bottom(12).margin_start(12).margin_end(12).build();

    // button.connect_clicked(|button| {
    //     button.set_label("Hello World!");
    // });

    // let window = ApplicationWindow::builder().application(app).title("My GTK App").child(&button).build();    

     window.present();
}