use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::sync::Mutex;

pub type Inception = DateTime<Utc>;
pub type Universe = BTreeMap<String, Object>;

pub static UNIVERSE: Mutex<Universe> = Mutex::new(BTreeMap::new());

pub trait Fundamental {
    fn composed(&self) -> bool;
    fn flatten(&self) -> impl Fundamental;
}

#[derive(Clone, Debug)]
pub struct PureObject {
    name: String,
    data: Data,
    inception: Inception,
}

#[derive(Clone, Debug)]
pub struct Composition {
    /// The inception date of `method` matters here!
    method: PureObject,
    arguments: Vec<PureObject>,
}

#[derive(Clone, Debug)]
pub enum Data {
    Reference(String),
    Bytes(Vec<u8>),
    Integer(i64),
}

#[derive(Clone, Debug)]
pub struct Object {
    initial: PureObject,
    composition_stack: Vec<Composition>,
    /// `transients` holds prior states of a flattened or overwritten object.
    /// It should solely contain states that other objects referenced before.
    /// It may also be used to cache previous evaluations to speed things up.
    /// If a transient is no longer needed, it should be deleted, either when
    /// collecting garbage or the next time the object is actually flattened.
    transients: Vec<PureObject>,
}

impl Object {
    pub fn promote(pure_object: &PureObject) -> Self {
        Self {
            initial: pure_object.clone(),
            composition_stack: vec![],
            // Clearing `transients` here is definitely wrong.
            // TODO don't clear transients when promoting a PureObject!!!
            transients: vec![],
        }
    }

    fn find_transient_or_cached(&self, at_time: Inception) -> Option<PureObject> {
        for transient in &self.transients {
            if transient.inception == at_time {
                return Some(transient.clone());
            }
        }
        None
    }

    pub fn evaluate(&self, up_to: Inception) -> PureObject {
        if !self.composed() {
            self.initial.clone()
        } else {
            match self.find_transient_or_cached(up_to) {
                Some(res) => res,
                None => {
                    let mut data = self.initial.clone();
                    for composition in &self.composition_stack {
                        if composition.method.inception > up_to {
                            break;
                        }
                        data = data.apply(composition);
                    }
                    data
                }
            }
        }
    }

    pub fn compose(&mut self, composition: Composition) {
        self.composition_stack.push(composition);
    }
}

impl PureObject {
    pub fn new(name: String, data: Data) -> Self {
        Self {
            name,
            data,
            inception: Utc::now(),
        }
    }

    pub fn apply(&self, composition: &Composition) -> PureObject {
        match composition.method.data.clone() {
            Data::Reference(name) => self.apply_reference(name, composition),
            _ => panic!("method's data shouldn't be anything but a reference"),
        }
    }

    pub fn follow_reference(&self, up_to: Inception) -> PureObject {
        match self.data.clone() {
            Data::Reference(name) => {
                let referenced_object = {
                    let universe = UNIVERSE.lock().unwrap();
                    universe[&name].clone()
                };
                referenced_object.evaluate(up_to)
            }
            _ => panic!("attempted to follow object that was not actually a reference"),
        }
    }

    fn apply_reference(&self, name: String, composition: &Composition) -> PureObject {
        match name.as_str() {
            "Concatenate" => self.concatenate(composition),
	    "Truncate" => self.truncate(composition),
	    "Add" => self.add(composition),
	    "Subtract" => self.subtract(composition),
            _ => todo!(),
        }
    }

    fn concatenate(&self, composition: &Composition) -> PureObject {
        match &self.data {
            Data::Bytes(bytes) => {
                let mut bytes = bytes.clone();
                for other_object in &composition.arguments {
                    match other_object.follow_reference(self.inception).data {
                        Data::Bytes(other_bytes) => bytes.append(&mut other_bytes.clone()),
                        _ => panic!("cannot Concatenate with an object of that type"),
                    }
                }
                PureObject {
                    name: self.name.clone(),
                    data: Data::Bytes(bytes),
                    inception: composition.method.inception,
                }
            }
            _ => panic!("attempted Concatenate on object of wrong type (expected Bytes)"),
        }
    }

    fn truncate(&self, composition: &Composition) -> PureObject {
	match &self.data {
	    Data::Bytes(bytes) => {
                let mut bytes = bytes.clone();
                for other_object in &composition.arguments {
                    match other_object.follow_reference(self.inception).data {
                        Data::Integer(to_length) => bytes.truncate(to_length.max(0) as usize),
                        _ => panic!("cannot Truncate an object to a length of that type (expected Integer)"),
                    }
		    break;
                }
                PureObject {
                    name: self.name.clone(),
                    data: Data::Bytes(bytes),
                    inception: composition.method.inception,
                }
            }
            _ => panic!("attempted Truncate on object of wrong type (expected Bytes)"),
	}
    }

    fn add(&self, composition: &Composition) -> PureObject {
	match &self.data {
	    Data::Integer(left_addend) => {
                let mut summand = left_addend.clone();
                for other_object in &composition.arguments {
                    match other_object.follow_reference(self.inception).data {
                        Data::Integer(addend) => summand += addend,
                        _ => panic!("cannot Add an object of that type (expected Integer)"),
                    }
                }
                PureObject {
                    name: self.name.clone(),
                    data: Data::Integer(summand),
                    inception: composition.method.inception,
                }
            }
            _ => panic!("attempted Add on object of wrong type (expected Integer)"),
	}
    }

    fn subtract(&self, composition: &Composition) -> PureObject {
	match &self.data {
	    Data::Integer(left_subbend) => {
                let mut difference = left_subbend.clone();
                for other_object in &composition.arguments {
                    match other_object.follow_reference(self.inception).data {
                        Data::Integer(subbend) => difference -= subbend,
                        _ => panic!("cannot Subtract an object of that type (expected Integer)"),
                    }
                }
                PureObject {
                    name: self.name.clone(),
                    data: Data::Integer(difference),
                    inception: composition.method.inception,
                }
            }
            _ => panic!("attempted Subtract on object of wrong type (expected Integer)"),
	}
    }
}

impl Fundamental for PureObject {
    fn composed(&self) -> bool {
        false
    }

    fn flatten(&self) -> impl Fundamental {
        Object::promote(self)
    }
}

impl Fundamental for Object {
    fn composed(&self) -> bool {
        !self.composition_stack.is_empty()
    }

    fn flatten(&self) -> impl Fundamental {
        self.evaluate(Utc::now())
    }
}

impl Composition {
    pub fn new(method: PureObject, arguments: Vec<PureObject>) -> Self {
        Self { method, arguments }
    }
}
