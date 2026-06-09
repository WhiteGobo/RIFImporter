use std::ptr;
use std::ffi::{c_char, CStr, c_uchar};
use crate::shared::{RIFIData, RIFTerm};

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
pub struct RIFIEqual;

#[repr(C)]
pub struct RIFIMember;

#[repr(C)]
pub struct RIFISubclass;

#[repr(C)]
pub struct RIFIFrame;

#[repr(C)]
pub struct RIFIAtom;

const RIF_TERM_TYPE_IRI: u8 = 0;
const RIF_TERM_TYPE_TYPEDLITERAL: u8 = 1;
const RIF_TERM_TYPE_LANGLITERAL: u8 = 2;
const RIF_TERM_TYPE_LIST: u8 = 3;
const RIF_TERM_TYPE_LOCAL: u8 = 4;


fn c2rust_convert_iri(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        let value = match CStr::from_ptr((*cterm).value).to_str() {
            Ok(x) => x,
            Err(_e) => {return None;},
        };
        Some(RIFTerm::IRI(value.to_owned()))
    }
}

fn c2rust_convert_typedliteral(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        let value = match CStr::from_ptr((*cterm).value).to_str() {
            Ok(x) => x.to_owned(),
            Err(_e) => {return None;},
        };
        let suffix = if (*cterm).suffix.is_null() {
            None
        } else {
            match CStr::from_ptr((*cterm).suffix).to_str() {
                Ok(x) => Some(x.to_owned()),
                Err(_e) => {return None;},
            }
        };
        Some(RIFTerm::TypedLiteral(value, suffix))
    }
}

fn c2rust_convert_langliteral(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        if (*cterm).value.is_null() {return None;}
        if (*cterm).suffix.is_null() {return None;}
        let value = match CStr::from_ptr((*cterm).value).to_str() {
            Ok(x) => x.to_owned(),
            Err(_e) => {return None;},
        };
        let suffix = match CStr::from_ptr((*cterm).suffix).to_str() {
            Ok(x) => x.to_owned(),
            Err(_e) => {return None;},
        };
        Some(RIFTerm::LangLiteral(value, suffix))
    }
}

fn c2rust_convert_list(cterm: *const RIFITerm) -> Option<RIFTerm>{
    unsafe {
        let c_list: *const RIFITermList = (*cterm).value.cast();
        let c_rest: *const RIFITerm = (*cterm).suffix.cast();

        if c_list.is_null() {return None;}
        let list = c2rust_riftermlist(c_list);
        let rest = if c_rest.is_null() {
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
