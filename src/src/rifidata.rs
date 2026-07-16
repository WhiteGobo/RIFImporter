use std::ffi::CString;
use std::collections::HashMap;
use oxrdf::{
    NamedNodeRef, TermRef, NamedOrBlankNodeRef, BlankNode, NamedOrBlankNode,
    Term, NamedNode, LiteralRef, Literal, Graph,
};
use crate::error::GenTermError;
use crate::genterms::BNodeMap;
use crate::entailment_map::Entailment;
use crate::interpreter::{
    RIFInterpreter, SimpleInterpreter, RDFInterpreter, RDFSInterpreter,
    DInterpreter, OWLRDFBasedInterpreter, OWLDirectInterpreter, MyInterpreter,
};
use crate::rififormulas::RIFIFormulas;
use std::iter::zip;

#[derive(Debug, PartialEq)]
pub enum RIFTerm {
    IRI(CString),
    TypedLiteral(CString, Option<CString>),
    LangLiteral(CString, CString),
    List(Vec<RIFTerm>),
    Local(CString),
    Variable(CString),
    Var,
}

impl RIFTerm {
    pub fn empty_list() -> Self {
        RIFTerm::List(Vec::new())
    }

    pub fn get_local(oldvalue: &str) -> Result<Self, GenTermError> {
        let mut value = "l0_".to_owned();
        value.push_str(oldvalue);
        match CString::new(value){
            Ok(x) => Ok(RIFTerm::Local(x)),
            Err(e) => Err(e.into()),
        }
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


pub struct RIFIDataOld {
    bnodemap: HashMap<String, BlankNode>,
    interpreter: Box<dyn MyInterpreter>,
}


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
*/

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
    Formulas(RIFIFormulas),
}


impl RIFIData {
    pub fn from_graph(graph: Graph, entailment: Option<&str>) -> Option<Self> {
        use crate::entailment_map::Entailment::{
            Simple, RDF, RDFS, D, OWLRDFBased, RIF, OWLDirect,
        };
        Some(match entailment {
            None => Self::RIF(RIFInterpreter::new(graph)),
            Some(ent) => match Entailment::from(ent) {
                Some(RIF) => Self::RIF(RIFInterpreter::new(graph)),
                Some(Simple) => Self::Simple(SimpleInterpreter::new(graph)),
                Some(RDF) => Self::RDF(RDFInterpreter::new(graph)),
                Some(RDFS) => Self::RDFS(RDFSInterpreter::new(graph)),
                Some(D) => Self::D(DInterpreter::new(graph)),
                Some(OWLRDFBased) => Self::OWLRDFBased(OWLRDFBasedInterpreter::new(graph)),
                Some(OWLDirect) => Self::OWLDirect(OWLDirectInterpreter::new(graph)),
                None => {return None},
            }
        })
    }

    pub fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        match self {
            Self::Simple(x) => x.get_next_atom(op, args),
            Self::RIF(x) => x.get_next_atom(op, args),
            Self::RDF(x) => x.get_next_atom(op, args),
            Self::RDFS(x) => x.get_next_atom(op, args),
            Self::D(x) => x.get_next_atom(op, args),
            Self::OWLRDFBased(x) => x.get_next_atom(op, args),
            Self::OWLDirect(x) => x.get_next_atom(op, args),
            Self::Formulas(x) => x.get_next_atom(op, args),
        }
    }

    pub fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        match self {
            Self::Simple(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::RIF(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::RDF(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::RDFS(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::D(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::OWLRDFBased(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::OWLDirect(x) => x.get_next_frame(object, slotkey, slotvalue),
            Self::Formulas(x) => x.get_next_frame(object, slotkey, slotvalue),
        }
    }

    pub fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        match self {
            Self::Simple(x) => x.get_next_subclass(sub, super_),
            Self::RIF(x) => x.get_next_subclass(sub, super_),
            Self::RDF(x) => x.get_next_subclass(sub, super_),
            Self::RDFS(x) => x.get_next_subclass(sub, super_),
            Self::D(x) => x.get_next_subclass(sub, super_),
            Self::OWLRDFBased(x) => x.get_next_subclass(sub, super_),
            Self::OWLDirect(x) => x.get_next_subclass(sub, super_),
            Self::Formulas(x) => x.get_next_subclass(sub, super_),
        }
    }
    pub fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        match self {
            Self::Simple(x) => x.get_next_member(instance, class),
            Self::RIF(x) => x.get_next_member(instance, class),
            Self::RDF(x) => x.get_next_member(instance, class),
            Self::RDFS(x) => x.get_next_member(instance, class),
            Self::D(x) => x.get_next_member(instance, class),
            Self::OWLRDFBased(x) => x.get_next_member(instance, class),
            Self::OWLDirect(x) => x.get_next_member(instance, class),
            Self::Formulas(x) => x.get_next_member(instance, class),
        }
    }
    pub fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        match self {
            Self::Simple(x) => x.get_next_equal(left, right),
            Self::RIF(x) => x.get_next_equal(left, right),
            Self::RDF(x) => x.get_next_equal(left, right),
            Self::RDFS(x) => x.get_next_equal(left, right),
            Self::D(x) => x.get_next_equal(left, right),
            Self::OWLRDFBased(x) => x.get_next_equal(left, right),
            Self::OWLDirect(x) => x.get_next_equal(left, right),
            Self::Formulas(x) => x.get_next_equal(left, right),
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

pub fn list_equal_or_valid_for_rifterm(
    query: &Vec<RIFTerm>, target: &Vec<RIFTerm>,
) -> bool {
    if query.len() != target.len() {return false;}
    for (y1, y2) in zip(query, target) {
        if !y1.equal_or_valid_for(y2) {
            return false;
        }
    }
    true
}

impl RIFTerm {
    pub fn equal_or_valid_for(&self, target: &RIFTerm) -> bool {
        match (self, target) {
            (Self::Var, _) => true,
            (Self::IRI(x1), Self::IRI(x2)) => x1 == x2,
            (Self::TypedLiteral(x1, y1), Self::TypedLiteral(x2, y2)) => {
                x1 == x2 && y1 == y2
            },
            (Self::LangLiteral(x1, y1), Self::LangLiteral(x2, y2)) => {
                x1 == x2 && y1 == y2
            },
            (Self::List(x1), Self::List(x2)) => {
                list_equal_or_valid_for_rifterm(x1, x2)
            },
            (Self::Local(x1), Self::Local(x2)) => x1 == x2,
            _ => false,
        }
    }
}
