use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use prost::Message;

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

fn criterion_benchmark(c: &mut Criterion) {
    let mut p = test_protobuf::Person::default();
    p.name = Some("Someone with a long long long long long long name".to_string());
    for _i in 0..100 {
        p.phones.push(test_protobuf::person::PhoneNumber {
            number: Some("123412341234123412341234123412341234123412341234".to_string()),
            r#type: Some(i32::from(test_protobuf::person::PhoneType::Mobile)),
        });
    }
    let p_buf = serialize_person(&p);

    use prost_reflect::{DescriptorPool, DynamicMessage};

    // test_protobuf.desc will be created by build.rs
    let pool = DescriptorPool::decode(include_bytes!("../test_protobuf.desc").as_ref()).unwrap();
    let message_descriptor = pool.get_message_by_name("test_protobuf.Person").unwrap();

    let dynamic_message =
        DynamicMessage::decode(message_descriptor.clone(), p_buf.as_slice()).unwrap();

    let mut group = c.benchmark_group("test");
    //group.sample_size(10000000);
    group.measurement_time(Duration::from_secs(20));
    group.warm_up_time(Duration::from_secs(10));

    group.bench_function("to_vrl", |b| {
        b.iter(|| {
            protobuf_desc_demo::to_vrl(black_box(prost_reflect::Value::Message(
                dynamic_message.clone(),
            )))
        })
    });
    group.bench_function("to_vrl_via_json", |b| {
        b.iter(|| protobuf_desc_demo::to_vrl_via_json(black_box(dynamic_message.clone())))
    });

    group.bench_function("to_vrl_with_kind", |b| {
        b.iter(|| {
            protobuf_desc_demo::to_vrl_with_kind(
                black_box(prost_reflect::Value::Message(dynamic_message.clone())),
                black_box(&prost_reflect::Kind::Message(message_descriptor.clone())),
            )
        })
    });

    group.bench_function("to_vrl_by_ref", |b| {
        b.iter(|| {
            protobuf_desc_demo::to_vrl_by_ref(black_box(&prost_reflect::Value::Message(
                dynamic_message.clone(),
            )))
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
