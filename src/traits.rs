trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting with {}", color);
    }
}

//Super trait
trait Vehicle: Paint {
    fn drive(&self);
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting house {}", color);
    }
}

impl Paint for Car {}
impl Vehicle for Car {
    fn drive(&self) {
        println!("Driving car");
    }
}
impl Paint for Truck {}

impl Park for Car {
    fn park(&self) {
        println!("Parking car");
    }
}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck");
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking truck");
    }
}

fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}

fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T)
where
    T: Vehicle,
{
    object.paint("red".to_owned());
}

//Similiar to how a factory function works
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Ford".to_owned(),
                model: "Mustang".to_owned(),
                year: 2013,
            },
        })
    } else {
        Box::new(House {})
    }
}

pub fn run_example() {
    let car = Car {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            model: "Mustang".to_owned(),
            year: 2013,
        },
    };

    let truck = Truck {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            model: "F-150".to_owned(),
            year: 2013,
        },
    };

    let house = House {};

    let paitable = create_paintable_object(true);
    let _paitable_objects: Vec<&dyn Paint> = vec![&car, &truck, &house];

    paint_red(paitable.as_ref());
    paint_red(&truck);
    paint_red(&house);

    paint_red2(&truck);
    paint_red2(&house);

    paint_vehicle_red(&car);

    truck.park();
    truck.unload();
}
