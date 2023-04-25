use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct CustomButton;

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME : &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}