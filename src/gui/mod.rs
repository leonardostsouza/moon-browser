// gui.rs
// This file contains all functions related to building the Graphical User
// interface using GTK

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, MenuItemExt, Object};

extern crate glib;

extern crate cairo;
use gui::cairo::enums::{FontSlant, FontWeight};
use self::cairo::Context;

mod ipfs;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//*******************************
extern crate rand;
use self::rand::Rng;
//*******************************

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

    let drawing_area: gtk::DrawingArea = object(&builder, "drawingarea1");

    build_menu_bar(&builder, &window);
    build_address_bar(&builder, &drawing_area, &window);

    window.show_all();
}

fn build_menu_bar(builder: &gtk::Builder, window: &gtk::ApplicationWindow) {
    let not_impl_dialog: gtk::MessageDialog = object(&builder, "not-impl-dialog");

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
    let about_dialog: gtk::AboutDialog = object(&builder, "aboutdialog");
    let about: gtk::ImageMenuItem = object(&builder, "imagemenuitem9");
    about.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });
}

fn build_address_bar(builder: &gtk::Builder, drawing_area: &gtk::DrawingArea, window: &gtk::ApplicationWindow) {
    let not_impl_dialog: gtk::MessageDialog = object(&builder, "not-impl-dialog");

    let entry: gtk::Entry = object(&builder, "address-bar");
    entry.connect_activate(clone!(drawing_area, entry => move |_| {
        let hash = entry.get_text().unwrap();
        println!("HASH: {:?}", hash);
        let data = ipfs::block_get(&hash);
        println!("{:?}", data);
        /*drawing_area.get_buffer().expect("Error while loading text buffer")
                                 .set_text(&data);*/
        draw(&drawing_area, "DRAWING_AREA TEST");
        println!("done!")
    }));

    let download: gtk::Button = object(&builder, "download-button");
    download.connect_clicked(clone!(window, entry => move |_| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Save File"), Some(&window), gtk::FileChooserAction::Save);
        file_chooser.add_buttons(&[
            ("Save", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into()),
        ]);

        let response: i32 = gtk::ResponseType::Ok.into();
        if file_chooser.run() == response {
            let hash = entry.get_text().unwrap();
            let data = ipfs::block_get(&hash);

            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let mut file = File::create(&filename).expect("Couldn't save file");
            file.write_all(&data.as_bytes());
        }

        file_chooser.destroy();
    }));

    let upload: gtk::Button = object(&builder, "upload-button");
    upload.connect_clicked(clone!(drawing_area, window => move |_| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"), Some(&window), gtk::FileChooserAction::Open);
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into()),
        ]);

        let response: i32 = gtk::ResponseType::Ok.into();
        if file_chooser.run() == response {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let mut file = File::open(&filename).expect("Couldn't open file");

            let mut contents: String = String::new();
            file.read_to_string(&mut contents).expect("Error while reading file");

            let static_str = Box::leak(contents.into_boxed_str());
            let hash = ipfs::block_put(static_str.as_bytes());
            //drawing_area.get_buffer().expect("Couldn't get window").set_text(&hash);
            println!("Uploaded file hash: {:?}", hash);
        }

        file_chooser.destroy();
    }));

    /*let profile: gtk::Button = object(&builder, "profile-button");
    profile.connect_clicked(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));*/
}

fn draw(drawing_area: &gtk::DrawingArea, shape: &str){
    println!("drawing shape {:?}", shape);

    let surface = cairo::ImageSurface::create(
        cairo::Format::ARgb32, 120, 120)
        .expect("Can't create surface");
    let cr = Context::new(&surface);

    drawing_area.connect_draw(|_, cr| {
        cr.scale(500f64, 500f64);

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(0.04);

        cr.move_to(0.08, 0.08);
        cr.show_text("The quick brown fox jumps over the lazy dog.");

        //cr.set_source_rgb(0.5, 0.5, 1.0);
        //cr.fill_preserve();
        let mut rng = rand::thread_rng();
        let red: f64 = rng.gen_range(0.0, 1.0);
        let green: f64 = rng.gen_range(0.0, 1.0);
        let blue: f64 = rng.gen_range(0.0, 1.0);

        cr.set_source_rgba(red, green, blue, 0.9);
        cr.arc(0.14, 0.53, 0.12, 0.0, 3.14159 * 2.);
        //cr.arc(0.27, 0.65, 0.02, 0.0, 3.14159 * 2.);
        cr.fill();

        Inhibit(false)
    });

    drawing_area.queue_draw();
}
