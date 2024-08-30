use ace_prototype::object;
use ace_prototype::object::Fundamental;
use chrono::Utc;

fn main() {
    {
        let a = {
            let mut universe = object::UNIVERSE.lock().unwrap();
            universe.insert(
                "A".into(),
                object::Object::promote(&object::PureObject::new(
                    "A".into(),
                    object::Data::Bytes(vec![0x41]),
                )),
            );
            universe.insert(
                "B".into(),
                object::Object::promote(&object::PureObject::new(
                    "B".into(),
                    object::Data::Bytes(vec![0x42]),
                )),
            );
            universe.insert(
                "C".into(),
                object::Object::promote(&object::PureObject::new(
                    "C".into(),
                    object::Data::Bytes(vec![0x43]),
                )),
            );
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::PureObject::new(
                        "".into(),
                        object::Data::Reference("Concatenate".into()),
                    ),
                    vec![
                        object::PureObject::new("".into(), object::Data::Reference("B".into())),
                        object::PureObject::new("".into(), object::Data::Reference("C".into())),
                    ],
                ));
            universe
                .get_mut("C")
                .unwrap()
                .compose(object::Composition::new(
                    object::PureObject::new(
                        "".into(),
                        object::Data::Reference("Concatenate".into()),
                    ),
                    vec![object::PureObject::new(
                        "".into(),
                        object::Data::Reference("C".into()),
                    )],
                ));
            universe
                .get_mut("A")
                .unwrap()
                .compose(object::Composition::new(
                    object::PureObject::new(
                        "".into(),
                        object::Data::Reference("Concatenate".into()),
                    ),
                    vec![
                        object::PureObject::new("".into(), object::Data::Reference("B".into())),
                        object::PureObject::new("".into(), object::Data::Reference("C".into())),
                    ],
                ));
            println!("{universe:#?}");
            universe["A"].clone()
        };
        println!("{:?}", a.evaluate(Utc::now()))
    }
}
