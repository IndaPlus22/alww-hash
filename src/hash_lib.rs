use crate::traits::*;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use std::fs::File;
use std::io::Write;
use std::{error::Error, fs::OpenOptions, io::Read};

/////////////////////////////////////////////////////////////////////////////////
//                                DB-abstraction                               //
/////////////////////////////////////////////////////////////////////////////////
pub struct DB {
    db: HashMap,
}

impl DB {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
    pub fn insert<T>(&mut self, input: T)
    where
        T: Hash + ToString,
    {
        self.db.insert(input, true);
    }
    pub fn delete<T>(&mut self, input: T)
    where
        T: Hash + ToString,
    {
        self.db.delete(input);
    }
    pub fn get<T>(&mut self, input: T) -> Option<usize>
    where
        T: Hash + ToString,
    {
        self.db.get(input)
    }
    pub fn write(&self) {
        let mut writer = csv::Writer::from_path("data.csv").unwrap();
        writer.serialize(("Key", "Value"));

        for list in &self.db.list {
            if list.is_empty() {
                continue;
            }
            for node in list {
                writer.serialize(node);
            }
        }

        writer.flush();
    }
    pub fn load(&mut self, file: &mut File) -> Result<(), Box<dyn Error>> {
        let mut csv = String::new();
        file.read_to_string(&mut csv);

        let mut reader = csv::Reader::from_reader(csv.as_bytes());

        for node in reader.records() {
            let node = node?;
            let value = node[1].to_owned();
            self.insert(value);
        }
        Ok(())
    }
    pub fn print(&self) {
        let mut items: Vec<HashNode> = Vec::new();
        for list in &self.db.list {
            if list.is_empty() {
                continue;
            }
            for item in list {
                items.push(item.to_owned());
            }
        }
        for item in items {
            println!("{}", item)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////
//                                HashNode                                     //
/////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, PartialEq, Debug, Deserialize, Eq)]
struct HashNode {
    key: usize,
    value: String,
}

impl std::fmt::Display for HashNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<key: {}, value: \"{}\">", self.key, self.value)
    }
}

impl Serialize for HashNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl HashNode {
    fn new(key: usize, value: String) -> Self {
        Self { key, value }
    }
}
/////////////////////////////////////////////////////////////////////////////////
//                                HashMap                                      //
/////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct HashMap {
    list: Vec<Vec<HashNode>>,
    n: usize,
    m: usize,
}

impl HashMap {
    fn new() -> Self {
        Self {
            list: vec![Vec::<HashNode>::new(); 2],
            n: 0,
            m: 2,
        }
    }
    fn insert<T>(&mut self, input: T, do_it: bool)
    where
        T: Hash + ToString,
    {
        let key = self.hash(&input);
        let node = HashNode::new(key, input.to_string_());
        let value_exists = self.list[key].contains(&node);
        if !value_exists {
            self.list[key].push(node);
            if do_it {
                self.n += 1;
            }
            self.resize();
        }
        // eprintln!("\n\n{:?}\n\n", self)
    }
    fn delete<T>(&mut self, input: T)
    where
        T: Hash + ToString,
    {
        let key = self.hash(&input);
        let node = HashNode::new(key, input.to_string_());
        let value_exists = self.list[key].contains(&node);
        if value_exists {
            let position = self.list[key].iter().position(|x| *x == node).unwrap();
            self.list[key].remove(position);
            self.n -= 1;
            self.resize();
        }
    }
    fn get<T>(&mut self, input: T) -> Option<usize>
    where
        T: Hash + ToString,
    {
        let key = self.hash(&input);
        let node = HashNode::new(key, input.to_string_());
        let value_exists = self.list[key].contains(&node);
        if !value_exists {
            return None;
        }
        Some(key)
    }
    fn hash<T>(&self, input: &T) -> usize
    where
        T: Hash + ToString,
    {
        input.hash() % self.m
    }
    fn resize(&mut self) {
        if (self.n as f64 / self.m as f64) < 0.5 {
            self.m /= 2;
        } else if (self.n as f64 / self.m as f64) >= 1.5 {
            let l = self.m;
            self.m *= 2;
        } else {
            return;
        }

        let mut items: Vec<HashNode> = Vec::new();
        for list in &self.list {
            if list.is_empty() {
                continue;
            }
            for item in list {
                items.push(item.to_owned());
            }
        }
        self.list = vec![Vec::<HashNode>::new(); self.m];
        for item in items {
            self.insert(item.value, false);
        }
    }
}
