// gui.rs
// This file contains all functions related to building the Graphical User
// interface using GTK

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Menu, MenuBar, MenuItem, MenuItemExt, Application, ApplicationWindow,
    AboutDialog, Inhibit, ObjectExt, WidgetExt, traits::*};

extern crate glib;


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

pub fn build_ui(application: &gtk::Application, width: i32, height: i32) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Moon-rs Browser");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(width, height);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    build_menu_bar(&window, &v_box);
    build_drawing_area(&window, &v_box, width, height);
    build_address_bar(&window, &v_box);

    window.add(&v_box);

    window.show_all();
}


fn build_menu_bar(window: &gtk::ApplicationWindow, v_box: &gtk::Box) {
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

fn build_address_bar(window: &gtk::ApplicationWindow, v_box: &gtk::Box) {
    //println!("build_address_bar");
    let entry = gtk::Entry::new();
    v_box.pack_start(&entry, false, false, 0);

    entry.connect_activate(clone!(entry => move |_| {
            let url = entry.get_text().unwrap();
            println!("URL: {}", url);
            // TODO: GET IPFS WEBPAGE HERE
    }));
}

fn build_drawing_area(window: &gtk::ApplicationWindow, v_box: &gtk::Box, width: i32, height: i32) {
    println!("build_drawing_area");
    let drawing_area = gtk::DrawingArea::new();
    drawing_area.set_size_request(width, height);

    let layout = gtk::Layout::new(None, None);

    let overlay = gtk::Overlay::new();
    {
        use gtk::OverlayExt;
        overlay.add_overlay(&drawing_area);
        overlay.set_child_index(&drawing_area, 0);
        overlay.add_overlay(&layout);
        overlay.set_child_index(&layout, 1);
    }

    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.add(&overlay);
    v_box.pack_end(&scrolled_window, true, true, 0);
}
