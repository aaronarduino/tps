enum Plan<'a> {
    Hotel(&'a Hotel<'a>),
    Airline(&'a Airline),
}

struct Hotel<'a> {
    p: f32,
    name: &'a str,
}

struct Airline {
    p: f32,
}

impl<'a> Cost for Hotel<'a> {
    fn price(&self) -> f32 {
        self.p
    }
}

impl Cost for Airline {
    fn price(&self) -> f32 {
        self.p
    }
}

trait Cost {
    fn price(&self) -> f32;
}

fn main() {
    let am = Hotel{p: 200.00, name: "Some Hotel"};
    let al = Airline{p: 800.00};
    let mut plan: Vec<Plan> = Vec::new();

    plan.push(Plan::Airline(&al));
    plan.push(Plan::Hotel(&am));

    for val in plan {
        match val {
            Plan::Airline(v) => cost_of_item(v),
            Plan::Hotel(v) => cost_of_item(v),
        }
    }
    println!("Name of hotel is: {}", am.name);
}

fn cost_of_item<T: Cost>(item: &T) {
    let cost = item.price();
    println!("Cost of hotel is: ${}", cost);
}
