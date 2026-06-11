pub mod simple;
pub mod rif;
pub mod owlrdfbased;
pub mod owldirect;
pub mod d;
pub mod rdf;
pub mod rdfs;

use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef};
use crate::shared::{Atom, Frame, Member, Subclass, Equal, RIFTerm};

pub trait MyInterpreter {
    fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>);
    fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>;
    fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>;
    fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>;

    fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>;

    fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>;
}


impl MyInterpreter for Box<dyn MyInterpreter + '_> {
    fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>)
    {
        //call add from boxed item
        self.as_mut().add(subject, predicate, object)
    }

    fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        self.as_mut().get_next_atom(op, args)
    }
    fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        self.as_mut().get_next_frame(object, slotkey, slotvalue)
    }
    fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        self.as_mut().get_next_subclass(sub, super_)
    }

    fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        self.as_mut().get_next_member(instance, class)
    }

    fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        self.as_mut().get_next_equal(left, right)
    }
}


pub use owlrdfbased::OWLRDFBasedInterpreter;
pub use owldirect::OWLDirectInterpreter;
pub use simple::SimpleInterpreter;
pub use rif::RIFInterpreter;
pub use rdf::RDFInterpreter;
pub use rdfs::RDFSInterpreter;
pub use d::DInterpreter;

