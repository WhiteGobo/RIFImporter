use std::collections::HashMap;
use crate::genterms::BNodeMap;
use oxrdf::{
    NamedNodeRef, TermRef, NamedOrBlankNodeRef, Triple, TripleRef, Graph,
    BlankNode,
};
use crate::rifidata::RIFIData;

pub struct RIFIGraph {
    bnodemap: HashMap<String, BlankNode>,
    data: Graph
}

impl RIFIGraph {
    pub fn new() -> Self {
        RIFIGraph {
            bnodemap: HashMap::new(),
            data: Graph::new(),
        }
    }

    pub fn add<'a>(&mut self, subject: NamedOrBlankNodeRef<'a>,
        predicate: NamedNodeRef<'a>,
        object: TermRef<'a>)
    {
        let t = TripleRef::new(subject, predicate, object);
        self.data.insert(t);
    }

    pub fn to_RIFIData(self, entailment: Option<&str>) -> Option<RIFIData> {
        RIFIData::from_graph(self.data, entailment)
    }
}

impl BNodeMap for &mut RIFIGraph {
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
