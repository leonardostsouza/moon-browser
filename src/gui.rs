// gui.rs
// This file contains all functions related to building the Graphical User
// interface using GTK

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, MenuItemExt, Object};

extern crate glib;

// TODO: transfer this include to a custom module
extern crate ipfsapi;
use self::ipfsapi::IpfsApi;


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

fn object<T: IsA<Object>>(builder: &gtk::Builder, name: &str) -> T {
    builder.get_object(name).expect(&format!("Failed to get {}", name)[..])
}

pub fn build_ui(application: &gtk::Application, width: i32, height: i32) {
    let glade_src = include_str!("gui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = object(&builder, "applicationwindow1");
    window.set_default_size(width, height);
    window.set_application(application);
    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    // Changed drawing_area to TextView for testing purposes.
    // TODO: Change type back to gtk::DrawingArea when HTML interpreter is implemented
    let drawing_area: gtk::TextView = object(&builder, "textview1");

    build_menu_bar(&builder, &window);
    build_drawing_area(&builder, &drawing_area);
    build_address_bar(&builder, &drawing_area);

    window.show_all();
}


fn build_menu_bar(builder: &gtk::Builder, window: &gtk::ApplicationWindow) {
    let not_impl_dialog: gtk::MessageDialog = object(&builder, "messagedialog1");

    // File
    let new: gtk::ImageMenuItem = object(&builder, "imagemenuitem1");
    new.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let open: gtk::ImageMenuItem = object(&builder, "imagemenuitem2");
    open.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let save: gtk::ImageMenuItem = object(&builder, "imagemenuitem3");
    save.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let save_as: gtk::ImageMenuItem = object(&builder, "imagemenuitem4");
    save_as.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let quit: gtk::ImageMenuItem = object(&builder, "imagemenuitem5");
    quit.connect_activate(clone!(window => move |_| {
        window.destroy()
    }));


    // Edit
    let cut: gtk::ImageMenuItem = object(&builder, "imagemenuitem6");
    cut.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let copy: gtk::ImageMenuItem = object(&builder, "imagemenuitem7");
    copy.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));

    let paste: gtk::ImageMenuItem = object(&builder, "imagemenuitem8");
    paste.connect_activate(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));


    // Tools
    // -- Nothing here yet


    // Help
    let about_dialog: gtk::AboutDialog = object(&builder, "aboutdialog1");
    let about: gtk::ImageMenuItem = object(&builder, "imagemenuitem9");
    about.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });
}

fn build_drawing_area(builder: &gtk::Builder, drawing_area: &gtk::TextView) {
    println!("build_drawing_area");
}

fn build_address_bar(builder: &gtk::Builder, drawing_area: &gtk::TextView) {
    let entry: gtk::Entry = object(&builder, "entry1");

    entry.connect_activate(clone!(drawing_area, entry => move |_| {
            let hash = entry.get_text().unwrap();
            println!("HASH: {}", hash);
            // TODO: transfer this code to another module
            let api = IpfsApi::new("127.0.0.1", 5001);

            let bytes = api.block_get(&hash).unwrap();
            let data = String::from_utf8(bytes.collect()).unwrap();

            println!("{}", data);
            drawing_area.get_buffer().expect("Error while loading text buffer")
                                     .set_text(&data);
    }));
}
