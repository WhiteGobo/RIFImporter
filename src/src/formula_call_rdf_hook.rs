use std::fmt;
use std::ffi::{c_char, c_void, CString, NulError, CStr};
use std::io::Error;
use crate::shared::{RIFIData, Formula, RIFTerm, Atom, Frame, Subclass, Member, Equal};
use std::ptr;

pub enum HookError {
    Error(Error),
    HookReturn(i8),
    NulError(NulError),
    InternalError(u8),
}

impl HookError {
    pub fn other(value: &str) -> Self {
        HookError::Error(Error::other(value))
    }
    pub fn internal(value: u8) -> Self {
        HookError::InternalError(value)
    }
    pub fn retval(value: i8) -> Self {
        HookError::HookReturn(value)
    }
}

impl From<NulError> for HookError {
    fn from(e: NulError) -> Self {
        HookError::NulError(e)
    }
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookError::Error(e) => e.fmt(f),
            HookError::NulError(e) => e.fmt(f),
            HookError::InternalError(e) => {
                write!(f, "internal error({})", e)
            },
            HookError::HookReturn(e) => {
                write!(f, "triplehandler failed(return value: {})", e)
            }
        }
    }
}

pub type TripleHandler = extern "C" fn(
    *const c_char, u8,
    *const c_char,
    *const c_char, *const c_char, u8,
    *const c_char, u8,
    *mut c_void) -> i8;

fn new_bnode(id1: usize, id2: usize) -> String {
    format!("bnode_{}_{}", id1, id2).to_owned()
}

use oxrdf::vocab::rdf;
use crate::vocab::rif;

const URI: u8 = 0;
const BNODE: u8 = 1;
const TYPEDLITERAL: u8 = 2;
const LANGLITERAL: u8 = 3;
const RDF_TYPE: *const i8 = c"http://www.w3.org/1999/02/22-rdf-syntax-ns#type".as_ptr();
const RDF_REST: *const i8 = c"http://www.w3.org/1999/02/22-rdf-syntax-ns#rest".as_ptr();
const RDF_FIRST: *const i8 = c"http://www.w3.org/1999/02/22-rdf-syntax-ns#first".as_ptr();
const RDF_NIL_CSTR: &CStr = c"http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
const RIF_CONST: *const i8 = c"http://www.w3.org/2007/rif#Const".as_ptr();
const RIF_LIST: *const i8 = c"http://www.w3.org/2007/rif#List".as_ptr();
const RIF_ATOM: *const i8 = c"http://www.w3.org/2007/rif#Atom".as_ptr();
const RIF_FRAME: *const i8 = c"http://www.w3.org/2007/rif#Frame".as_ptr();
const RIF_SUBCLASS: *const i8 = c"http://www.w3.org/2007/rif#Subclass".as_ptr();
const RIF_MEMBER: *const i8 = c"http://www.w3.org/2007/rif#Member".as_ptr();
const RIF_EQUAL: *const i8 = c"http://www.w3.org/2007/rif#Equal".as_ptr();
const RIF_VAR: *const i8 = c"http://www.w3.org/2007/rif#Var".as_ptr();

const RIF_OP: *const i8 = c"http://www.w3.org/2007/rif#op".as_ptr();
const RIF_ARGS: *const i8 = c"http://www.w3.org/2007/rif#args".as_ptr();
const RIF_SLOTS: *const i8 = c"http://www.w3.org/2007/rif#slots".as_ptr();
const RIF_OBJECT: *const i8 = c"http://www.w3.org/2007/rif#object".as_ptr();
const RIF_SLOTKEY: *const i8 = c"http://www.w3.org/2007/rif#slotkey".as_ptr();
const RIF_SLOTVALUE: *const i8 = c"http://www.w3.org/2007/rif#slotvalue".as_ptr();
const RIF_LEFT: *const i8 = c"http://www.w3.org/2007/rif#left".as_ptr();
const RIF_RIGHT: *const i8 = c"http://www.w3.org/2007/rif#right".as_ptr();
const RIF_INSTANCE: *const i8 = c"http://www.w3.org/2007/rif#instance".as_ptr();
const RIF_CLASS: *const i8 = c"http://www.w3.org/2007/rif#class".as_ptr();
const RIF_SUB: *const i8 = c"http://www.w3.org/2007/rif#sub".as_ptr();
const RIF_SUPER: *const i8 = c"http://www.w3.org/2007/rif#super".as_ptr();
const RIF_CONSTIRI: *const i8 = c"http://www.w3.org/2007/rif#constIRI".as_ptr();
const RIF_VALUE: *const i8 = c"http://www.w3.org/2007/rif#value".as_ptr();
const RIF_ITEMS: *const i8 = c"http://www.w3.org/2007/rif#items".as_ptr();
const RIF_VARNAME: *const i8 = c"http://www.w3.org/2007/rif#varname".as_ptr();

const XSD_ANYURI: *const i8 = c"http://www.w3.org/2001/XMLSchema#anyURI".as_ptr();

impl RIFIData {
    pub fn send_as_rdf(&mut self,
        hook: TripleHandler,
        hook_data: *mut c_void,
    ) -> Result<(), HookError> {
        let mut i = 0;
        for formula in self.into_iter() {
            let base = match CString::new(format!("b{}", i)){
                Ok(x) => x,
                Err(_) => {
                    return Err(HookError::InternalError(0));
                }
            };
            match formula.send_rdf(base, BNODE, hook, hook_data) {
                Ok(_) => {},
                Err(e) => {return Err(e);},
            }
            i += 1;
        }
        Ok(())
    }
}

impl Formula {
    pub fn send_rdf(&self,
        id: CString, id_type: u8,
        hook: TripleHandler,
        hook_data: *mut c_void,
    ) -> Result<(CString, u8), HookError> {
        match self {
            Formula::Atom(a) => send_atom(a, id, id_type, hook, hook_data),
            Formula::Frame(f) => send_frame(f, id, id_type, hook, hook_data),
            Formula::Subclass(s)
                => send_subclass(s, id, id_type, hook, hook_data),
            Formula::Member(m) => send_member(m, id, id_type, hook, hook_data),
            Formula::Equal(e) => send_equal(e, id, id_type, hook, hook_data),
        }
    }
}

fn send_term(
    id: CString, id_type: u8,
    term: &RIFTerm,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    use RIFTerm::{IRI, TypedLiteral, LangLiteral, List, Local, Var};
    match term {
        IRI(x) => send_iri(id, id_type, x, hook, hook_data),
        TypedLiteral(x, y) => send_typedliteral(id, id_type, x, y, hook, hook_data),
        LangLiteral(x, y) => send_langliteral(id, id_type, x, y, hook, hook_data),
        List(l) => send_riftermlist(id, id_type, l, hook, hook_data),
        Local(x) => send_local(id, id_type, x, hook, hook_data),
        Var => send_var(id, id_type, hook, hook_data),
    }
}

fn send_typedliteral(
    id: CString, id_type: u8,
    value: &CStr, suffix: &Option<CString>,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    let csuffix = match suffix {
        Some(x) => x.as_ptr(),
        None => null,
    };
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_VALUE,
                value.as_ptr(), csuffix, TYPEDLITERAL,
                null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_langliteral(
    id: CString, id_type: u8,
    value: &CStr, suffix: &CStr,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, ptr::null(), URI, ptr::null(), BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_VALUE,
                value.as_ptr(), suffix.as_ptr(), LANGLITERAL,
                null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_riftermlist(
    id: CString, id_type: u8,
    term: &Vec<RIFTerm>,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_LIST, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    let list_base = match extend_cstring(&id, c"_list"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_rdftermlist(list_base, BNODE, term, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((x, t)) => {
            match hook(id.as_ptr(), id_type, RIF_ITEMS, x.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}

fn send_local(
    id: CString, id_type: u8,
    value: &CStr,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, ptr::null(), URI, ptr::null(), BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_var(
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_VAR, null, URI,
                null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    return Err(HookError::other("send var not implemented"));
    match hook(id.as_ptr(), id_type, RIF_VARNAME,
                c"x".as_ptr(), null, TYPEDLITERAL,
                null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_iri(
    id: CString, id_type: u8,
    value: &CStr,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_CONSTIRI, value.as_ptr(), XSD_ANYURI, TYPEDLITERAL, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_rdftermlist_item(
    id: &CString, id_type: u8,
    value: &RIFTerm,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(), HookError> {
    const null: *const c_char = ptr::null();
    let tnode_in = match extend_cstring(&id, c"_first"){
        Ok(x) => x,
        Err(e) => {return Err(HookError::NulError(e));},
    };
    let (tnode, ttype) = match send_term(tnode_in, BNODE, value, hook, hook_data)
    {
        Ok((x, y)) => (x, y),
        Err(e) => {return Err(e);},
    };
    match hook(id.as_ptr(), id_type, RDF_FIRST, tnode.as_ptr(), null, ttype, null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok(())
}

fn send_rdftermlist(
    id: CString, id_type: u8,
    list: &Vec<RIFTerm>,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    let mut iter = list.into_iter();
    let x_first = match iter.next() {
        Some(x) => x,
        None => {return Ok((RDF_NIL_CSTR.to_owned(), URI));},
    };
    let mut i = 0;
    match send_rdftermlist_item(&id, id_type, x_first, hook, hook_data) {
        Ok(()) => {},
        Err(e) => {return Err(e);},
    };
    let mut last = id.clone();
    let mut last_type = id_type;
    
    for x in iter {
        let current = match extend_cstring_listid(&id, i){
            Ok(x) => x,
            Err(e) => {return Err(HookError::NulError(e));},
        };
        i += 1;
        match send_rdftermlist_item(&current, BNODE, x, hook, hook_data) {
            Ok(()) => {},
            Err(e) => {return Err(e);},
        };
        match hook(last.as_ptr(), last_type, RDF_REST,
                                        current.as_ptr(), null, BNODE,
                                        null, BNODE, hook_data)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        last = current;
        last_type = BNODE;
    }
    match hook(last.as_ptr(), last_type, RDF_REST,
                RDF_NIL_CSTR.as_ptr(), null, URI, null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_atom(
    atom: &Atom,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    let null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_ATOM, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }

    let op_base = match extend_cstring(&id, c"_op"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(op_base, BNODE, &atom.op, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((op, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_OP, op.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    if atom.args.len() == 0 {
        return Ok((id, id_type));
    }
    let args_base = match extend_cstring(&id, c"_args") {
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(1));},
    };
    match send_rdftermlist(args_base, BNODE, &atom.args, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((args, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_ARGS, args.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}

fn send_frame(
    frame: &Frame,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_FRAME, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    let obj_base = match extend_cstring(&id, c"_obj"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(obj_base, BNODE, &frame.object, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_OBJECT, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    let slot_base = match extend_cstring(&id, c"_slot"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    let (slot, slottype) = match send_slot(&frame.slotkey, &frame.slotvalue, slot_base, BNODE, hook, hook_data) {
        Ok(x) => x, 
        Err(e) => {return Err(e);},
    };
    let (slots, slots_type) = match extend_cstring(&id, c"_slot"){
        Ok(x) => (x, BNODE),
        Err(_) => {return Err(HookError::internal(0));},
    };
    match hook(id.as_ptr(), id_type, RIF_SLOTS, slots.as_ptr(), null, slots_type, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(slot.as_ptr(), slots_type, RDF_FIRST, slot.as_ptr(), null, slottype, null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(slot.as_ptr(), slots_type, RDF_REST, RDF_NIL_CSTR.as_ptr(), null, URI, null, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_slot(
    key: &RIFTerm, val: &RIFTerm,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    let val_base = match extend_cstring(&id, c"_val"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(5));},
    };
    match send_term(val_base, BNODE, val, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((x, t)) => {
            match hook(id.as_ptr(), id_type, RIF_SLOTVALUE, x.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    let key_base = match extend_cstring(&id, c"_key"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(6));},
    };
    match send_term(key_base, BNODE, key, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((x, t)) => {
            match hook(id.as_ptr(), id_type, RIF_SLOTKEY, x.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}


fn send_subclass(
    subclass: &Subclass,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_SUBCLASS, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    let sub_base = match extend_cstring(&id, c"_sub"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(sub_base, BNODE, &subclass.sub, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_SUB, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    let super_base = match extend_cstring(&id, c"_super"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(super_base, BNODE, &subclass.super_, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_SUPER, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}

fn send_member(
    member: &Member,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_MEMBER, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    let inst_base = match extend_cstring(&id, c"_inst"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(inst_base, BNODE, &member.instance, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_INSTANCE, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    let class_base = match extend_cstring(&id, c"_class"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(class_base, BNODE, &member.class, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_CLASS, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}

fn send_equal(
    equal: &Equal,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const null: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_EQUAL, null, URI, null, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    let left_base = match extend_cstring(&id, c"_left"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(left_base, BNODE, &equal.left, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_LEFT, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    let right_base = match extend_cstring(&id, c"_right"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(0));},
    };
    match send_term(right_base, BNODE, &equal.right, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((obj, t)) => {
            match hook(id.as_ptr(), BNODE, RIF_RIGHT, obj.as_ptr(), null, t, null, BNODE, hook_data){
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
        },
    }
    Ok((id, id_type))
}

fn extend_cstring_listid(first: &CStr, i: i64) -> Result<CString, NulError> {
    let suffix: CString = match CString::new(format!("_{}", i)){
        Ok(x) => x,
        e => {return e;},
    };
    extend_cstring(first, &suffix)
}

fn extend_cstring(first: &CStr, second: &CStr) -> Result<CString, NulError> {
    let result_len = first.count_bytes() + second.count_bytes();
    let mut new: Vec<u8> = Vec::with_capacity(result_len);
    new.extend_from_slice(first.to_bytes());
    new.extend_from_slice(second.to_bytes());
    CString::new(new)
}
