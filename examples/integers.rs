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
            ); // A <- "A"
            universe.insert(
                "B".into(),
                object::Object::promote(&object::PureObject::new(
                    "B".into(),
                    object::Data::Bytes(vec![0x42]),
                )),
            ); // B <- "B"
            universe.insert(
                "C".into(),
                object::Object::promote(&object::PureObject::new(
                    "C".into(),
                    object::Data::Bytes(vec![0x43]),
                )),
            ); // C <- "C"
	    universe.insert(
		"One".into(),
		object::Object::promote(&object::PureObject::new(
		    "One".into(),
		    object::Data::Integer(1),
		)),
	    ); // One <- 1
	    universe.insert(
		"Two".into(),
		object::Object::promote(&object::PureObject::new(
		    "Two".into(),
		    object::Data::Integer(2),
		)),
	    ); // Two <- 2
	    universe.insert(
		"Fish".into(),
		object::Object::promote(&object::PureObject::new(
		    "Fish".into(),
		    object::Data::Integer(0),
		)),
	    ); // Fish <- 0
	    universe
		.get_mut("Fish")
		.unwrap()
		.compose(object::Composition::new(
		    object::PureObject::new(
			"".into(),
			object::Data::Reference("Add".into()),
		    ),
		    vec![
			object::PureObject::new("".into(), object::Data::Reference("One".into())),
			object::PureObject::new("".into(), object::Data::Reference("Two".into())),
		    ]
		)); // Fish <- Fish.Add(One, Two)
	    universe
		.get_mut("Fish")
		.unwrap()
		.compose(object::Composition::new(
		    object::PureObject::new(
			"".into(),
			object::Data::Reference("Add".into()),
		    ),
		    vec![
			object::PureObject::new("".into(), object::Data::Reference("Fish".into())),
		    ]
		)); // Fish <- Fish.Add(Fish)
	    universe
		.get_mut("Fish")
		.unwrap()
		.compose(object::Composition::new(
		    object::PureObject::new(
			"".into(),
			object::Data::Reference("Subtract".into()),
		    ),
		    vec![
			object::PureObject::new("".into(), object::Data::Reference("One".into())),
		    ]
		)); // Fish <- Fish.Subtract(One)
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
                )); // A <- A.Concatenate(B, C)
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
                )); // C <- C.Concatencate(C)
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
                )); // A <- A.Concatenate(B, C)
	    universe
		.get_mut("A")
		.unwrap()
		.compose(object::Composition::new(
                    object::PureObject::new(
                        "".into(),
                        object::Data::Reference("Truncate".into()),
                    ),
                    vec![
                        object::PureObject::new("".into(), object::Data::Reference("Fish".into())),
                    ],
                )); // A <- A.Truncate(Fish)
            println!("{universe:#?}");
            universe["A"].clone()
        };
        println!("{:?}", a.evaluate(Utc::now())) // Show(A) ;; Should be "ABCBC"
    }
}
