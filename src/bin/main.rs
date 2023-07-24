use prost::Message;
use protobuf_desc_demo::to_vrl;
use std::str;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

// Include the `test_protobuf` module, which is generated from items.proto.
pub mod test_protobuf {
    include!(concat!(env!("OUT_DIR"), "/test_protobuf.rs"));
}

pub fn serialize_person(person: &test_protobuf::Person) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(person.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    person.encode(&mut buf).unwrap();
    buf
}

fn main() {
    let mut p = test_protobuf::Person::default();
    p.name = Some("Someone".to_string());
    p.phones.push(test_protobuf::person::PhoneNumber {
        number: Some("1234".to_string()),
        r#type: Some(i32::from(test_protobuf::person::PhoneType::Mobile)),
    });
    let p_buf = serialize_person(&p);

    use prost_reflect::{DescriptorPool, DynamicMessage};

    // test_protobuf.desc will be created by build.rs
    let pool = DescriptorPool::decode(include_bytes!("../../test_protobuf.desc").as_ref()).unwrap();
    let message_descriptor = pool.get_message_by_name("test_protobuf.Person").unwrap();

    let dynamic_message = DynamicMessage::decode(message_descriptor, p_buf.as_slice()).unwrap();
    println!("as dynamic_message: {:?}", dynamic_message);
    assert_eq!(
        dynamic_message.get_field_by_name("name").unwrap().as_ref(),
        &prost_reflect::Value::String("someone".to_string())
    );

    // To json string
    let mut serializer = serde_json::Serializer::new(vec![]);
    let options = prost_reflect::SerializeOptions::new().skip_default_fields(false);
    dynamic_message
        .serialize_with_options(&mut serializer, &options)
        .unwrap();
    println!(
        "as json: {:?}",
        str::from_utf8(&serializer.into_inner()).unwrap()
    );

    // To vrl
    let proto_vrl = to_vrl(prost_reflect::Value::Message(dynamic_message));
    println!("as vrl: {:?}", proto_vrl);
}
