// gui.rs
// This file contains all functions related to building the Graphical User
// interface using GTK

extern crate gio;
use gui::gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Menu, MenuBar, MenuItem, MenuItemExt, Application, ApplicationWindow, AboutDialog};

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

    build_menu(&window);
    build_address_bar(&window);
    build_text_box(&window);

    window.show_all();
}


fn build_menu(window: &gtk::ApplicationWindow) {
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu_bar = MenuBar::new();
    let file_menu = Menu::new();
    let file = MenuItem::new_with_label("File");
    let new = MenuItem::new_with_label("New");
    let open = MenuItem::new_with_label("Open");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");

    file_menu.append(&new);
    file_menu.append(&open);
    file_menu.append(&about);
    file_menu.append(&quit);

    file.set_submenu(Some(&file_menu));
    menu_bar.append(&file);

    v_box.pack_start(&menu_bar, false, false, 0);
    window.add(&v_box);

    // Menu Item Functionality
    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Ethereum Foundation"]);
        p.set_website_label(Some("Ethereum Foundation"));
        p.set_website(Some("http://ethereum.org"));
        p.set_title("About");
        //p.set_transient_for(Some(window)); // <==== This is giving an error. Investigate
        p.run();
        p.destroy();
    });

    quit.connect_activate(clone!(window => move |_| {
        window.destroy()
    }));
}

fn build_address_bar(window: &gtk::ApplicationWindow) {
    println!("build_address_bar");
}

fn build_text_box(window: &gtk::ApplicationWindow) {
    println!("build_text_box");
}
