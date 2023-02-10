/////////////////////////////////////////////////////////////////////////////////
//                                Hash trait                                   //
/////////////////////////////////////////////////////////////////////////////////
pub trait Hash {
    fn hash(&self) -> usize;
}

impl Hash for String {
    fn hash(&self) -> usize {
        let bits = self.as_bytes();
        let length: usize = self.len();
        let mut hash_value: usize = 0;
        for x in 0..length {
            hash_value = hash_value.overflowing_add(usize::from(bits[x])).0;
            hash_value = hash_value.overflowing_mul(17).0;
        }
        hash_value
    }
}

impl Hash for isize {
    fn hash(&self) -> usize {
        if self.is_negative() {
            return usize::MAX - self.abs() as usize;
        } else {
            return *self as usize;
        }
    }
}

impl Hash for usize {
    fn hash(&self) -> usize {
        *self
    }
}
/////////////////////////////////////////////////////////////////////////////////
//                                ToString trait                               //
/////////////////////////////////////////////////////////////////////////////////

pub trait ToString {
    fn to_string_(&self) -> String;
}

impl ToString for String {
    fn to_string_(&self) -> String {
        self.to_owned()
    }
}

impl ToString for isize {
    fn to_string_(&self) -> String {
        self.to_string()
    }
}
impl ToString for usize {
    fn to_string_(&self) -> String {
        self.to_string()
    }
}
