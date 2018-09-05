extern crate gio;
use gui::gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Moon-rs Browser");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);

    window.show_all();
}
