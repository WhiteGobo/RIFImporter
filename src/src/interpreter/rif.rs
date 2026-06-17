use std::io::Error;
use oxrdf::{NamedNodeRef, TermRef, NamedOrBlankNodeRef, Graph, TripleRef, NamedOrBlankNode};
use crate::interpreter::MyInterpreter;
use crate::rifidata::{Atom, Frame, Member, Subclass, Equal, RIFTerm};
use crate::rdfhelper::{riftermlist_to_vec, retrieve_rifterm, rdfidlist_to_vec};
use oxrdf::vocab::{rdf};
use crate::vocab::{rif};


pub struct RIFInterpreter {
    data: Graph,
    removed: Vec<NamedOrBlankNode>,
}

impl RIFInterpreter {
    pub fn new(data: Graph)-> Self{
        RIFInterpreter{
            data: data,
            removed: Vec::new(),
        }
    }
}

fn get_subroot<'a>(graph: &'a Graph, root: NamedOrBlankNodeRef<'a>, pred: NamedNodeRef<'a>,
    ) -> Result<NamedOrBlankNodeRef<'a>, Error>
{
    match graph.object_for_subject_predicate(root, pred) {
        Some(TermRef::BlankNode(x)) => Ok(x.into()),
        Some(TermRef::NamedNode(x)) => Ok(x.into()),
        Some(_) => Err(Error::other("Not found")),
        _ => Err(Error::other("Found wrong term type")),
    }
}

impl MyInterpreter for RIFInterpreter {
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
        for q in self.data.subjects_for_predicate_object(rdf::TYPE, rif::FRAME){
            let obj_node = match get_subroot(&self.data, q, rif::OBJECT) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken frame, rif:object {:?}", e);
                    continue;
                },
            };
            let found_obj = match retrieve_rifterm(&self.data, obj_node){
                Some(x) => x,
                None => {
                    eprintln!("broken frame, rif:object not a rifterm");
                    continue;
                },
            };
            if object != RIFTerm::Var && object != found_obj {
                continue;
            }
            let slots_node = match get_subroot(&self.data, q, rif::SLOTS) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken frame, rif:slots {:?}", e);
                    continue;
                },
            };

            let slotlist = match rdfidlist_to_vec(&self.data, slots_node) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken frame, slotlist is broken: {:?}", e);
                    continue;
                },
            };
            for slot in slotlist {
                if self.removed.contains(&slot) {continue;}
                let s = slot.as_ref();
                let key_node = match get_subroot(&self.data, s, rif::SLOTKEY) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("broken rif:slot, {:?}", e);
                        continue;
                    },
                };
                let found_key = match retrieve_rifterm(&self.data, key_node){
                    Some(x) => x,
                    None => {
                        eprintln!("broken slot, rif:slotkey not a rifterm");
                        continue;
                    },
                };
                if slotkey != RIFTerm::Var && slotkey != found_key {
                    continue;
                }
                let val_node = match get_subroot(&self.data, s, rif::SLOTVALUE) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("broken rif:slot, {:?}", e);
                        continue;
                    },
                };
                let found_val = match retrieve_rifterm(&self.data, val_node){
                    Some(x) => x,
                    None => {
                        eprintln!("broken slot, rif:slotvalue not a rifterm");
                        continue;
                    },
                };
                if slotvalue != RIFTerm::Var && slotvalue != found_val {
                    continue;
                }
                self.removed.push(slot.clone());
                return Some(
                        Frame{object: found_obj, slotkey: found_key, slotvalue: found_val}
                        );
            }
        }
        None
    }

    fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        for q in self.data.subjects_for_predicate_object(rdf::TYPE, rif::SUBCLASS){
            let n: NamedOrBlankNode = q.into();
            if self.removed.contains(&n) {continue;}
            let sub_node = match get_subroot(&self.data, q, rif::SUB) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken subclass, rif:sub {:?}", e);
                    continue;
                },
            };
            let found_sub = match retrieve_rifterm(&self.data, sub_node){
                Some(x) => x,
                None => {
                    eprintln!("broken subclass, rif:sub not a rifterm");
                    continue;
                },
            };
            if sub != RIFTerm::Var && sub != found_sub {
                continue;
            }
            let super_node = match get_subroot(&self.data, q, rif::SUPER) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken subclass, rif:super {:?}", e);
                    continue;
                },
            };
            let found_super = match retrieve_rifterm(&self.data, super_node){
                Some(x) => x,
                None => {
                    eprintln!("broken subclass, rif:super not a rifterm");
                    continue;
                },
            };
            if super_ != RIFTerm::Var && super_ != found_super {
                continue;
            }
            self.removed.push(n.clone());
            return Some(
                    Subclass{sub: found_sub, super_: found_super}
                    );
        }
        None
    }

    fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        for q in self.data.subjects_for_predicate_object(rdf::TYPE, rif::MEMBER){
            let n: NamedOrBlankNode = q.into();
            if self.removed.contains(&n) {continue;}
            let inst_node = match get_subroot(&self.data, q, rif::INSTANCE) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken member, rif:instance {:?}", e);
                    continue;
                },
            };
            let found_inst = match retrieve_rifterm(&self.data, inst_node){
                Some(x) => x,
                None => {
                    eprintln!("broken member, rif:instance not a rifterm");
                    continue;
                },
            };
            if instance != RIFTerm::Var && instance != found_inst {
                eprintln!("instance is wrong {:?} {:?}", instance, found_inst);
                continue;
            }
            let class_node = match get_subroot(&self.data, q, rif::CLASS) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken member, rif:class {:?}", e);
                    continue;
                },
            };
            let found_class = match retrieve_rifterm(&self.data, class_node){
                Some(x) => x,
                None => {
                    eprintln!("broken member, rif:class not a rifterm");
                    continue;
                },
            };
            if class != RIFTerm::Var && class != found_class {
                eprintln!("class is wrong {:?} {:?}", class, found_class);
                continue;
            }
            self.removed.push(n.clone());
            return Some(
                    Member{instance: found_inst, class: found_class},
                    );
        }
        None
    }

    fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        for q in self.data.subjects_for_predicate_object(rdf::TYPE, rif::EQUAL){
            let mut findarray = Vec::new();
            if left != RIFTerm::Var {findarray.push(&left)};
            if right != RIFTerm::Var {findarray.push(&right)};

            let n: NamedOrBlankNode = q.into();
            if self.removed.contains(&n) {continue;}
            let left_node = match get_subroot(&self.data, q, rif::LEFT) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken equal, rif:left {:?}", e);
                    continue;
                },
            };
            let found_left = match retrieve_rifterm(&self.data, left_node){
                Some(x) => x,
                None => {
                    eprintln!("broken equal, rif:left not a rifterm");
                    continue;
                },
            };
            if found_left != RIFTerm::Var {
                let index = findarray.iter().position(|x| *x == &found_left);
                match index {
                    Some(i) => {findarray.remove(i);}
                    _ => {},
                }
            }
            if findarray.len() > 1 {continue;}
            let right_node = match get_subroot(&self.data, q, rif::RIGHT) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("broken equal, rif:right {:?}", e);
                    continue;
                },
            };
            let found_right = match retrieve_rifterm(&self.data, right_node){
                Some(x) => x,
                None => {
                    eprintln!("broken equal, rif:right not a rifterm");
                    continue;
                },
            };
            if found_right != RIFTerm::Var {
                let index = findarray.iter().position(|x| *x == &found_right);
                match index {
                    Some(i) => {findarray.remove(i);}
                    _ => {},
                }
            }
            if findarray.len() != 0 {continue;}
            self.removed.push(n.clone());
            return Some(
                    Equal{left: found_left, right: found_right},
                    );
        }
        None
    }

    fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        for q in self.data.subjects_for_predicate_object(rdf::TYPE, rif::ATOM){
            let n: NamedOrBlankNode = q.into();
            if self.removed.contains(&n) {continue;}
            let op_node: NamedOrBlankNodeRef
                = match self.data.object_for_subject_predicate(q, rif::OP)
            {
                Some(TermRef::BlankNode(x)) => x.into(),
                Some(TermRef::NamedNode(x)) => x.into(),
                Some(x) => {eprintln!("broken atom1 {}", x); continue;},
                _ => {eprintln!("broken atom1"); continue;},
            };
            let found_op = match retrieve_rifterm(&self.data, op_node){
                Some(x) => x,
                None => {eprintln!("broken atom2"); continue;},
            };
            if op != RIFTerm::Var && op != found_op {
                continue;
            }

            let args_node: Option<NamedOrBlankNodeRef>
                = match self.data.object_for_subject_predicate(q, rif::ARGS)
            {
                Some(TermRef::BlankNode(x)) => Some(x.into()),
                Some(TermRef::NamedNode(x)) => Some(x.into()),
                None => None,
                _ => {continue;},
            };
            let found_args = match args_node {
                None => Vec::new(),
                Some(n) => match riftermlist_to_vec(&self.data, n){
                    Some(x) => x,
                    None => {continue;},
                },
            };
            match args {
                Some(ref arglist) => {
                    if !is_expected_riftermlist(arglist, &found_args){
                        continue;
                    }
                },
                _ => {},
            }
            self.removed.push(n.clone());
            return Some(
                    Atom{op: found_op, args: found_args}
                    );
        }
        None
    }
}


fn is_expected_riftermlist(
    expected_list: &Vec<RIFTerm>, found_list: &Vec<RIFTerm>,
    ) -> bool
{
    use std::iter::zip;
    if found_list.len() != expected_list.len(){
        //eprintln!("wrong length {:?} {:?}", expected_list, found_list);
        return false;
    }
    for (expect, found) in zip(expected_list, found_list){
        match (expect, found) {
            (RIFTerm::Var, _) => {},
            (RIFTerm::List(l1), RIFTerm::List(l2)) => {
                if !is_expected_riftermlist(l1, l2){
                    //eprintln!("sublist doesnt match");
                    return false;
                }
            },
            (x, y) => {if x != y {
                //eprintln!("doesnt match {:?} {:?}", x, y);
                return false;
            }},
        }
    }
    return true;
}
