use crate::genterms::BNodeMap;
use std::collections::HashMap;
use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, BlankNode, NamedOrBlankNode, Term, NamedNode, LiteralRef, Literal};
//use crate::entailment_map::Entailment;
use crate::interpreter::{RIFInterpreter, SimpleInterpreter, RDFInterpreter, RDFSInterpreter, DInterpreter, OWLRDFBasedInterpreter, OWLDirectInterpreter, MyInterpreter};

#[derive(Debug, PartialEq)]
pub enum RIFTerm {
    IRI(String),
    TypedLiteral(String, Option<String>),
    LangLiteral(String, String),
    List(Vec<RIFTerm>),
    Local(String),
    Var,
}

impl RIFTerm {
    pub fn empty_list() -> Self {
        RIFTerm::List(Vec::new())
    }
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


pub struct RIFIData {
    bnodemap: HashMap<String, BlankNode>,
    interpreter: Box<dyn MyInterpreter>,
}

use crate::entailment_map::Entailment;
use crate::entailment_map::Entailment::{Simple, RDF, RDFS, D,
                                    OWLRDFBased, RIF, OWLDirect};

impl RIFIData {
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
        Some(RIFIData {
            bnodemap: HashMap::new(),
            interpreter: int,
        })
    }
    
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


impl BNodeMap for &mut RIFIData {
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
                x.value().to_string(),
                y.to_string()
            )
        } else {
            RIFTerm::TypedLiteral(
                x.value().to_string(),
                Some(x.datatype().as_str().to_owned()),
            )
        }
    }
}

impl From<Literal> for RIFTerm {
    fn from(x: Literal) -> Self {
        if let Some(y) = x.language() {
            RIFTerm::LangLiteral(
                x.value().to_string(),
                y.to_string()
            )
        } else {
            RIFTerm::TypedLiteral(
                x.value().to_string(),
                Some(x.datatype().as_str().to_owned()),
            )
        }
    }
}

impl From<Term> for RIFTerm {
    fn from(x: Term) -> Self {
        match x {
            Term::Literal(x) => x.into(),
            Term::NamedNode(x) => RIFTerm::IRI(x.as_str().to_owned()),
            Term::BlankNode(x) => RIFTerm::Local(x.as_str().to_owned()),
        }
    }
}

impl From<NamedNode> for RIFTerm {
    fn from(x: NamedNode) -> Self {
        RIFTerm::IRI(x.as_str().to_owned())
    }
}

impl From<NamedOrBlankNode> for RIFTerm {
    fn from(x: NamedOrBlankNode) -> Self {
        match x {
            NamedOrBlankNode::NamedNode(x) => RIFTerm::IRI(x.as_str().to_owned()),
            NamedOrBlankNode::BlankNode(x) => RIFTerm::Local(x.as_str().to_owned()),
        }
    }
}


impl From<TermRef<'_>> for RIFTerm {
    fn from(x: TermRef<'_>) -> Self {
        match x {
            TermRef::Literal(x) => x.into(),
            TermRef::NamedNode(x) => RIFTerm::IRI(x.as_str().to_owned()),
            TermRef::BlankNode(x) => RIFTerm::Local(x.as_str().to_owned()),
        }
    }
}

impl From<NamedNodeRef<'_>> for RIFTerm {
    fn from(x: NamedNodeRef<'_>) -> Self {
        RIFTerm::IRI(x.as_str().to_owned())
    }
}

impl From<NamedOrBlankNodeRef<'_>> for RIFTerm {
    fn from(x: NamedOrBlankNodeRef<'_>) -> Self {
        match x {
            NamedOrBlankNodeRef::NamedNode(x) => RIFTerm::IRI(x.as_str().to_owned()),
            NamedOrBlankNodeRef::BlankNode(x) => RIFTerm::Local(x.as_str().to_owned()),
        }
    }
}
