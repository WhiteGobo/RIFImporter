use std::ptr;
use std::ffi::{c_char, CStr, c_uchar};
use crate::shared::{RIFIData, RIFTerm, Atom, Frame, Member, Equal, Subclass};
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


unsafe extern "C" {
    //pub unsafe fn copy2cstring(input: *mut c_uchar) -> *mut c_uchar;
    pub unsafe fn RIFITerm_new_iri(value: *const c_uchar, value_length: u64) -> *mut RIFITerm;
    pub unsafe fn RIFITerm_new_typedliteral(value: *const c_uchar, value_length: u64, suffix: *const c_uchar, value_length: u64) -> *mut RIFITerm;
    pub unsafe fn RIFITerm_new_langliteral(value: *const c_uchar, value_length: u64, suffix: *const c_uchar, value_length: u64) -> *mut RIFITerm;
    pub unsafe fn RIFITermList_append(old: *mut RIFITermList, new: *mut RIFITerm) -> *mut RIFITermList;
    pub unsafe fn RIFITerm_new_local(value: *const c_uchar, value_length: u64) -> *mut RIFITerm;
    pub unsafe fn RIFIFrame_new(object: *mut RIFITerm, slotkey: *mut RIFITerm, slotvalue: *mut RIFITerm) -> *mut RIFIFrame;
    pub unsafe fn RIFIAtom_new(op: *mut RIFITerm, args: *mut RIFITermList) -> *mut RIFIAtom;
    pub unsafe fn RIFISubclass_new(sub: *mut RIFITerm, super_: *mut RIFITerm) -> *mut RIFISubclass;
    pub unsafe fn RIFIMember_new(instance: *mut RIFITerm, class: *mut RIFITerm) -> *mut RIFIMember;
    pub unsafe fn RIFIEqual_new(left: *mut RIFITerm, right: *mut RIFITerm) -> *mut RIFIEqual;
    pub unsafe fn RIFITerm_new_list(list: *const RIFITermList, rest: *const RIFITerm) -> *mut RIFITerm;
    pub unsafe fn free_RIFITermList(list: *mut RIFITermList);
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_new(entailment: *const c_char) -> *mut RIFIData
{
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
    let x = match RIFIData::new(ent){
        Some(x) => x,
        None => {return ptr::null_mut();},
    };
    let mybox = Box::new(x);
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_add(
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
        _graph_id: *const c_char, _graph_type: u8,
        data: *mut RIFIData,
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
            Err(_) => {eprintln!("Failed to translate object"); return -4;},
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

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_atom(
    data: *mut RIFIData, op: *const RIFITerm, args: *const RIFITermList,
    ) -> *mut RIFIAtom
{
    if data.is_null() {return ptr::null_mut();}
    let op_q = c2rust_rifterm(op);
    let args_q = c2rust_riftermlist(args);
    let new_atom = unsafe {
        match (*data).get_next_atom(op_q, Some(args_q)){
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

#[unsafe(no_mangle)]
pub extern "C" fn RIFIData_get_next_frame(data: *mut RIFIData, object: *const RIFITerm, slotkey: *const RIFITerm, slotvalue: *const RIFITerm) -> *mut RIFIFrame
{
    if data.is_null() {return ptr::null_mut();}
    let obj_q = c2rust_rifterm(object);
    let key_q = c2rust_rifterm(slotkey);
    let val_q = c2rust_rifterm(slotvalue);
    unsafe {
        let new_frame = unsafe {
            match (*data).get_next_frame(obj_q, key_q, val_q){
                Some(x) => x,
                None => {return ptr::null_mut();},
            }
        };
        eprintln!("return frame: {:?}", new_frame);
        match new_frame.to_c_frame() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("internal error {}", e);
                ptr::null_mut()
            },
        }
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
    eprintln!("return subclass: {:?}", new_subclass);
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
    eprintln!("return member: {:?}", new_member);
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
    eprintln!("return equal: {:?}", new_equal);
    match new_equal.to_c_equal() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("internal error {}", e);
            ptr::null_mut()
        },
    }
}

impl Equal {
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
                let vlen: u64 = match value.len().try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                RIFITerm_new_iri(value.as_ptr(), vlen)
            },
            RIFTerm::TypedLiteral(value, suffix) => unsafe {
                let vlen: u64 = match value.len().try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                let (c_suffix, suffix_len) = match suffix {
                    Some(x) => (x.as_ptr(), x.len()),
                    None => (ptr::null(), 0),
                };
                let slen: u64 = match suffix_len.try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                RIFITerm_new_typedliteral(value.as_ptr(), vlen, c_suffix, slen)
            }
            RIFTerm::LangLiteral(value, suffix) => unsafe {
                let vlen: u64 = match value.len().try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                let slen: u64 = match suffix.len().try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                RIFITerm_new_langliteral(value.as_ptr(), vlen,
                                        suffix.as_ptr(), slen)
            },
            RIFTerm::List(list) => unsafe {
                let c_list = vec_to_rifitermlist(list);
                let ret = RIFITerm_new_list(c_list, ptr::null_mut());
                free_RIFITermList(c_list);
                ret
            },
            RIFTerm::Local(value) => unsafe {
                let vlen: u64 = match value.len().try_into() {
                    Ok(x) => x,
                    Err(_) => usize::MAX.try_into().unwrap(),
                };
                RIFITerm_new_local(value.as_ptr(), vlen)
            },
            RIFTerm::Var => ptr::null_mut(),
        }
    }
}
