use std::fmt;

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(weight: {}, value: £{})", self.weight, self.value)
    }
}

pub fn generate(item_count: usize) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    for _ in 0..item_count {
        items.push(Item {
            weight: (rand::random::<u32>() % 30) + 2,
            value: (rand::random::<u32>() % 30) + 2 ,
        })
    }

    return items;
}