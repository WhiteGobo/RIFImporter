use oxrdf::{NamedNodeRef, BlankNode, GraphName, NamedOrBlankNode, Term, Literal, NamedNode};
use std::ffi::{c_char, CStr};

pub trait BNodeMap {
    fn get_bnode(self, key: &str) -> BlankNode;
}

pub fn generate_IdentifiedNode(
    value: *const c_char, value_type: u8,
    map: impl BNodeMap,
    ) -> Result<NamedOrBlankNode, ()>
{
    if value.is_null(){
        return Err(());
    }
    let obj = match unsafe{ CStr::from_ptr(value) }.to_str() {
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    match value_type {
        0 => {
            match NamedNode::new(obj){
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            }
        },
        1 => Ok(map.get_bnode(obj).into()),
        _ => Err(())
    }
}

pub fn generate_Term(
    value: *const c_char, value_suffix: *const c_char,
    value_type: u8,
    map: impl BNodeMap,
    ) -> Result<Term, ()>
{
    if value.is_null(){
        return Err(());
    }
    let obj = match unsafe{ CStr::from_ptr(value) }.to_str() {
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    match value_type {
        0 => {
            match NamedNode::new(obj){
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            }
        },
        1 => Ok(map.get_bnode(obj).into()),
        2 => {
            if value_suffix.is_null(){
                Ok(Literal::new_simple_literal(obj).into())
            } else {
                let suf_c = unsafe{ CStr::from_ptr(value_suffix) };
                let suf_iri = match suf_c.to_str() {
                    Ok(suf) => match NamedNode::new(suf) {
                        Ok(x) => x,
                        Err(_) => {return Err(());},
                    },
                    Err(_) => {return Err(());},
                };
                Ok(Literal::new_typed_literal(obj, suf_iri).into())
            }
        }
        3 => {
            if value_suffix.is_null(){
                Ok(Literal::new_language_tagged_literal_unchecked(obj, "").into())
            } else {
                let suf_c = unsafe{ CStr::from_ptr(value_suffix) };
                let suf = match suf_c.to_str() {
                    Ok(x) => x,
                    Err(_) => {return Err(());},
                };
                Ok(Literal::new_language_tagged_literal_unchecked(obj, suf).into())
            }
        }
        _ => Err(())
    }
}

pub fn generate_IRI<'a>(value: *const c_char) -> Result<NamedNodeRef<'a>, ()>
{
    if value.is_null() {
        return Err(());
    }
    let x: &str = match unsafe {CStr::from_ptr(value)}.to_str(){
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    match NamedNodeRef::new(x) {
        Ok(x) => Ok(x),
        Err(_) => Err(()),
    }
}

pub fn generate_Graph<'a>(
    graph_id: *const c_char, graph_type: u8,
    map: impl BNodeMap,
    ) -> Result<GraphName, ()>
{
    if graph_id.is_null(){
        return Ok(GraphName::DefaultGraph);
    }
    let obj = match unsafe{ CStr::from_ptr(graph_id) }.to_str() {
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    match graph_type {
        0 => {
            match NamedNodeRef::new(obj){
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            }
        },
        1 => Ok(map.get_bnode(obj).into()),
        _ => Err(())
    }
}
