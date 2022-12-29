use num::BigInt;

type Operation = Box<dyn Fn(u128) -> u128>;
type Test = Box<dyn Fn(u128) -> usize>;

struct Monkey {
    pub items: Vec<u128>,
    pub operation: Operation,
    pub test: Test,
}

impl Monkey {
    pub fn get_worry_level(&mut self) -> u128 {
        match self.items.pop() {
            Some(item) => (self.operation)(item),
            None => 0,
        }
    }

    pub fn add_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

fn set_monkeys_sample() -> Vec<Monkey> {
    let mut monkey0 = Monkey {
        items: Vec::new(),
        operation: Box::new(|worry_level: u128| worry_level * 19),
        test: Box::new(|worry_level: u128| if worry_level % 23 == 0 { 2 } else { 3 }),
    };
    monkey0.items.push(79);
    monkey0.items.push(98);

    let mut monkey1 = Monkey {
        items: Vec::new(),
        operation: Box::new(|worry_level: u128| worry_level + 6),
        test: Box::new(|worry_level: u128| if worry_level % 19 == 0 { 2 } else { 0 }),
    };
    monkey1.items.push(54);
    monkey1.items.push(65);
    monkey1.items.push(75);
    monkey1.items.push(74);

    let mut monkey2 = Monkey {
        items: Vec::new(),
        operation: Box::new(|worry_level: u128| worry_level * worry_level),
        test: Box::new(|worry_level: u128| if worry_level % 13 == 0 { 1 } else { 3 }),
    };
    monkey2.items.push(79);
    monkey2.items.push(60);
    monkey2.items.push(97);

    let mut monkey3 = Monkey {
        items: Vec::new(),
        operation: Box::new(|worry_level: u128| worry_level + 3),
        test: Box::new(|worry_level: u128| if worry_level % 17 == 0 { 0 } else { 1 }),
    };

    monkey3.items.push(74);

    Vec::from([monkey0, monkey1, monkey2, monkey3])
}

fn set_monkeys() -> Vec<Monkey> {
    let monkey0 = Monkey {
        items: Vec::from([89, 84, 88, 78, 70]),
        operation: Box::new(|worry_level: u128| worry_level * 5),
        test: Box::new(|worry_level: u128| if worry_level % 7 == 0 { 6 } else { 7}),
    };

    let monkey1 = Monkey {
        items: Vec::from([76, 62, 61, 54, 69, 60, 85]),
        operation: Box::new(|worry_level: u128| worry_level + 1),
        test: Box::new(|worry_level: u128| if worry_level % 17 == 0 { 0 } else { 6 }),
    };

    let monkey2 = Monkey {
        items: Vec::from([83, 89, 53]),
        operation: Box::new(|worry_level: u128| worry_level + 8),
        test: Box::new(|worry_level: u128| if worry_level % 11 == 0 { 5 } else { 3 }),
    };

    let monkey3 = Monkey {
        items: Vec::from([95, 94, 85, 57]),
        operation: Box::new(|worry_level: u128| worry_level + 4),
        test: Box::new(|worry_level: u128| if worry_level % 13 == 0 { 0 } else { 1 }),
    };

    let monkey4 = Monkey {
        items: Vec::from([82, 98]),
        operation: Box::new(|worry_level: u128| worry_level + 7),
        test: Box::new(|worry_level: u128| if worry_level % 19 == 0 { 5 } else { 2 }),
    };

    let monkey5 = Monkey {
        items: Vec::from([69]),
        operation: Box::new(|worry_level: u128| worry_level + 2),
        test: Box::new(|worry_level: u128| if worry_level % 2 == 0 { 1 } else { 3 }),
    };

    let monkey6 = Monkey {
        items: Vec::from([82, 70, 58, 87, 59, 99, 92, 65]),
        operation: Box::new(|worry_level: u128| worry_level * 11),
        test: Box::new(|worry_level: u128| if worry_level % 5 == 0 { 7 } else { 4 }),
    };

    let monkey7 = Monkey {
        items: Vec::from([91, 53, 96, 98, 68, 82]),
        operation: Box::new(|worry_level: u128| worry_level * worry_level),
        test: Box::new(|worry_level: u128| if worry_level % 3 == 0 { 4 } else { 2 }),
    };

    Vec::from([monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7])
}

fn main() {
    let mut monkeys = set_monkeys();
    let mut inspections = Vec::from([0; 8]);
    let rounds = 10_000;
    let mod_all = 2 * 3 * 5 * 7 * 13 * 11 * 17 * 19;
    //let mod_all = 13 * 17 * 19 * 23;

    for _r in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut worry_level = monkeys[i].get_worry_level();

                worry_level %= mod_all;
                let next_monkey_id = (monkeys[i].test)(worry_level);
                monkeys[next_monkey_id].add_item(worry_level);
                inspections[i] += 1;
            }
        }
    }

    for i in 0..monkeys.len() {
        println!("Mokey {}, inspected items {} times, it has items {:?}", i, inspections[i], monkeys[i].items);
    }
    inspections.sort();
    println!("Inspections {:?}", inspections);
    let last = BigInt::from(inspections[inspections.len() - 1]);
    let prelast = BigInt::from(inspections[inspections.len() - 2]);
    println!("Result: {}", last * prelast);
}
