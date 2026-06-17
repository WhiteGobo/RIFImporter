use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, Triple, TripleRef, Graph};
use oxrdf::vocab::rdf;
use crate::interpreter::MyInterpreter;
use crate::rifidata::{Atom, Frame, Member, Subclass, Equal, RIFTerm};
use crate::rdfhelper::rdflist_to_riftermvec;

pub struct RDFInterpreter {
    data: Graph,
}

impl RDFInterpreter {
    pub fn new(data: Graph) -> Self {
        RDFInterpreter{
            data: data,
        }
    }
}

impl MyInterpreter for RDFInterpreter {
    fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>)
    {
        match subject {
            NamedOrBlankNodeRef::NamedNode(_) => {
                match predicate {
                    rdf::FIRST => {
                        eprintln!("unspecified behaviour on <IRI> rdf:first");
                        return ();
                    },
                    rdf::REST => {
                        eprintln!("unspecified behaviour on <IRI> rdf:rest");
                        return ();
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        let t = TripleRef::new(subject, predicate, object);
        self.data.insert(t);
    }

    fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        let mut ret = None;
        let mut remove: Option<Triple> = None;
        for triple in self.data.iter() {
            match triple.predicate {
                rdf::TYPE => {continue;},
                rdf::FIRST => {continue;},
                rdf::REST => {continue;},
                _ => {},
            }
            let found_subj: RIFTerm
                = match rdflist_to_riftermvec(&self.data, triple.subject) {
                    Some(l) => RIFTerm::List(l),
                    None => triple.subject.into(),
                };
            let found_pred: RIFTerm = triple.predicate.into();
            let found_obj: RIFTerm = match triple.object {
                TermRef::BlankNode(x) => match rdflist_to_riftermvec(&self.data, x.into()){
                    Some(l) => RIFTerm::List(l),
                    None => {triple.object.into()},
                },
                _ => triple.object.into(),
            };
            if object != RIFTerm::Var && object != found_subj {continue;}
            if slotkey != RIFTerm::Var && slotkey != found_pred {continue;}
            if slotvalue != RIFTerm::Var && slotvalue != found_obj {continue;}
            remove = Some(triple.into());
            ret = Some(Frame{
                object: found_subj,
                slotkey: found_pred,
                slotvalue: found_obj,
            });
            break;
        }
        match remove {
            Some(x) => {self.data.remove(x.as_ref());},
            None => {},
        }
        return ret
    }

    fn get_next_subclass(&mut self, _sub: RIFTerm, _super_: RIFTerm,
        ) -> Option<Subclass>
    {
        None
    }

    fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        let mut ret = None;
        let mut remove: Option<Triple> = None;
        for triple in self.data.triples_for_predicate(rdf::TYPE) {
            let subj: RIFTerm = triple.subject.into();
            let obj: RIFTerm = triple.object.into();
            if instance != RIFTerm::Var && instance != subj {continue;}
            if class != RIFTerm::Var && class != obj {continue;}
            remove = Some(triple.into());
            ret = Some(Member{
                instance: subj,
                class: obj,
            });
            break;
        }
        match remove {
            Some(x) => {self.data.remove(x.as_ref());},
            None => {},
        }
        return ret
    }

    fn get_next_equal(&mut self, _left: RIFTerm, _right: RIFTerm,
        ) -> Option<Equal>
    {
        None
    }

    fn get_next_atom(&mut self, _op: RIFTerm, _args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        None
    }
}
