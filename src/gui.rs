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

use gio;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{
    ApplicationWindow, Window, Box, Builder, Button, Entry, TreeStore,TreeView,CellRendererText,TreeViewColumn, Statusbar
};
use std::env::args;
use std::cell::RefCell;
use std::rc::Rc;
use crate::wiki_api;

fn build_practice_window(builder: &gtk::Builder) {
    let window: Window = builder.get_object("practice_window").expect("Couldn't get window");
    window.set_size_request(500,100);
    window.connect_delete_event(clone!(window => move |_, _| {
        gtk::WidgetExt::hide_on_delete(&window);
        Inhibit(true)
    }));
}

fn build_prepositions_window(builder: &gtk::Builder) {
    let window: Window = builder.get_object("prepositions_window").expect("Couldn't get window");
    let tree_view: TreeView = builder.get_object("prepositions.treeview_prepositions").expect("Couldn't get treeView");
    let box_prepositions: Box = builder.get_object("prepositions.box_prepositions").expect("Couldn't get treeView");
    let model_prepositions =
			TreeStore::new(&[String::static_type(),String::static_type(),String::static_type()]);

    let status_bar = Statusbar::new();
    box_prepositions.pack_end(&status_bar, false, false, 0);
    status_bar.push(status_bar.get_context_id("prepositions"), "source: https://en.wikibooks.org/wiki/Russian/Prepositions");

    tree_view.set_model(Some(&model_prepositions));
    if tree_view.get_n_columns() == 0 {
		// Column case
		let mut column = TreeViewColumn::new();
		let mut cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 0);
		column.set_title("Case");
		tree_view.append_column(&column);

		// Column preposition
		column = TreeViewColumn::new();
		cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 1);
		column.set_title("Preposition");
		tree_view.append_column(&column);

		// Column translation
		column = TreeViewColumn::new();
		cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 2);
		column.set_title("Translation");
		tree_view.append_column(&column);
        tree_view.set_grid_lines(gtk::TreeViewGridLines::Horizontal);
	}

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Accusative and Prepositional", &"в or во", &"in, inside of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"на", &"on, on top of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Accusative and Instrumental", &"за", &"behind, for"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"под", &"under"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Accusative, Genitive, Instrumental", &"с", &"approximately (acc.), from (gen.), with (inst.)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"по", &"to & including; apiece (acc.), upon, directly after (prep.), along, according to (dat.)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Acusative", &"в", &"in"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"на", &"in"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"c", &"approximately"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"за", &"for"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"о", &"against (physical contact)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"под", &"under"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"по", &"up to, as far as"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"про", &"on the topic of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"сквозь", &"through, across"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"через", &"in, after, by"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Genitive", &"без", &"without"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"близ", &"near, close"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"вдоль", &"along"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"вместо", &"instead of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"вне", &"outside of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"внутри", &"inside, within"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"возле", &"by, near"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"вокруг", &"about, around"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"для", &"for"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"до", &"until"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"из(о)", &"of, outside of, from"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"из-за", &"from behind"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"из-под", &"from below"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"кроме", &"except (for)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"мимо", &"past (movement)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"накануне", &"on the eve"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"около", &"around"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"от(о)", &" off, (away) from"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"после", &"after"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"(на)против", &"against, across from"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"ради", &"for the sake of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"с(о)", &"(down) from"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"(по)среди", &"(down) from"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"у", &"at, close to. Equivalent to the French 'chez'"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Used with Genitive plural", &"мало", &"few, little"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"мало", &"few, little"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"много", &"a lot, many, much"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"несколько", &"a few, not many"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"ско́лько", &"how much? how many?"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Dative", &"к(о)", &"to, towards"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"по", &"many meanings in English; no direct translation"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"благодаря", &"thanks to"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"вопреки", &"contrary to, despite"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"наперекор", &"in defiance of (more intense than вопреки)"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"согласно", &"according to, in accordance with"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Instrumental", &"с(о)", &"with"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"над", &"above, on top of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"под", &"under, beneath"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"за", &"behind, before"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"перед", &"in front of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"между", &"in between"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"", &""]);

    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"Prepositional", &"в", &"in"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"на", &"in"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"при", &"in times of, in the presence of"]);
    model_prepositions.insert_with_values(None, None, &[0, 1, 2], &[&"", &"о(б)", &"about"]);

    window.set_size_request(500,100);
    window.connect_delete_event(clone!(window => move |_, _| {
        gtk::WidgetExt::hide_on_delete(&window);
        Inhibit(true)
    }));
}

fn build_search_window(builder: &gtk::Builder) {
    let window: Window = builder.get_object("search_window").expect("Couldn't get window");
    let btn_search: Button = builder.get_object("search.btn_search_word").expect("Couldn't get search button");

    let box_prepositions: Box = builder.get_object("search.box_search").expect("Couldn't get treeView");
    let status_bar = Statusbar::new();
    box_prepositions.pack_end(&status_bar, false, false, 0);
    status_bar.push(status_bar.get_context_id("search"), "the declension tables are retrieved from https://ru.wiktionary.org/");

    let entry_word: Entry = builder.get_object("search.entry_word").expect("Couldn't get entry word");
    let tree_view: TreeView = builder.get_object("search.treeview_declension").expect("Couldn't get treeView");

    let model_declension =
			TreeStore::new(&[String::static_type(),String::static_type(),String::static_type()]);


    //let ref_entry: Rc<RefCell<gtk::Entry>> = Rc::new(RefCell::new(entry_word));
    tree_view.set_model(Some(&model_declension));

	// Don't create the columns again if they are already there (the user closed the window and opened again)
	if tree_view.get_n_columns() == 0 {
		// Column case
		let mut column = TreeViewColumn::new();
		let mut cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 0);
		column.set_title("Case");
		tree_view.append_column(&column);

		// Column Singular
		column = TreeViewColumn::new();
		cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 1);
		column.set_title("Singular");
		tree_view.append_column(&column);

		// Column Plural
		column = TreeViewColumn::new();
		cell = CellRendererText::new();

		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 2);
		column.set_title("Plural");
		tree_view.append_column(&column);
	}

    let entry_copy = entry_word.clone();
    let model_declension_copy = model_declension.clone();
    let ref_treestore: Rc<RefCell<gtk::TreeStore>> = Rc::new(RefCell::new(model_declension_copy));
    let ref_entry: Rc<RefCell<gtk::Entry>> = Rc::new(RefCell::new(entry_copy));

    btn_search.connect_clicked(move |_| {
        let declensions: Vec<Vec<String>> = match wiki_api::get_declension_table(&ref_entry.borrow().get_text().unwrap()) {
			Ok(table) => table,
			Err(err) => {
				println!("Error: {}",err);
				return;
			}
		};
        ref_treestore.borrow_mut().clear();
        for i in 0..declensions.len() - 1 {
			let case = declensions.get(i).unwrap();
			ref_treestore.borrow_mut().
					insert_with_values(None,
					None,
					&[0, 1, 2],
					&[&wiki_api::get_case_name(i as usize), &case[wiki_api::SINGULAR], &case[wiki_api::PLURAL]]);
		}
		ref_entry.borrow_mut().set_text("");
    });

    window.set_size_request(600,200);

    window.connect_delete_event(clone!(window => move |_, _| {
        gtk::WidgetExt::hide_on_delete(&window);
        Inhibit(true)
    }));
}

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("glade/ru_practice.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get window");
    window.set_application(application);
    let btn_practice: Button = builder.get_object("btn_practice").expect("Couldn't get btn_practice");
    let btn_prepositions: Button = builder.get_object("btn_prepositions").expect("Couldn't get btn_practice");
    let btn_search: Button = builder.get_object("btn_search").expect("Couldn't get btn_search");

	build_practice_window(&builder);
    build_prepositions_window(&builder);
    build_search_window(&builder);

    let builder_clone_practice = builder.clone();
    btn_practice.connect_clicked(move |_| {
		let window: Window = builder_clone_practice.get_object("practice_window").expect("Couldn't get window");
		window.show_all();
    });

    let builder_clone_prepositions = builder.clone();
    btn_prepositions.connect_clicked(move |_| {
		let window: Window = builder_clone_prepositions.get_object("prepositions_window").expect("Couldn't get window");
		window.show_all();
    });

    let builder_clone_search = builder.clone();
    btn_search.connect_clicked(move |_| {
		let window: Window = builder_clone_search.get_object("search_window").expect("Couldn't get window");
		window.show_all();
    });

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));
    window.set_size_request(600,200);
    window.show_all();
}

pub fn main() {
    let application = gtk::Application::new("com.github.ru_practice",
                                            gio::ApplicationFlags::empty())
                                    .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
