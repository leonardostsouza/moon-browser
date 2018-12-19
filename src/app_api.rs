use formality_document;
use formality_document::formality::term::{Defs, Term};
use formality_document::document::*;

#[derive(Debug, Clone)]
pub struct App {
    pub inistate:   Term,
    pub transact:   Term,
    pub render:     Term,
    pub curr_state: Term,
    pub defs:       Option<Defs>
}

impl App {
    // Creates a new App intance based on formality definitions
    // TODO: Receive an AppSpec instead of simple definitions
    pub fn new(raw_defs: Option<&[u8]>) -> App { // Remove "Option"?
        let mut new_app = App::blank();
        new_app.defs = Some(build_defs(raw_defs));

        let defs = &new_app.clone().defs.unwrap().clone();
        let get = |name| get_term(name, defs); // convenience to get terms
        let apply = |func, args| apply(func, args, defs); // convenience to apply terms
        let app = get(b"demo_app"); //Should it be stored?

        new_app.inistate    = apply(get(b"get_app_local_inistate"), vec![app.clone()]);
        new_app.transact    = apply(get(b"get_app_local_transact"), vec![app.clone()]);
        new_app.render      = apply(get(b"get_app_render"), vec![app.clone()]);
        new_app.curr_state  = new_app.inistate.clone();
        new_app
    }

    // Returns a blank App instance
    pub fn blank() -> App {
        let blank_app: App = App {
            inistate:   Term::Set,
            transact:   Term::Set,
            render:     Term::Set,
            curr_state: Term::Set,
            defs:       None
        };
        blank_app
    }

    // Copy values from another struct instance
    pub fn copy(&mut self, other: App){
        *self = App {.. other};
    }

    // TODO: Implement a generic App::apply()
    pub fn apply(&mut self) {
        // local transaction
        match &self.defs {
            Some(app_defs) => {
                let defs = app_defs.clone();
                let get = |name| get_term(name, &defs); // convenience to get terms
                let apply = |func, args| apply(func, args, &defs); // convenience to apply terms

                let local_event = get(b"demo_local_event"); //TODO: Implement generic apply function

                let new_state = apply(self.transact.clone(), vec![local_event.clone(), self.curr_state.clone()]);
                self.curr_state = new_state;
            },
            None => {/*Ignore*/},
        }

    }

    // export App as Formality Document Terms
    pub fn f_doc(&mut self) -> Term {
        let term: Term;
        match &self.defs {
            Some(app_defs) => {
                term = apply(self.render.clone(),
                            vec![self.curr_state.clone()],
                            &app_defs.clone())
            }
            _ => {
                term = Term::Set
            }
        }
        term
    }

    // export App as a Formality Document
    pub fn doc(&mut self) -> Document {
        term_to_document(&self.f_doc()).unwrap()
    }
}
