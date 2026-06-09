pub enum Entailment {
    Simple,
    RDF,
    RDFS,
    D,
    OWLRDFBased,
    RIF,
    OWLDirect,
}

impl Entailment {
    pub fn from(entailment: &str) -> Option<Self> {
        use Entailment as Ent;
        use Entailment::*;
        match entailment {
            "http://www.w3.org/ns/entailment/Simple" => Some(Simple),
            "http://www.w3.org/ns/entailment/RDF" => Some(Ent::RDF),
            "http://www.w3.org/ns/entailment/RDFS" => Some(Ent::RDFS),
            "http://www.w3.org/ns/entailment/D" => Some(Ent::D),
            "http://www.w3.org/ns/entailment/OWL-RDF-Based"
                => Some(Ent::OWLRDFBased),
            "http://www.w3.org/ns/entailment/RIF" => Some(Ent::RIF),
            "http://www.w3.org/ns/entailment/OWL-Direct"
                => Some(Ent::OWLDirect),
            _ => None,
        }
    }
}
