use ace_prototype::object;
use ace_prototype::object::Fundamental;
use chrono::Utc;

fn main() {
    {
        let a = {
            let mut universe = object::UNIVERSE.lock().unwrap();
            universe.insert(
                "A".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x41]))),
            );
            universe.insert(
                "B".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x42]))),
            );
            universe.insert(
                "C".into(),
                object::Object::promote(&object::Pure::new(object::Data::Bytes(vec![0x43]))),
            );
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![
                        object::Pure::new(object::Data::Reference("B".into())),
                        object::Pure::new(object::Data::Reference("C".into())),
                    ],
                ));
            universe
                .get_mut("C")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![object::Pure::new(object::Data::Reference("C".into()))],
                ));
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::Pure::new(object::Data::Reference("Concatenate".into())),
                    vec![
                        object::Pure::new(object::Data::Reference("B".into())),
                        object::Pure::new(object::Data::Reference("C".into())),
                    ],
                ));
            println!("{universe:#?}");
            universe["A"].clone()
        };
        println!("{:?}", a.evaluate(Utc::now()))
    }
}
