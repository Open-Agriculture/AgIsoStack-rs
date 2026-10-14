use ag_iso_stack::object_pool::{
    object::{Object, StringVariable, VtString},
    vt_version::VtVersion,
    ObjectId,
};

#[test]
fn deserialized_object_ids_preserve_the_null_sentinel_invariant() {
    assert_eq!(
        serde_json::from_str::<ObjectId>("65534").unwrap().value(),
        65534
    );
    assert!(serde_json::from_str::<ObjectId>("65535").is_err());
}

#[test]
fn legacy_boolean_writes_match_the_reader_constraints() {
    let bytes = vec![175, 0, 7, 0, 1, 0, 255, 255, 255, 255, 2, 1, 0];
    let object = Object::read_for_version(&mut bytes.into_iter(), VtVersion::Version4).unwrap();
    assert!(object.try_write_for_version(VtVersion::Version3).is_err());
    let bytes = object.try_write_for_version(VtVersion::Version4).unwrap();
    assert_eq!(
        Object::read_for_version(&mut bytes.into_iter(), VtVersion::Version4).unwrap(),
        object
    );
}

#[test]
fn widestring_writes_require_vt4() {
    for value in [
        VtString::from("hello"),
        VtString::Encoded {
            encoded_bytes: vec![255, 254, 65, 0],
        },
    ] {
        let object = Object::StringVariable(StringVariable {
            id: ObjectId::new(1).unwrap(),
            value,
        });
        assert!(object.try_write_for_version(VtVersion::Version3).is_err());
        assert!(object.try_write_for_version(VtVersion::Version4).is_ok());
    }
    let object = Object::StringVariable(StringVariable {
        id: ObjectId::new(1).unwrap(),
        value: VtString::Encoded {
            encoded_bytes: b"hello".to_vec(),
        },
    });
    assert!(object.try_write_for_version(VtVersion::Version3).is_ok());
}
