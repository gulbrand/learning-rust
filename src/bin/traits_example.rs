use std::fmt::{Debug, Error, Formatter};

struct Dataset {
    key: String,
    version: String,
}

impl Debug for Dataset {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.key.as_str());
        f.write_str(self.version.as_str())
    }
}

fn main() {
    println!("hi");
    let data_set = Dataset {
        key: "abc".to_string(),
        version: "1.0".to_string(),
    };
    println!("data_set = {:?}", data_set);
}