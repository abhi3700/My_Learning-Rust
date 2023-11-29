//! Vector of different elements(u32, custom type)

#[derive(Debug)]
enum MyType {
    U32(u32),
    Custom(ProducerAuthorityList),
}

#[derive(Debug)]
struct ProducerAuthorityList {
    pub threshold: u32,
    pub keys: Vec<Key>,
}

#[derive(Debug)]
struct Key {
    pub key: String,
    pub weight: u64,
}

pub fn main() {
    let mut v1 = Vec::<MyType>::new();
    v1.push(MyType::U32(0));

    println!("{:?}", v1);

    v1.push(MyType::Custom(ProducerAuthorityList {
        threshold: 1,
        keys: vec![Key {
            key: "EOS7PfA3A4UdfMu2wKbuXdbHn8EWAxbMnFoFWui4X2zsr2oPwdQJP".to_string(),
            weight: 1,
        }],
    }));

    println!("{:?}", v1);
}
