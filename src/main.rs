use prost::Message;
use prost_reflect::ReflectMessage;
use std::collections::BTreeMap;
use std::str;
use vrl::prelude::NotNan;
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

fn to_vrl(prost_reflect_value: prost_reflect::Value) -> Result<vrl::value::Value, Error> {
    let v = match prost_reflect_value {
        prost_reflect::Value::Bool(v) => vrl::value::Value::from(v),
        prost_reflect::Value::I32(v) => vrl::value::Value::from(v),
        prost_reflect::Value::I64(v) => vrl::value::Value::from(v),
        prost_reflect::Value::U32(v) => vrl::value::Value::from(v),
        prost_reflect::Value::U64(v) => vrl::value::Value::from(v),
        prost_reflect::Value::F32(v) => vrl::value::Value::Float(
            NotNan::new(f64::from(v)).map_err(|_e| format!("Float number cannot be Nan"))?,
        ),
        prost_reflect::Value::F64(v) => vrl::value::Value::Float(
            NotNan::new(v).map_err(|_e| format!("F64 number cannot be Nan"))?,
        ),
        prost_reflect::Value::String(v) => vrl::value::Value::from(v),
        prost_reflect::Value::Bytes(v) => vrl::value::Value::from(v),
        prost_reflect::Value::EnumNumber(v) => vrl::value::Value::from(v), // TODO: maybe enum value should the string value
        prost_reflect::Value::Message(mut v) => {
            let mut obj_map = BTreeMap::new();
            for field_desc in v.descriptor().fields() {
                let field = v.get_field_mut(&field_desc);
                let mut taken_value = prost_reflect::Value::Bool(false);
                std::mem::swap(&mut taken_value, field);
                let out = to_vrl(taken_value)?;
                obj_map.insert(field_desc.name().to_string(), out);
            }
            vrl::value::Value::from(obj_map)
        }
        prost_reflect::Value::List(v) => {
            let vec = v
                .into_iter()
                .map(|o| to_vrl(o))
                .collect::<Result<Vec<_>, Error>>()?;
            vrl::value::Value::from(vec)
        }
        prost_reflect::Value::Map(v) => vrl::value::Value::from(
            v.into_iter()
                // TODO: handle unwrap
                .map(|kv| (kv.0.as_str().unwrap().to_string(), to_vrl(kv.1).unwrap()))
                .collect::<BTreeMap<String, _>>(),
        ),
    };
    Ok(v)
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
    let pool = DescriptorPool::decode(include_bytes!("../test_protobuf.desc").as_ref()).unwrap();
    let message_descriptor = pool.get_message_by_name("test_protobuf.Person").unwrap();

    let dynamic_message = DynamicMessage::decode(message_descriptor, p_buf.as_slice()).unwrap();
    println!("as dynamic_message: {:?}", dynamic_message);
    assert_eq!(
        dynamic_message.get_field_by_name("name").unwrap().as_ref(),
        &prost_reflect::Value::String("Someone".to_string())
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
