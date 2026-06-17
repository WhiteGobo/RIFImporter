use crate::genterms::BNodeMap;
use std::ffi::CString;
use std::collections::HashMap;
use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, BlankNode, NamedOrBlankNode, Term, NamedNode, LiteralRef, Literal};
//use crate::entailment_map::Entailment;
use crate::interpreter::{RIFInterpreter, SimpleInterpreter, RDFInterpreter, RDFSInterpreter, DInterpreter, OWLRDFBasedInterpreter, OWLDirectInterpreter, MyInterpreter};

#[derive(Debug, PartialEq)]
pub enum RIFTerm {
    IRI(CString),
    TypedLiteral(CString, Option<CString>),
    LangLiteral(CString, CString),
    List(Vec<RIFTerm>),
    Local(CString),
    Var,
}

impl RIFTerm {
    pub fn empty_list() -> Self {
        RIFTerm::List(Vec::new())
    }
}

#[derive(Debug)]
pub enum Formula {
    Atom(Atom),
    Frame(Frame),
    Subclass(Subclass),
    Member(Member),
    Equal(Equal),
}

#[derive(Debug)]
pub struct Atom {
    pub op: RIFTerm,
    pub args: Vec<RIFTerm>,
}

#[derive(Debug)]
pub struct Frame {
    pub object: RIFTerm,
    pub slotkey: RIFTerm,
    pub slotvalue: RIFTerm,
}

#[derive(Debug)]
pub struct Subclass {
    pub sub: RIFTerm,
    pub super_: RIFTerm,
}

#[derive(Debug)]
pub struct Member {
    pub instance: RIFTerm,
    pub class: RIFTerm,
}

#[derive(Debug)]
pub struct Equal {
    pub left: RIFTerm,
    pub right: RIFTerm,
}


/*
pub enum Interpreter {
    Simple(SimpleInterpreter),
    RDF(RDFInterpreter),
    RDFS(RDFSInterpreter),
    D(DInterpreter),
    OWLRDFBased(OWLRDFBasedInterpreter),
    RIF(RIFInterpreter),
    OWLDirect(OWLDirectInterpreter),
}
*/


pub struct RIFIDataOld {
    bnodemap: HashMap<String, BlankNode>,
    interpreter: Box<dyn MyInterpreter>,
}

use crate::entailment_map::Entailment;

impl Iterator for RIFIDataOld {
    type Item = Formula;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_next_atom(RIFTerm::Var, None){
            Some(x) => {return Some(Formula::Atom(x));},
            None => {},
        }
        match self.get_next_frame(RIFTerm::Var, RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Frame(x));},
            None => {},
        }
        match self.get_next_subclass(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Subclass(x));},
            None => {},
        }
        match self.get_next_member(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Member(x));},
            None => {},
        }
        match self.get_next_equal(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Equal(x));},
            None => {},
        }
        None
    }
}


impl RIFIDataOld {
    /*
    pub fn new(entailment: Option<&str>) -> Option<Self> {
        //use Interpreter as ip;
        let int: Box<dyn MyInterpreter> = match entailment {
            None => Box::new(RIFInterpreter::new()),
            Some(ent) => match Entailment::from(ent) {
                Some(RIF) => Box::new(RIFInterpreter::new()),
                Some(Simple) => Box::new(SimpleInterpreter::new()),
                Some(RDF) => Box::new(RDFInterpreter::new()),
                Some(RDFS) => Box::new(RDFSInterpreter::new()),
                Some(D) => Box::new(DInterpreter::new()),
                Some(OWLRDFBased) => Box::new(OWLRDFBasedInterpreter::new()),
                Some(OWLDirect) => Box::new(OWLDirectInterpreter::new()),
                None => {return None},
            }
        };
        Some(RIFIDataOld {
            bnodemap: HashMap::new(),
            interpreter: int,
        })
    }
    */
    pub fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>)
    {
        self.interpreter.add(subject, predicate, object)
    }

    pub fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        self.interpreter.get_next_atom(op, args)
    }

    pub fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        self.interpreter.get_next_frame(object, slotkey, slotvalue)
    }

    pub fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        self.interpreter.get_next_subclass(sub, super_)
    }
    pub fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        self.interpreter.get_next_member(instance, class)
    }
    pub fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        self.interpreter.get_next_equal(left, right)
    }
}


impl BNodeMap for &mut RIFIDataOld {
    fn get_bnode(self, key: &str) -> BlankNode
    {
        let m = &mut self.bnodemap;
        match m.get(key){
            Some(bnode) => bnode.clone(),
            None => {
                let bnode = BlankNode::default();
                m.insert(key.to_owned(), bnode.clone());
                bnode
            },
        }
    }
}

impl From<LiteralRef<'_>> for RIFTerm {
    fn from(x: LiteralRef<'_>) -> Self {
        if let Some(y) = x.language() {
            RIFTerm::LangLiteral(
                CString::new(x.value()).unwrap(),
                CString::new(y).unwrap(),
            )
        } else {
            RIFTerm::TypedLiteral(
                CString::new(x.value()).unwrap(),
                Some(CString::new(x.datatype().as_str()).unwrap()),
            )
        }
    }
}

impl From<Literal> for RIFTerm {
    fn from(x: Literal) -> Self {
        if let Some(y) = x.language() {
            RIFTerm::LangLiteral(
                CString::new(x.value()).unwrap(),
                CString::new(y).unwrap(),
            )
        } else {
            RIFTerm::TypedLiteral(
                CString::new(x.value()).unwrap(),
                Some(CString::new(x.datatype().as_str()).unwrap()),
            )
        }
    }
}

impl From<Term> for RIFTerm {
    fn from(x: Term) -> Self {
        match x {
            Term::Literal(x) => x.into(),
            Term::NamedNode(x)
                => RIFTerm::IRI(CString::new(x.as_str()).unwrap()),
            Term::BlankNode(x)
                => RIFTerm::Local(CString::new(x.as_str()).unwrap()),
        }
    }
}

impl From<NamedNode> for RIFTerm {
    fn from(x: NamedNode) -> Self {
        RIFTerm::IRI(CString::new(x.as_str()).unwrap())
    }
}

impl From<NamedOrBlankNode> for RIFTerm {
    fn from(x: NamedOrBlankNode) -> Self {
        match x {
            NamedOrBlankNode::NamedNode(x)
                => RIFTerm::IRI(CString::new(x.as_str()).unwrap()),
            NamedOrBlankNode::BlankNode(x)
                => RIFTerm::Local(CString::new(x.as_str()).unwrap()),
        }
    }
}


impl From<TermRef<'_>> for RIFTerm {
    fn from(x: TermRef<'_>) -> Self {
        match x {
            TermRef::Literal(x) => x.into(),
            TermRef::NamedNode(x)
                => RIFTerm::IRI(CString::new(x.as_str()).unwrap()),
            TermRef::BlankNode(x)
                => RIFTerm::Local(CString::new(x.as_str()).unwrap()),
        }
    }
}

impl From<NamedNodeRef<'_>> for RIFTerm {
    fn from(x: NamedNodeRef<'_>) -> Self {
        RIFTerm::IRI(CString::new(x.as_str()).unwrap())
    }
}

impl From<NamedOrBlankNodeRef<'_>> for RIFTerm {
    fn from(x: NamedOrBlankNodeRef<'_>) -> Self {
        match x {
            NamedOrBlankNodeRef::NamedNode(x)
                => RIFTerm::IRI(CString::new(x.as_str()).unwrap()),
            NamedOrBlankNodeRef::BlankNode(x)
                => RIFTerm::Local(CString::new(x.as_str()).unwrap()),
        }
    }
}





pub enum RIFIData {
    Simple(SimpleInterpreter),
    RIF(RIFInterpreter),
    RDF(RDFInterpreter),
    RDFS(RDFSInterpreter),
    D(DInterpreter),
    OWLRDFBased(OWLRDFBasedInterpreter),
    OWLDirect(OWLDirectInterpreter),
}

use oxrdf::Graph;

impl RIFIData {
    pub fn from_graph(graph: Graph, entailment: Option<&str>) -> Option<Self> {
        use RIFIData as RD;
        use crate::entailment_map::Entailment::{Simple, RDF, RDFS, D,
                                            OWLRDFBased, RIF, OWLDirect};
        Some(match entailment {
            None => RD::RIF(RIFInterpreter::new(graph)),
            Some(ent) => match Entailment::from(ent) {
                Some(RIF) => RD::RIF(RIFInterpreter::new(graph)),
                Some(Simple) => RD::Simple(SimpleInterpreter::new(graph)),
                Some(RDF) => RD::RDF(RDFInterpreter::new(graph)),
                Some(RDFS) => RD::RDFS(RDFSInterpreter::new(graph)),
                Some(D) => RD::D(DInterpreter::new(graph)),
                Some(OWLRDFBased) => RD::OWLRDFBased(OWLRDFBasedInterpreter::new(graph)),
                Some(OWLDirect) => RD::OWLDirect(OWLDirectInterpreter::new(graph)),
                None => {return None},
            }
        })
    }

    pub fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        use RIFIData::{Simple, RIF, RDF, RDFS, D, OWLRDFBased, OWLDirect};
        match self {
            Simple(x) => x.get_next_atom(op, args),
            RIF(x) => x.get_next_atom(op, args),
            RDF(x) => x.get_next_atom(op, args),
            RDFS(x) => x.get_next_atom(op, args),
            D(x) => x.get_next_atom(op, args),
            OWLRDFBased(x) => x.get_next_atom(op, args),
            OWLDirect(x) => x.get_next_atom(op, args),
        }
    }

    pub fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        use RIFIData::{Simple, RIF, RDF, RDFS, D, OWLRDFBased, OWLDirect};
        match self {
            Simple(x) => x.get_next_frame(object, slotkey, slotvalue),
            RIF(x) => x.get_next_frame(object, slotkey, slotvalue),
            RDF(x) => x.get_next_frame(object, slotkey, slotvalue),
            RDFS(x) => x.get_next_frame(object, slotkey, slotvalue),
            D(x) => x.get_next_frame(object, slotkey, slotvalue),
            OWLRDFBased(x) => x.get_next_frame(object, slotkey, slotvalue),
            OWLDirect(x) => x.get_next_frame(object, slotkey, slotvalue),
        }
    }

    pub fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        use RIFIData::{Simple, RIF, RDF, RDFS, D, OWLRDFBased, OWLDirect};
        match self {
            Simple(x) => x.get_next_subclass(sub, super_),
            RIF(x) => x.get_next_subclass(sub, super_),
            RDF(x) => x.get_next_subclass(sub, super_),
            RDFS(x) => x.get_next_subclass(sub, super_),
            D(x) => x.get_next_subclass(sub, super_),
            OWLRDFBased(x) => x.get_next_subclass(sub, super_),
            OWLDirect(x) => x.get_next_subclass(sub, super_),
        }
    }
    pub fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        use RIFIData::{Simple, RIF, RDF, RDFS, D, OWLRDFBased, OWLDirect};
        match self {
            Simple(x) => x.get_next_member(instance, class),
            RIF(x) => x.get_next_member(instance, class),
            RDF(x) => x.get_next_member(instance, class),
            RDFS(x) => x.get_next_member(instance, class),
            D(x) => x.get_next_member(instance, class),
            OWLRDFBased(x) => x.get_next_member(instance, class),
            OWLDirect(x) => x.get_next_member(instance, class),
        }
    }
    pub fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        use RIFIData::{Simple, RIF, RDF, RDFS, D, OWLRDFBased, OWLDirect};
        match self {
            Simple(x) => x.get_next_equal(left, right),
            RIF(x) => x.get_next_equal(left, right),
            RDF(x) => x.get_next_equal(left, right),
            RDFS(x) => x.get_next_equal(left, right),
            D(x) => x.get_next_equal(left, right),
            OWLRDFBased(x) => x.get_next_equal(left, right),
            OWLDirect(x) => x.get_next_equal(left, right),
        }
    }
}

impl Iterator for RIFIData {
    type Item = Formula;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_next_atom(RIFTerm::Var, None){
            Some(x) => {return Some(Formula::Atom(x));},
            None => {},
        }
        match self.get_next_frame(RIFTerm::Var, RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Frame(x));},
            None => {},
        }
        match self.get_next_subclass(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Subclass(x));},
            None => {},
        }
        match self.get_next_member(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Member(x));},
            None => {},
        }
        match self.get_next_equal(RIFTerm::Var, RIFTerm::Var) {
            Some(x) => {return Some(Formula::Equal(x));},
            None => {},
        }
        None
    }
}
