// gui.rs
// This file contains all functions related to building the Graphical User
// interface using GTK

use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, MenuItemExt, Object};
use cairo::{Context, Format, ImageSurface, Operator};

use ipfs;
use formality_document::document::*;

use std::fs::File;
use std::io::prelude::*;


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
    builder.get_object(name).expect(&format!("ERROR: Failed to get {}", name)[..])
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
    let entry: gtk::Entry = object(&builder, "address-bar");
    entry.connect_activate(clone!(drawing_area, entry => move |_| {
        let hash = entry.get_text().unwrap();
        println!("DEBUG => Address bar HASH: {:?}", hash);
        let data = ipfs::block_get(&hash);
        println!("DEBUG => Request done. DATA: {:?}", data);
        let doc: Document = serde_json::from_str(&data).unwrap();
        render(&drawing_area, doc);
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
            // TODO: Implement error handling for file.write_all Result.
            file.write_all(&data.as_bytes());
        }

        file_chooser.destroy();
    }));

    let upload: gtk::Button = object(&builder, "upload-button");
    upload.connect_clicked(clone!(window => move |_| {
        // TODO: move this to a impl?
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
            println!("DEBUG: Uploaded file. HASH = {:?}", hash);
        }

        file_chooser.destroy();
    }));

    // ATTENTION: This piece of code was removed temporarily.
    //Please, do not erase the commented section below.
    // let not_impl_dialog: gtk::MessageDialog = object(&builder, "not-impl-dialog");
    /*let profile: gtk::Button = object(&builder, "profile-button");
    profile.connect_clicked(clone!(not_impl_dialog => move |_| {
        not_impl_dialog.run();
        not_impl_dialog.hide();
    }));*/
}

fn render_element(elem: &Element, ctx: &Context) {
    match elem {
        Element::Circle{x, y, r} => {
            ctx.set_source_rgb(0.5, 0.5, 1.0);
            ctx.arc(*x as f64, *y as f64, *r as f64, 0.0, 3.14159 * 2.);
            ctx.fill();
        }
        Element::Square{x, y, r} => {
            ctx.set_source_rgb(0.5, 0.5, 1.0);
            ctx.rectangle(*x as f64, *y as f64, *r as f64, *r as f64);
            ctx.fill();
        }
    }
}

pub fn render(drawing_area: &gtk::DrawingArea, doc: Document){
    println!("DEBUG: drawing document {:?}", doc);
    let surface = ImageSurface::create(Format::ARgb32, 120, 120)
        .expect("ERROR: Can't create surface");
    let ctx = Context::new(&surface);
    ctx.save(); // save default cairo contect state

    drawing_area.connect_draw(move |_, ctx| {
        // Clear painting surface
        ctx.save();
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.set_operator(Operator::Source);
        ctx.paint();
        ctx.restore();

        // Draw formality-document
        for elem in &doc {
            render_element(&elem, &ctx);
        }

        Inhibit(false)
    });

    drawing_area.queue_draw();
    println!("DEBUG: Formality-document render complete");
}
