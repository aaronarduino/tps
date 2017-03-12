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
    cost_of_hotel(am);
}

fn cost_of_hotel<T: Cost>(item: T) {
    let cost = item.price();
    println!("Cost of hotel is: {}", cost);
}
