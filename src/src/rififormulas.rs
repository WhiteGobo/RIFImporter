use crate::rifidata::{
    RIFIData, Atom, Frame, Subclass, Member, Equal, RIFTerm,
    list_equal_or_valid_for_rifterm,
};

pub struct RIFIFormulas {
    atoms: Vec<Atom>,
    frames: Vec<Frame>,
    subclasses: Vec<Subclass>,
    members: Vec<Member>,
    equals: Vec<Equal>,
}

impl RIFIFormulas {
    pub fn new() -> Self {
        RIFIFormulas {
            atoms: Vec::new(),
            frames: Vec::new(),
            subclasses: Vec::new(),
            members: Vec::new(),
            equals: Vec::new(),
        }
    }

    pub fn to_RIFIData(self) -> RIFIData {
        RIFIData::Formulas(self)
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }

    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn add_subclass(&mut self, subclass: Subclass) {
        self.subclasses.push(subclass);
    }
    
    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn add_equal(&mut self, equal: Equal) {
        self.equals.push(equal);
    }


    pub fn get_next_atom(&mut self, op: RIFTerm, args: Option<Vec<RIFTerm>>,
        ) -> Option<Atom>
    {
        for (pos, x) in self.atoms.iter().enumerate() {
            if !op.equal_or_valid_for(&x.op) {
                continue;
            }
            match args {
                None => {},
                Some(ref q_args) => {
                    if !list_equal_or_valid_for_rifterm(&q_args, &x.args) {
                        continue;
                    }
                }
            }
            return Some(self.atoms.remove(pos));
        }
        None
    }

    pub fn get_next_frame(&mut self, object: RIFTerm, slotkey: RIFTerm, slotvalue: RIFTerm,
        ) -> Option<Frame>
    {
        for (pos, x) in self.frames.iter().enumerate() {
            if !object.equal_or_valid_for(&x.object) {
                continue;
            }
            if !slotkey.equal_or_valid_for(&x.slotkey) {
                continue;
            }
            if !slotvalue.equal_or_valid_for(&x.slotvalue) {
                continue;
            }
            return Some(self.frames.remove(pos));
        }
        None
    }

    pub fn get_next_subclass(&mut self, sub: RIFTerm, super_: RIFTerm,
        ) -> Option<Subclass>
    {
        for (pos, x) in self.subclasses.iter().enumerate() {
            if !sub.equal_or_valid_for(&x.sub) {
                continue;
            }
            if !super_.equal_or_valid_for(&x.super_) {
                continue;
            }
            return Some(self.subclasses.remove(pos));
        }
        None
    }

    pub fn get_next_member(&mut self, instance: RIFTerm, class: RIFTerm,
        ) -> Option<Member>
    {
        for (pos, x) in self.members.iter().enumerate() {
            if !instance.equal_or_valid_for(&x.instance) {
                continue;
            }
            if !class.equal_or_valid_for(&x.class) {
                continue;
            }
            return Some(self.members.remove(pos));
        }
        None
    }

    pub fn get_next_equal(&mut self, left: RIFTerm, right: RIFTerm,
        ) -> Option<Equal>
    {
        for (pos, x) in self.equals.iter().enumerate() {
            if left.equal_or_valid_for(&x.left) 
                && right.equal_or_valid_for(&x.right)
            {
                return Some(self.equals.remove(pos));
            }
            if left.equal_or_valid_for(&x.right) 
                && right.equal_or_valid_for(&x.left)
            {
                return Some(self.equals.remove(pos));
            }
        }
        None
    }

}
