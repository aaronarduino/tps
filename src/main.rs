enum Plan {
    Hotel,
    Airline,
    PointOfInterest,
}

struct Hotel {
    p: f32,
}

struct Airline {
    p: f32,
}

impl Cost for Hotel {
    fn price(&self) -> f32 {
        self.p
    }
}

trait Cost {
    fn price(&self) -> f32;
}

fn main() {
    let am = Hotel{p: 200.00};
    let bm = Airline{p: 300.00};

    let mut ab:Cost = Vec::new();

    ab.push(am);
    ab.push(bm);

    cost_of_items(am);
}

fn cost_of_items<T: Cost>(items: Vec<T>) {
    let cost = item.price();
    println!("Cost of hotel is: ${}", cost);
}
