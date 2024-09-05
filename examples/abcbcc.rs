use ace_prototype::object;
use ace_prototype::object::Fundamental;

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
                )); // C <- Concatenate(C, C)
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
            println!("{universe:#?}");
            universe["A"].clone()
        };
        println!("{:?}", a.flatten()); // Show(A) ;; Should be "ABCBCC"
    }
}
