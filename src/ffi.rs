use std::{ffi::{c_char, CString, CStr}, mem};

#[repr(C)]
enum Gender {
    Male,
    Female,
    Nonbinary, 
    Unknown
}

impl From<&crate::types::Gender> for Gender {
    fn from(value: &crate::types::Gender) -> Self {
        match value {
            crate::types::Gender::Male => Gender::Male,
            crate::types::Gender::Female => Gender::Female,
            crate::types::Gender::Unknown => Gender::Unknown,
            crate::types::Gender::Nonbinary => Gender::Nonbinary,
        }
    }
}




/// C compatible representation of Optional<String>
#[repr(C)]
pub struct MaybeString {
    /// C string
    pub data: *mut c_char,
    /// Length of string
    pub len: usize,
}


impl MaybeString {
    fn new(opt : &Option<String>) -> Self {
        if let Some(maybe_string_val) = opt {
            MaybeString {
                data : CString::new(maybe_string_val.clone()).expect("Error").into_raw(),
                len : maybe_string_val.len()
            }
        } else {
            MaybeString {
                data : std::ptr::null_mut(),
                len : 0
            }
        }
    }
}

impl Drop for MaybeString {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe { 
                drop(CString::from_raw(self.data));
            }
        }
    }
}



#[repr(C)]
pub struct VecString {
    pub data : *mut *mut c_char,
    pub len : usize
}


impl VecString {
    pub fn new(vec : &Vec<String>) ->Self{
        let mut cstring_vec : Vec<*mut c_char> = vec.iter().map(|s| CString::new(s.clone()).expect("Error").into_raw()).collect();
        cstring_vec.shrink_to_fit();
        let retval = VecString {
            data : cstring_vec.as_mut_ptr(),
            len : cstring_vec.len()

        };
        std::mem::forget(cstring_vec);
        retval
    }
}


impl Drop for VecString {
    fn drop(&mut self) {

        let cstring_vec = unsafe  {
            Vec::from_raw_parts(self.data, self.len, self.len)
        };

        for cstring in &cstring_vec {
            unsafe {
                drop(CString::from_raw(*cstring))
            }
        }
        drop(cstring_vec);
    }
}


/// C compatible representation of gedcom::types::Name
#[repr(C)]
pub struct Name {
    /// Value
    pub value: MaybeString,
    /// Given name
    pub given: MaybeString,
    /// Surname
    pub surname: MaybeString,
    /// Prefix
    pub prefix: MaybeString,
    /// Surname prefix
    pub surname_prefix: MaybeString,
    /// Sufix
    pub suffix: MaybeString,
}

/// C compatible representation of Optional<Name>
#[repr(C)]
pub struct MaybeName {
    /// Name pointer
    pub data: *mut Name,
}

impl MaybeName {
    fn new(opt : &Option<crate::types::Name>) -> Self {

        if let Some(name_val) = opt {
            let name_box = Box::new(Name {
                    value : MaybeString::new(&name_val.value),
                    given : MaybeString::new(&name_val.given),
                    surname : MaybeString::new(&name_val.surname),
                    prefix : MaybeString::new(&name_val.prefix),
                    surname_prefix : MaybeString::new(&name_val.surname_prefix),
                    suffix : MaybeString::new(&name_val.suffix),
                });
            MaybeName {
                data : Box::<Name>::into_raw(name_box)
            }
        } else {
            MaybeName {
                data : std::ptr::null_mut()
            }
        }
    }
}

impl Drop for MaybeName {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                drop(Box::<Name>::from_raw(self.data));
            }
        }
    }
}



/// C compatible representation of Individual
#[repr(C)]
pub struct Individual {
    /// Xref
    pub xref: MaybeString,
    /// Name
    pub name: MaybeName,
    /// Gender 
    pub sex : Gender,
}

impl Individual{
    fn new(ind : &crate::types::Individual) -> Self {
        Individual {
            xref : MaybeString::new(&ind.xref),
            name : MaybeName::new(&ind.name),
            sex : Gender::from(&ind.sex)
        }
    }
}


/// C compatible representation of Vec<Individual>
#[repr(C)]
pub struct VecIndividual {
    /// Data pointer
    pub data: *mut Individual,
    /// Length of vector
    pub len: usize,
}

impl VecIndividual {
    fn new(vec : &Vec<crate::types::Individual>) -> Self {
        let mut internal_individuals : Vec<Individual> = vec.iter().map(|gc_ind| Individual::new(&gc_ind)).collect();
        internal_individuals.shrink_to_fit();
        let c_vec =  VecIndividual {
            data : internal_individuals.as_mut_ptr(),
            len : vec.len()
        };

        mem::forget(internal_individuals);
        return c_vec;
    }
}

impl Drop for VecIndividual {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.data, self.len, self.len))
        }
    }
}


#[repr(C)]
pub struct Family {
    pub xref: MaybeString,
    pub individual1: MaybeString, // mapped from HUSB
    pub individual2: MaybeString, // mapped from WIFE
    pub children: VecString,
}


impl Family {
    pub fn new(fam : &crate::types::Family) -> Self{
        Family {
            xref : MaybeString::new(&fam.xref),
            individual1 : MaybeString::new(&fam.individual1),
            individual2 : MaybeString::new(&fam.individual2),
            children : VecString::new(&fam.children)
        }
    }
}




#[repr(C)]
pub struct VecFamily {
    pub data : *mut Family,
    pub len : usize
}

impl VecFamily {
    pub fn new(vec : &Vec<crate::types::Family>) -> Self {
        let mut internal_families : Vec<Family> = vec.iter().map(|gc_ind| Family::new(gc_ind)).collect();
        internal_families.shrink_to_fit();
        let c_vec =  VecFamily {
            data : internal_families.as_mut_ptr(),
            len : vec.len()
        };

        mem::forget(internal_families);
        return c_vec;
    }
}

impl Drop for VecFamily {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.data, self.len, self.len))
        }
    }
}





/// C compatible representation of GedcomData
#[repr(C)]
pub struct GedcomData {
    /// Individuals
    pub individuals: VecIndividual,
    /// Families
    pub families: VecFamily,
}

impl GedcomData {
    fn new(data : crate::GedcomData) -> Self {
        GedcomData {
            individuals : VecIndividual::new(&data.individuals),
            families : VecFamily::new(&data.families)
            
        }
    }
}


/// Parse string and return C compatible gedcom data
/// Calle is responsible for freeing data, calling `free_parse`
#[unsafe(no_mangle)]
pub extern fn parse(content_raw: *const c_char) -> *mut GedcomData {
    let content_cstr = unsafe {
        CStr::from_ptr(content_raw)
    };
    let content = content_cstr.to_string_lossy();
    let data = crate::parse(content.chars());
    let c_data = GedcomData::new(data);
    Box::<GedcomData>::into_raw(Box::new(c_data))
}


/// Free GedcomData, returned from `parse`
#[unsafe(no_mangle)]
pub extern fn free_parse(gdata : *mut GedcomData){
    unsafe {
        drop(Box::from_raw(gdata))
    }
}



