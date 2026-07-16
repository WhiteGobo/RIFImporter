use std::fmt;
use std::ffi::{c_char, c_void, CString, NulError, CStr};
use std::io::Error;
use crate::rifidata::{RIFIData, Formula, RIFTerm, Atom, Frame, Subclass, Member, Equal};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

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

fn new_bnode(id_generator: &AtomicUsize) -> CString {
    let id = id_generator.update(Ordering::Relaxed, Ordering::Relaxed, |x| x+1);
    //Ensure this always works
    let bnode_name = unsafe{
        CString::new(format!("bnode_{}", id)).unwrap()
    };
    bnode_name
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
const RIF_DOCUMENT: *const i8 = c"http://www.w3.org/2007/rif#Document".as_ptr();
const RIF_GROUP: *const i8 = c"http://www.w3.org/2007/rif#Group".as_ptr();

const RIF_PAYLOAD: *const i8 = c"http://www.w3.org/2007/rif#payload".as_ptr();
const RIF_SENTENCES: *const i8 = c"http://www.w3.org/2007/rif#sentences".as_ptr();
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

struct MyContext {
    atomic: AtomicUsize,
    hook: TripleHandler,
    hook_data: *mut c_void,
}

impl RIFIData {
    pub fn send_document_as_rdf(&mut self,
        hook: TripleHandler,
        hook_data: *mut c_void,
    ) -> Result<(), HookError> {
        let atomic = AtomicUsize::new(0);
        let mut i = 0;
        let mut ret: Vec<(CString, u8)> = Vec::new();
        for formula in self.into_iter() {
            let base = match CString::new(format!("b{}", i)){
                Ok(x) => x,
                Err(_) => {
                    return Err(HookError::InternalError(0));
                }
            };
            match formula.send_rdf(base, BNODE, hook, hook_data) {
                Ok(x) => {ret.push(x);},
                Err(e) => {return Err(e);},
            }
            i += 1;
        }
        let cntxt = MyContext::new(atomic, hook, hook_data);
        let (sentences, sent_type) = {
            let bid = cntxt.new_bnode();
            match cntxt.send_rifgroup(bid, BNODE, &ret){
                Err(e) => {return Err(e);},
                Ok(x) => x,
            }
        };
        {
            let bid = cntxt.new_bnode();
            match cntxt.send_rifdocument(bid, BNODE, sentences, sent_type){
                Err(e) => {return Err(e);},
                Ok(x) => x,
            };
        }
        Ok(())
    }

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
    use RIFTerm::{IRI, TypedLiteral, LangLiteral, List, Local, Var, Variable};
    match term {
        IRI(x) => send_iri(id, id_type, x, hook, hook_data),
        TypedLiteral(x, y) => send_typedliteral(id, id_type, x, y, hook, hook_data),
        LangLiteral(x, y) => send_langliteral(id, id_type, x, y, hook, hook_data),
        List(l) => send_riftermlist(id, id_type, l, hook, hook_data),
        Local(x) => send_local(id, id_type, x, hook, hook_data),
        Var => send_var(id, id_type, hook, hook_data),
        Variable(x) => send_variable(id, id_type, x, hook, hook_data),
    }
}

fn send_typedliteral(
    id: CString, id_type: u8,
    value: &CStr, suffix: &Option<CString>,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const NULL: *const c_char = ptr::null();
    let csuffix = match suffix {
        Some(x) => x.as_ptr(),
        None => NULL,
    };
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, NULL, URI, NULL, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_VALUE,
                value.as_ptr(), csuffix, TYPEDLITERAL,
                NULL, BNODE, hook_data)
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, ptr::null(), URI, ptr::null(), BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_VALUE,
                value.as_ptr(), suffix.as_ptr(), LANGLITERAL,
                NULL, BNODE, hook_data)
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_LIST, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), id_type, RIF_ITEMS, x.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_VAR, NULL, URI,
                NULL, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    return Err(HookError::other("send var not implemented"));
    match hook(id.as_ptr(), id_type, RIF_VARNAME,
                c"x".as_ptr(), NULL, TYPEDLITERAL,
                NULL, BNODE, hook_data)
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_CONST, NULL, URI, NULL, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_CONSTIRI, value.as_ptr(), XSD_ANYURI, TYPEDLITERAL, NULL, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}

fn send_variable(
    id: CString, id_type: u8,
    value: &CStr,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_VAR, NULL, URI, NULL, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(id.as_ptr(), id_type, RIF_VARNAME, value.as_ptr(), XSD_ANYURI, TYPEDLITERAL, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    let tnode_in = match extend_cstring(&id, c"_first"){
        Ok(x) => x,
        Err(e) => {return Err(HookError::NulError(e));},
    };
    let (tnode, ttype) = match send_term(tnode_in, BNODE, value, hook, hook_data)
    {
        Ok((x, y)) => (x, y),
        Err(e) => {return Err(e);},
    };
    match hook(id.as_ptr(), id_type, RDF_FIRST, tnode.as_ptr(), NULL, ttype, NULL, BNODE, hook_data)
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
    const NULL: *const c_char = ptr::null();
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
                                        current.as_ptr(), NULL, BNODE,
                                        NULL, BNODE, hook_data)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        last = current;
        last_type = BNODE;
    }
    match hook(last.as_ptr(), last_type, RDF_REST,
                RDF_NIL_CSTR.as_ptr(), NULL, URI, NULL, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    Ok((id, id_type))
}



impl MyContext {
    pub fn new(atomic: AtomicUsize,
        hook: TripleHandler,
        hook_data: *mut c_void,
    ) -> Self {
        MyContext {
            atomic: atomic,
            hook: hook,
            hook_data: hook_data,
        }
    }

    pub fn send_rifgroup(
        &self,
        id: CString, id_type: u8,
        sentences: &Vec<(CString, u8)>,
    ) -> Result<(CString, u8), HookError> {
        const NULL: *const c_char = ptr::null();
        match self.send(id.as_ptr(), id_type, RDF_TYPE,
                        RIF_GROUP, NULL, URI)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        let (listid, listtype) = {
            let tmpid = self.new_bnode();
            match self.send_rdflist(tmpid, BNODE, sentences) {
                Ok(x) => x,
                Err(e) => {return Err(e);},
            }
        };
        match self.send(id.as_ptr(), id_type, RIF_SENTENCES,
                        listid.as_ptr(), NULL, listtype)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        Ok((id, id_type))
    }

    pub fn send_rifdocument(
        &self,
        id: CString, id_type: u8,
        payload_id: CString, payload_type: u8,
    ) -> Result<(CString, u8), HookError> {
        const NULL: *const c_char = ptr::null();
        match self.send(id.as_ptr(), id_type, RDF_TYPE,
                        RIF_DOCUMENT, NULL, URI)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        match self.send(id.as_ptr(), id_type, RIF_PAYLOAD,
                        payload_id.as_ptr(), NULL, payload_type)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        Ok((id, id_type))
    }

    fn send(
        &self,
        subj: *const c_char, subj_type: u8,
        pred: *const c_char,
        obj: *const c_char, obj_suffix: *const c_char, obj_type: u8,
    ) -> i8 {
        const NULL: *const c_char = ptr::null();
        (self.hook)(
            subj, subj_type,
            pred,
            obj, obj_suffix, obj_type,
            NULL, BNODE, self.hook_data)
    }

    fn send_rdflist(
        &self,
        id: CString, id_type: u8,
        list: &Vec<(CString, u8)>,
    ) -> Result<(CString, u8), HookError> {
        const NULL: *const c_char = ptr::null();
        let mut iter = list.into_iter();
        let (first_v, first_t) = match iter.next() {
            Some(x) => x,
            None => {return Ok((RDF_NIL_CSTR.to_owned(), URI));},
        };
        match self.send(id.as_ptr(), id_type, RDF_FIRST,
                        first_v.as_ptr(), NULL, *first_t)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        let mut last = id.clone();
        let mut last_type = id_type;
        
        for (xv, xt) in iter {
            let current = new_bnode(&self.atomic);
            match self.send(current.as_ptr(), BNODE, RDF_FIRST, xv.as_ptr(), NULL, *xt)
            {
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
            match self.send(last.as_ptr(), last_type, RDF_REST, current.as_ptr(), NULL, BNODE)
            {
                0 => {},
                x => {return Err(HookError::retval(x));},
            }
            last = current;
            last_type = BNODE;
        }
        match self.send(last.as_ptr(), last_type, RDF_REST, RDF_NIL_CSTR.as_ptr(), NULL, URI)
        {
            0 => {},
            x => {return Err(HookError::retval(x));},
        }
        Ok((id, id_type))
    }

    pub fn new_bnode(&self) -> CString {
        new_bnode(&self.atomic)
    }
}


fn send_atom(
    atom: &Atom,
    id: CString, id_type: u8,
    hook: TripleHandler,
    hook_data: *mut c_void,
) -> Result<(CString, u8), HookError> {
    let NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_ATOM, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_OP, op.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_ARGS, args.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_FRAME, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_OBJECT, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    match hook(id.as_ptr(), id_type, RIF_SLOTS, slots.as_ptr(), NULL, slots_type, NULL, BNODE, hook_data){
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(slot.as_ptr(), slots_type, RDF_FIRST, slot.as_ptr(), NULL, slottype, NULL, BNODE, hook_data)
    {
        0 => {},
        x => {return Err(HookError::retval(x));},
    }
    match hook(slot.as_ptr(), slots_type, RDF_REST, RDF_NIL_CSTR.as_ptr(), NULL, URI, NULL, BNODE, hook_data)
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
    const NULL: *const c_char = ptr::null();
    let val_base = match extend_cstring(&id, c"_val"){
        Ok(x) => x,
        Err(_) => {return Err(HookError::internal(5));},
    };
    match send_term(val_base, BNODE, val, hook, hook_data) {
        Err(e) => {return Err(e);},
        Ok((x, t)) => {
            match hook(id.as_ptr(), id_type, RIF_SLOTVALUE, x.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), id_type, RIF_SLOTKEY, x.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_SUBCLASS, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_SUB, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_SUPER, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_MEMBER, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_INSTANCE, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_CLASS, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
    const NULL: *const c_char = ptr::null();
    match hook(id.as_ptr(), id_type, RDF_TYPE, RIF_EQUAL, NULL, URI, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_LEFT, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
            match hook(id.as_ptr(), BNODE, RIF_RIGHT, obj.as_ptr(), NULL, t, NULL, BNODE, hook_data){
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
