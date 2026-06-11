use std::io::Error;
use std::ffi::CString;
use crate::shared::{RIFTerm};
use crate::vocab::{rif};
use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, Graph, NamedOrBlankNode, Term};


pub fn retrieve_rifterm(graph: &Graph, root: NamedOrBlankNodeRef,
    ) -> Option<RIFTerm>
{
    let mut const_iri: Option<CString> = None;
    let mut value: Option<CString> = None;
    let mut items: Option<NamedOrBlankNodeRef> = None;
    let mut lang: Option<CString> = None;
    let mut valuetype: Option<CString> = None;
    let mut var: Option<CString> = None;
    for x in graph.triples_for_subject(root){
        let pred: NamedNodeRef = x.predicate;
        match pred {
            rif::CONSTIRI => {
                const_iri = match x.object {
                    TermRef::Literal(iri) => Some(CString::new(iri.value()).unwrap()),
                    _ => {return None;},
                };
            },
            rif::VALUE => {
                let val = match x.object {
                    TermRef::Literal(iri) => iri,
                    _ => {return None;},
                };
                value = Some(CString::new(val.value()).unwrap());
                match val.language() {
                    Some(l) => {lang = Some(CString::new(l).unwrap());},
                    None => {
                        valuetype = Some(CString::new(val.datatype().as_str()).unwrap());
                    },
                }
            },
            rif::ITEMS => {
                let root: NamedOrBlankNodeRef = match x.object{
                    TermRef::NamedNode(x) => x.into(),
                    TermRef::BlankNode(x) => x.into(),
                    _ => {return None;},
                };
                items = Some(root);
            }
            rif::VARNAME => {
                var = match x.object {
                    TermRef::Literal(iri) => Some(CString::new(iri.value()).unwrap()),
                    _ => {return None;},
                };
            }
            _ => {},
        }
    }
    match (const_iri, value, valuetype, lang, var, items) {
        (Some(x), None, None, None, None, None)
            => Some(RIFTerm::IRI(x)),
        (None, Some(x), None, None, None, None)
            => Some(RIFTerm::TypedLiteral(x, None)),
        (None, Some(x), Some(y), None, None, None)
            => Some(RIFTerm::TypedLiteral(x, Some(y))),
        (None, Some(x), None, Some(y), None, None)
            => Some(RIFTerm::LangLiteral(x, y)),
        (None, None, None, None, Some(_), None)
            => Some(RIFTerm::Var),
        (None, None, None, None, None, Some(x))
            => match riftermlist_to_vec(graph, x) {
                Some(x) => Some(RIFTerm::List(x)),
                None => None,
            },
        _ => None,
    }
}

pub fn rdfidlist_to_vec(graph: &Graph, root: NamedOrBlankNodeRef,
    ) -> Result<Vec<NamedOrBlankNode>, Error>
{
    let list = match rdflist_to_vec(graph, root){
        Some(x) => x,
        None => {return Err(Error::other("not a rdflist"));},
    };
    let mut ret: Vec<NamedOrBlankNode> = Vec::new();
    for x in list {
        match x {
            Term::BlankNode(n) => {ret.push(n.into());},
            Term::NamedNode(n) => {ret.push(n.into());},
            _ => {
                return Err(Error::other("not a named or blank node in list"));
            },
        }
    }
    Ok(ret)
}

pub fn rdflist_to_vec(graph: &Graph, root: NamedOrBlankNodeRef,
    ) -> Option<Vec<Term>>
{
    use oxrdf::vocab::rdf;

    let mut ret1 = Vec::new();
    let mut tmp: NamedOrBlankNodeRef = root.into();
    let rdf_nil: NamedOrBlankNodeRef = rdf::NIL.into();
    while tmp != rdf_nil {
        let first = graph.object_for_subject_predicate(tmp, rdf::FIRST)?;
        let rest: NamedOrBlankNodeRef
            = match graph.object_for_subject_predicate(tmp, rdf::REST) {
                Some(TermRef::NamedNode(x)) => x.into(),
                Some(TermRef::BlankNode(x)) => x.into(),
                _ => {return None;},
            };
        ret1.push(first.into());
        tmp = rest;
    }
    Some(ret1)
}

pub fn rdflist_to_riftermvec(graph: &Graph, root: NamedOrBlankNodeRef,
    ) -> Option<Vec<RIFTerm>> 
{
    let ret1 = rdflist_to_vec(graph, root)?;
    let mut ret2 = Vec::new();
    for x in ret1 {
        ret2.push(x.into());
    }
    Some(ret2)
}

pub fn riftermlist_to_vec(graph: &Graph, root: NamedOrBlankNodeRef,
    ) -> Option<Vec<RIFTerm>> 
{
    let ret1 = rdflist_to_vec(graph, root)?;
    let mut ret2 = Vec::new();
    for x in ret1 {
        let root: NamedOrBlankNode = x.try_into().ok()?;
        ret2.push(retrieve_rifterm(graph, root.as_ref())?);
    }
    Some(ret2)
}


pub fn debug_node(graph: &Graph, node: NamedOrBlankNodeRef) {
    eprintln!("debug node:");
    for t in graph.triples_for_subject(node){
        eprintln!("{}", t);
    }
}
