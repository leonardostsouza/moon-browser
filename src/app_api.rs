use formality_document;
use formality_document::formality::term::{Defs, Term};
use formality_document::document::*;

#[derive(Debug, Clone)]
pub struct App {
    pub inistate:   Term,
    pub transact:   Term,
    pub render:     Term,
    pub curr_state: Term,
    defs:           Defs
}

impl App {
    pub fn new(raw_defs: Option<&[u8]>) -> App { // Remove "Option"?
        let mut new_app =
        App {
            inistate:   Term::Set,
            transact:   Term::Set,
            render:     Term::Set,
            curr_state: Term::Set,
            defs:       build_defs(raw_defs)
        };

        // Update App instance with definitions
        let defs = &new_app.defs.clone();
        let get = |name| get_term(name, defs); // convenience to get terms
        let apply = |func, args| apply(func, args, defs); // convenience to apply terms
        let app = get(b"demo_app"); //Should it be stored?
        new_app.inistate    = apply(get(b"get_app_local_inistate"), vec![app.clone()]);
        new_app.transact    = apply(get(b"get_app_local_transact"), vec![app.clone()]);
        new_app.render      = apply(get(b"get_app_render"), vec![app.clone()]);
        new_app.curr_state  = new_app.inistate.clone();
        new_app
    }

    pub fn apply(&mut self) {
        // local transaction
        let defs = &self.defs.clone();
        let get = |name| get_term(name, defs); // convenience to get terms
        let apply = |func, args| apply(func, args, defs); // convenience to apply terms

        let local_event = get(b"demo_local_event");

        let new_state = apply(self.transact.clone(), vec![local_event.clone(), self.curr_state.clone()]);
        self.curr_state = new_state;
    }

    pub fn f_doc(&self) -> Term {
        apply(self.render.clone(), vec![self.curr_state.clone()], &self.defs.clone())
    }

    pub fn doc(&self) -> Document {
        term_to_document(&self.f_doc()).unwrap()
    }
}
