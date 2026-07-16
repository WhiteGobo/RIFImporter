use std::ptr;
use std::ffi::{c_char, CStr, c_uchar, c_void, CString};
use crate::rifidata::{RIFIData, RIFTerm, Atom, Frame, Member, Equal, Subclass};
use crate::rifigraph::RIFIGraph;
use crate::genterms::{
    generate_IdentifiedNode, generate_IRI, generate_Term,
};
use std::io::Error;

use crate::extern_c_structs::{
    RIFITerm,
    RIFITermList,
    RIFIEqual,
    RIFIMember,
    RIFISubclass,
    RIFIFrame,
    RIFIAtom,
    c2rust_rifterm,
    c2rust_riftermlist,
};


///declared in `RIFImporterTermGenerator.h`
unsafe extern "C" {
    pub unsafe fn RIFITerm_new_iri(value: *const c_char) -> *mut RIFITerm;
    pub unsafe fn RIFITerm_new_variable(value: *const c_char) -> *mut RIFITerm;
    pub unsafe fn RIFITerm_new_typedliteral(value: *const c_char, suffix: *const c_char) -> *mut RIFITerm;
    pub unsafe fn RIFITerm_new_langliteral(value: *const c_char, suffix: *const c_char) -> *mut RIFITerm;
    pub unsafe fn RIFITermList_append(old: *mut RIFITermList, new: *mut RIFITerm) -> *mut RIFITermList;
    pub unsafe fn RIFITerm_new_local(value: *const c_char) -> *mut RIFITerm;
    pub unsafe fn RIFIFrame_new(object: *mut RIFITerm, slotkey: *mut RIFITerm, slotvalue: *mut RIFITerm) -> *mut RIFIFrame;
    pub unsafe fn RIFIAtom_new(op: *mut RIFITerm, args: *mut RIFITermList) -> *mut RIFIAtom;
    pub unsafe fn RIFISubclass_new(sub: *mut RIFITerm, super_: *mut RIFITerm) -> *mut RIFISubclass;
    pub unsafe fn RIFIMember_new(instance: *mut RIFITerm, class: *mut RIFITerm) -> *mut RIFIMember;
    pub unsafe fn RIFIEqual_new(left: *mut RIFITerm, right: *mut RIFITerm) -> *mut RIFIEqual;
    pub unsafe fn RIFITerm_new_list(list: *const RIFITermList, rest: *const RIFITerm) -> *mut RIFITerm;
    pub unsafe fn free_RIFITermList(list: *mut RIFITermList);
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIGraph_to_RIFIData(
    graph: *mut RIFIGraph, entailment: *const c_char,
) -> *mut RIFIData {
    if graph.is_null(){
        return ptr::null_mut();
    }
    let ent = if entailment.is_null() {
        None
    } else {
        match unsafe {CStr::from_ptr(entailment)}.to_str() {
            Ok(x) => Some(x),
            Err(e) => {
                eprintln!("Failed to read entailment {:?}", e);
                return ptr::null_mut();
            }
        }
    };
    //consume graph
    let g: RIFIGraph = Box::into_inner(unsafe {Box::from_raw(graph)});
    let x = match g.to_RIFIData(ent) {
        Some(x) => x,
        None => {return ptr::null_mut();},
    };

    let mybox = Box::new(x);
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIGraph_new() -> *mut RIFIGraph
{
    let mybox = Box::new(RIFIGraph::new());
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIGraph_add(
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
        _graph_id: *const c_char, _graph_type: u8,
        data: *mut RIFIGraph,
        ) -> i64
{
    if data.is_null() {return -1;}
    unsafe {
        let subj = match generate_IdentifiedNode(
            subject, subject_type, &mut (*data))
        {
            Ok(x) => x,
            Err(_) => {eprintln!("Failed to translate subject"); return -2;},
        };
        let pred = match generate_IRI(predicate) {
            Ok(x) => x,
            Err(_) => {eprintln!("Failed to translate predicate");return -3;},
        };
        let obj = match generate_Term(
            object, object_suffix, object_type, &mut (*data))
        {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to translate object with: {}", e);
                return -4;
            },
        };
        (*data).add(subj.as_ref(), pred, obj.as_ref());
    }
    return 0;
}


#[unsafe(no_mangle)]
pub extern "C" fn free_RIFIData(data: *mut RIFIData){
    if !data.is_null(){
        unsafe { let _ = Box::from_raw(data); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_remaining(data: *mut RIFIData) -> u64
{
    if data.is_null() {return 0;}
    return 0;
}


#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_atom_any_args(data: *mut RIFIData, op: *const RIFITerm) -> *mut RIFIAtom
{
    if data.is_null() {return ptr::null_mut();}
    let op_q = c2rust_rifterm(op);
    let new_atom = unsafe {
        match (*data).get_next_atom(op_q, None){
            Some(x) => x,
            None => {return ptr::null_mut();},
        }
    };
    match new_atom.to_c_atom() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

fn replace_vars(input: Vec<RIFTerm>) -> Vec<RIFTerm>{
    let mut output = Vec::new();
    for x in input {
        output.push(match x {
            RIFTerm::Variable(_) => RIFTerm::Var,
            _ => x,
        });
    }
    output
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_atom(
    data: *mut RIFIData, op: *const RIFITerm, args: *const RIFITermList,
    ) -> *mut RIFIAtom
{
    if data.is_null() {return ptr::null_mut();}
    let op_q = c2rust_rifterm(op);
    let args_q = replace_vars(c2rust_riftermlist(args));
    let new_atom = unsafe {
        match (*data).get_next_atom(op_q, Some(args_q)){
            Some(x) => x,
            None => {
                return ptr::null_mut();
            },
        }
    };
    match new_atom.to_c_atom() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("RIFImporter: internal error {}", e);
            ptr::null_mut()
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_frame(data: *mut RIFIData, object: *const RIFITerm, slotkey: *const RIFITerm, slotvalue: *const RIFITerm) -> *mut RIFIFrame
{
    if data.is_null() {return ptr::null_mut();}
    let obj_q = c2rust_rifterm(object);
    let key_q = c2rust_rifterm(slotkey);
    let val_q = c2rust_rifterm(slotvalue);
    let new_frame = unsafe {
        match (*data).get_next_frame(obj_q, key_q, val_q){
            Some(x) => x,
            None => {return ptr::null_mut();},
        }
    };
    match new_frame.to_c_frame() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_subclass(data: *mut RIFIData, sub: *const RIFITerm, super_: *const RIFITerm
    ) -> *mut RIFISubclass 
{
    if data.is_null() {return ptr::null_mut();}
    let sub_q = c2rust_rifterm(sub);
    let super_q = c2rust_rifterm(super_);
    let new_subclass = unsafe {
        match (*data).get_next_subclass(sub_q, super_q){
            Some(x) => x,
            None => {return ptr::null_mut();},
        }
    };
    match new_subclass.to_c_subclass() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_member(data: *mut RIFIData, instance: *const RIFITerm, class: *const RIFITerm,
    ) -> *mut RIFIMember
{
    if data.is_null() {return ptr::null_mut();}
    let instance_q = c2rust_rifterm(instance);
    let class_q = c2rust_rifterm(class);
    let new_member = unsafe {
        match (*data).get_next_member(instance_q, class_q){
            Some(x) => x,
            None => {return ptr::null_mut();},
        }
    };
    match new_member.to_c_member() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_equal(data: *mut RIFIData, left: *const RIFITerm, right: *const RIFITerm,
    ) -> *mut RIFIEqual
{
    if data.is_null() {return ptr::null_mut();}
    let left_q = c2rust_rifterm(left);
    let right_q = c2rust_rifterm(right);
    let new_equal = unsafe {
        match (*data).get_next_equal(left_q, right_q){
            Some(x) => x,
            None => {return ptr::null_mut();},
        }
    };
    match new_equal.to_c_equal() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

impl Equal {
    pub fn from_c_equal(equal: *const RIFIEqual) -> Self {
        unsafe {
            let left = c2rust_rifterm((*equal).left);
            let right = c2rust_rifterm((*equal).right);
            Equal {
                left: left,
                right: right,
            }
        }
    }

    pub fn to_c_equal(&self) -> Result<*mut RIFIEqual, Error> {
        let ret = unsafe {
            let left = self.left.to_c_term();
            let right = self.right.to_c_term();
            RIFIEqual_new(left, right)
        };
        Ok(ret)
    }
}

impl Subclass {
    pub fn from_c_subclass(subclass: *const RIFISubclass) -> Self {
        unsafe {
            let sub = c2rust_rifterm((*subclass).sub_class);
            let super_ = c2rust_rifterm((*subclass).super_class);
            Subclass {
                sub: sub,
                super_: super_,
            }
        }
    }

    pub fn to_c_subclass(&self) -> Result<*mut RIFISubclass, Error> {
        let ret = unsafe {
            let sub = self.sub.to_c_term();
            let super_ = self.super_.to_c_term();
            RIFISubclass_new(sub, super_)
        };
        Ok(ret)
    }
}

impl Member {
    pub fn from_c_member(member: *const RIFIMember) -> Self {
        unsafe {
            let instance = c2rust_rifterm((*member).instance);
            let class = c2rust_rifterm((*member).class);
            Member {
                instance: instance,
                class: class,
            }
        }
    }

    pub fn to_c_member(&self) -> Result<*mut RIFIMember, Error> {
        let ret = unsafe {
            let class = self.class.to_c_term();
            let instance = self.instance.to_c_term();
            RIFIMember_new(instance, class)
        };
        Ok(ret)
    }
}
impl Frame {
    pub fn from_c_frame(frame: *const RIFIFrame) -> Self {
        unsafe {
            let obj = c2rust_rifterm((*frame).object);
            let key = c2rust_rifterm((*frame).slotkey);
            let value = c2rust_rifterm((*frame).slotvalue);
            Frame {
                object: obj,
                slotkey: key,
                slotvalue: value,
            }
        }
    }

    pub fn to_c_frame(&self) -> Result<*mut RIFIFrame, Error> {
        let ret = unsafe {
            let obj = self.object.to_c_term();
            let key = self.slotkey.to_c_term();
            let val = self.slotvalue.to_c_term();
            RIFIFrame_new(obj, key, val)
        };
        Ok(ret)
    }
}

impl Atom {
    pub fn from_c_atom(atom: *const RIFIAtom) -> Self {
        unsafe {
            let op = c2rust_rifterm((*atom).op);
            let args = c2rust_riftermlist((*atom).args);
            Atom {
                op: op,
                args: args,
            }
        }
    }

    pub fn to_c_atom(&self) -> Result<*mut RIFIAtom, Error> {
        let ret = unsafe {
            let a = self.op.to_c_term();
            let b = vec_to_rifitermlist(&self.args);
            RIFIAtom_new(a, b)
        };
        Ok(ret)
    }
}

fn vec_to_rifitermlist(list: &Vec<RIFTerm>) -> *mut RIFITermList {
    let mut ret = ptr::null_mut();
    for x in list {
        unsafe {
            let new = x.to_c_term();
            ret = RIFITermList_append(ret, new);
        }
    }
    ret
}


impl RIFTerm {
    pub fn to_c_term(&self) -> *mut RIFITerm {
        match self {
            RIFTerm::IRI(value) => unsafe {
                RIFITerm_new_iri(value.as_ptr())
            },
            RIFTerm::TypedLiteral(value, suffix) => unsafe {
                let c_suffix = match suffix {
                    Some(x) => x.as_ptr(),
                    None => ptr::null(),
                };
                RIFITerm_new_typedliteral(value.as_ptr(), c_suffix)
            }
            RIFTerm::LangLiteral(value, suffix) => unsafe {
                RIFITerm_new_langliteral(value.as_ptr(), suffix.as_ptr())
            },
            RIFTerm::List(list) => unsafe {
                let c_list = vec_to_rifitermlist(list);
                let ret = RIFITerm_new_list(c_list, ptr::null_mut());
                free_RIFITermList(c_list);
                ret
            },
            RIFTerm::Local(value) => unsafe {
                RIFITerm_new_local(value.as_ptr())
            },
            RIFTerm::Var => ptr::null_mut(),
            RIFTerm::Variable(value) => unsafe {
                RIFITerm_new_variable(value.as_ptr())
            }
        }
    }
}

use crate::formula_call_rdf_hook::*;

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_send_as_rdf(
    data: *mut RIFIData,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> i64 {
    unsafe {
        match (*data).send_as_rdf(hook, hook_data) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("RIFIData_send_as_rdf failed with: {}", e);
                1
            },
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_send_document_as_rdf(
    data: *mut RIFIData,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> i64 {
    unsafe {
        match (*data).send_document_as_rdf(hook, hook_data) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("RIFIData_send_document as_rdf failed with: {}", e);
                1
            },
        }
    }
}


use crate::rififormulas::RIFIFormulas;


#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_new() -> *mut RIFIFormulas {
    let mybox = Box::new(RIFIFormulas::new());
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_to_RIFIData(
    formulas: *mut RIFIFormulas,
) -> *mut RIFIData {
    if formulas.is_null(){
        return ptr::null_mut();
    }
    //consume graph
    let g: RIFIFormulas = Box::into_inner(unsafe {Box::from_raw(formulas)});
    let x = g.to_RIFIData();

    let mybox = Box::new(x);
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_add_atom(
    formulas: *mut RIFIFormulas, atom: *const RIFIAtom)
{
    if formulas.is_null(){return;}
    if atom.is_null(){return;}
    unsafe {
        let rust_atom = Atom::from_c_atom(atom);
        (*formulas).add_atom(rust_atom);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_add_frame(
    formulas: *mut RIFIFormulas, frame: *const RIFIFrame)
{
    if formulas.is_null(){return;}
    if frame.is_null(){return;}
    unsafe {
        let rust_frame = Frame::from_c_frame(frame);
        (*formulas).add_frame(rust_frame);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_add_subclass(
    formulas: *mut RIFIFormulas, subclass: *const RIFISubclass)
{
    if formulas.is_null(){return;}
    if subclass.is_null(){return;}
    unsafe {
        let rust_subclass = Subclass::from_c_subclass(subclass);
        (*formulas).add_subclass(rust_subclass);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_add_member(
    formulas: *mut RIFIFormulas, member: *const RIFIMember)
{
    if formulas.is_null(){return;}
    if member.is_null(){return;}
    unsafe {
        let rust_member = Member::from_c_member(member);
        (*formulas).add_member(rust_member);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIFormulas_add_equal(
    formulas: *mut RIFIFormulas, equal: *const RIFIEqual)
{
    if formulas.is_null(){return;}
    if equal.is_null(){return;}
    unsafe {
        let rust_equal = Equal::from_c_equal(equal);
        (*formulas).add_equal(rust_equal);
    }
}
