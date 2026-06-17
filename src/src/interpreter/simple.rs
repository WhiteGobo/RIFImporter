use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, Triple, Graph, TripleRef};
use crate::interpreter::MyInterpreter;
use crate::rifidata::{Atom, Frame, Member, Subclass, Equal, RIFTerm};

pub struct SimpleInterpreter {
    data: Graph,
}
impl SimpleInterpreter {
    pub fn new(data: Graph) -> Self {
        SimpleInterpreter{
            data: data,
        }
    }
}


impl MyInterpreter for SimpleInterpreter {
    fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>)
    {
        let t = TripleRef::new(subject, predicate, object);
        self.data.insert(t);
    }

    fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        let mut ret = None;
        let mut remove: Option<Triple> = None;
        for triple in self.data.iter() {
            let subj: RIFTerm = triple.subject.into();
            let pred: RIFTerm = triple.predicate.into();
            let obj: RIFTerm = triple.object.into();
            if object != RIFTerm::Var && object != subj {
                continue;}
            if slotkey != RIFTerm::Var && slotkey != pred {
                continue;}
            if slotvalue != RIFTerm::Var && slotvalue != obj {
                continue;}
            remove = Some(triple.into());
            ret = Some(Frame{
                object: subj,
                slotkey: pred,
                slotvalue: obj,
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

    fn get_next_member(&mut self, _instance: RIFTerm, _class: RIFTerm,
        ) -> Option<Member>
    {
        None
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
