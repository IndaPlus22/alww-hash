use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use std::{error::Error, fs::OpenOptions, io::Read};

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
        self.db.insert(input);
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
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("data.csv")
            .unwrap();

        let mut writer = csv::Writer::from_writer(file);
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
    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data.csv")
            .unwrap();
        let mut csv = String::new();
        file.read_to_string(&mut csv);

        let mut reader = csv::Reader::from_reader(csv.as_bytes());

        for node in reader.records() {
            let node = node?;
            let key: usize = node[0].parse().unwrap();
            let value = node[1].to_owned();
            let node = HashNode { key, value };
            self.insert(node.value);
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

pub trait Hash {
    fn hash(&self) -> usize;
}

pub trait ToString {
    fn to_string_(&self) -> String;
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

impl ToString for String {
    fn to_string_(&self) -> String {
        self.to_owned()
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

impl ToString for isize {
    fn to_string_(&self) -> String {
        self.to_string()
    }
}

impl Hash for usize {
    fn hash(&self) -> usize {
        *self
    }
}

impl ToString for usize {
    fn to_string_(&self) -> String {
        self.to_string()
    }
}
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
    fn insert<T>(&mut self, input: T)
    where
        T: Hash + ToString,
    {
        let key = self.hash(&input);
        let node = HashNode::new(key, input.to_string_());
        let value_exists = self.list[key].contains(&node);
        if !value_exists {
            self.list[key].push(node);
            self.n += 1;
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
    fn hp<T>(&self, input: T)
    where
        T: Hash + ToString,
    {
        let key = self.hash(&input);
        println!("{}", key);
    }
    fn resize(&mut self) {
        if (self.n as f64 / self.m as f64) < 0.5 {
            self.m /= 2;
        } else if (self.n as f64 / self.m as f64) >= 1.5 {
            let l = self.m;
            self.m *= 2;
            for x in 0..(self.m - l) {
                self.list.push(Vec::<HashNode>::new());
            }
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
            self.insert(item.value);
        }
    }
}
