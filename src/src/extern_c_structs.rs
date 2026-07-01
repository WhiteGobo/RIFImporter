use std::ffi::{c_char, CStr};
use crate::rifidata::{RIFTerm};

#[repr(C)]
pub struct RIFITerm {
    pub term_type: u8,
    pub value: *mut c_char,
    pub suffix: *mut c_char,
}

#[repr(C)]
pub struct RIFITermList {
    pub first: *mut RIFITerm,
    pub rest: *mut RIFITermList,
}

#[repr(C)]
pub struct RIFIEqual {
    pub left: *mut RIFITerm,
    pub right: *mut RIFITerm,
}

#[repr(C)]
pub struct RIFIMember {
    pub instance: *mut RIFITerm,
    pub class: *mut RIFITerm,
}

#[repr(C)]
pub struct RIFISubclass {
    pub sub_class: *mut RIFITerm,
    pub super_class: *mut RIFITerm,
}

#[repr(C)]
pub struct RIFIFrame {
    pub object: *mut RIFITerm,
    pub slotkey: *mut RIFITerm,
    pub slotvalue: *mut RIFITerm,
}

#[repr(C)]
pub struct RIFIAtom {
    pub op: *mut RIFITerm,
    pub args: *mut RIFITermList,
}

const RIF_TERM_TYPE_IRI: u8 = 0;
const RIF_TERM_TYPE_TYPEDLITERAL: u8 = 1;
const RIF_TERM_TYPE_LANGLITERAL: u8 = 2;
const RIF_TERM_TYPE_LIST: u8 = 3;
const RIF_TERM_TYPE_LOCAL: u8 = 4;


fn c2rust_convert_iri(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        let value = CStr::from_ptr((*cterm).value).to_owned();
        Some(RIFTerm::IRI(value))
    }
}

fn c2rust_convert_typedliteral(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        let value = CStr::from_ptr((*cterm).value).to_owned();
        let suffix = if (*cterm).suffix.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*cterm).suffix).to_owned())
        };
        Some(RIFTerm::TypedLiteral(value, suffix))
    }
}

fn c2rust_convert_langliteral(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        if (*cterm).suffix.is_null() {return None;}
        let value = CStr::from_ptr((*cterm).value).to_owned();
        let suffix = CStr::from_ptr((*cterm).suffix).to_owned();
        Some(RIFTerm::LangLiteral(value, suffix))
    }
}

fn c2rust_convert_list(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        let c_list: *const RIFITermList = (*cterm).value.cast();
        let c_rest: *const RIFITerm = (*cterm).suffix.cast();

        if c_list.is_null() {return None;}
        let list = c2rust_riftermlist(c_list);
        let _rest = if c_rest.is_null() {
            None
        } else {
            Some(c2rust_rifterm(c_rest))
        };
        Some(RIFTerm::List(list))
    }
}

pub fn c2rust_rifterm(cterm: *const RIFITerm) -> RIFTerm {
    if cterm.is_null() {
        return RIFTerm::Var;
    }
    let ret = unsafe{
        match (*cterm).term_type {
            RIF_TERM_TYPE_IRI => c2rust_convert_iri(cterm),
            RIF_TERM_TYPE_TYPEDLITERAL => c2rust_convert_typedliteral(cterm),
            RIF_TERM_TYPE_LANGLITERAL => c2rust_convert_langliteral(cterm),
            RIF_TERM_TYPE_LIST => c2rust_convert_list(cterm),
            RIF_TERM_TYPE_LOCAL => Some(RIFTerm::Var),
            _ => None,
        }
    };
    match ret {
        Some(x) => x,
        None => RIFTerm::Var,
    }
}

pub fn c2rust_riftermlist(cterm: *const RIFITermList) -> Vec<RIFTerm> {
    let mut ret = Vec::new();
    let mut tmp = cterm;
    while !tmp.is_null() {unsafe {
        ret.push(c2rust_rifterm((*tmp).first));
        tmp = (*tmp).rest;
    }}
    return ret;
}
