use ace_prototype::object::{self, Fundamental};

fn main() {
    {
        let a = {
            let mut universe = object::UNIVERSE.lock().unwrap();
            universe.insert(
                "A".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x41]))),
            ); // A <- "A"
            universe.insert(
                "B".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x42]))),
            ); // B <- "B"
            universe.insert(
                "C".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x43]))),
            ); // C <- "C"
            universe.insert(
                "One".into(),
                object::Object::promote(&object::Pure::new(object::Data::Integer(1))),
            ); // One <- 1
            universe.insert(
                "Two".into(),
                object::Object::promote(&object::Pure::new(object::Data::Integer(2))),
            ); // Two <- 2
            universe.insert(
                "Fish".into(),
                object::Object::promote(&object::Pure::new(object::Data::Integer(0))),
            ); // Fish <- 0
            universe
                .get_mut("Fish")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Add".into())),
                    vec![
                        object::Pure::new(object::Data::Reference("One".into())),
                        object::Pure::new(object::Data::Reference("Two".into())),
                    ],
                )); // Fish <- Add(Fish, One, Two)
            universe
                .get_mut("Fish")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Add".into())),
                    vec![object::Pure::new(object::Data::Reference("Fish".into()))],
                )); // Fish <- Add(Fish, Fish)
            universe
                .get_mut("Fish")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Subtract".into())),
                    vec![object::Pure::new(object::Data::Reference("One".into()))],
                )); // Fish <- Subtract(Fish, One)
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![
                        object::Pure::new(object::Data::Reference("B".into())),
                        object::Pure::new(object::Data::Reference("C".into())),
                    ],
                )); // A <- Concatenate(A, B, C)
            universe
                .get_mut("C")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![object::Pure::new(object::Data::Reference("C".into()))],
                )); // C <- Concatencate(C, C)
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![
                        object::Pure::new(object::Data::Reference("B".into())),
                        object::Pure::new(object::Data::Reference("C".into())),
                    ],
                )); // A <- Concatenate(A, B, C)
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Truncate".into())),
                    vec![object::Pure::new(object::Data::Reference("Fish".into()))],
                )); // A <- Truncate(A, Fish)
            println!("{universe:#?}");
            universe["A"].clone()
        };
        println!("{:?}", a.flatten()); // Show(A) ;; Should be "ABCBC"
    }
}
