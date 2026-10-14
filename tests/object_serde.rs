use ag_iso_stack::object_pool::object::{NumberVariable, Object};
use ag_iso_stack::object_pool::ObjectId;
use serde_json::json;

#[test]
fn object_json_round_trip() {
    let object = Object::NumberVariable(NumberVariable {
        id: ObjectId::new(123).unwrap(),
        value: 42,
    });

    let json = serde_json::to_value(&object).unwrap();
    assert_eq!(
        json,
        json!({
            "type": "NumberVariable",
            "properties": { "id": 123, "value": 42 }
        })
    );
    assert_eq!(serde_json::from_value::<Object>(json).unwrap(), object);
}
