use prost_reflect::ReflectMessage;
use std::collections::BTreeMap;
use vrl::prelude::NotNan;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn to_vrl_with_kind(
    prost_reflect_value: prost_reflect::Value,
    kind: &prost_reflect::Kind,
) -> Result<vrl::value::Value, Error> {
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
        prost_reflect::Value::EnumNumber(v) => {
            let enum_desc = kind.as_enum().unwrap();
            vrl::value::Value::from(
                enum_desc
                    .get_value(v)
                    .ok_or_else(|| format!("The number {} cannot be in {}", v, enum_desc.name()))?
                    .name(),
            )
        }
        prost_reflect::Value::Message(mut v) => {
            let mut obj_map = BTreeMap::new();
            for field_desc in v.descriptor().fields() {
                let field = v.get_field_mut(&field_desc);
                let mut taken_value = prost_reflect::Value::Bool(false);
                std::mem::swap(&mut taken_value, field);
                let out = to_vrl_with_kind(taken_value, &field_desc.kind())?;
                obj_map.insert(field_desc.name().to_string(), out);
            }
            vrl::value::Value::from(obj_map)
        }
        prost_reflect::Value::List(v) => {
            let vec = v
                .into_iter()
                .map(|o| to_vrl_with_kind(o, &kind))
                .collect::<Result<Vec<_>, Error>>()?;
            vrl::value::Value::from(vec)
        }
        prost_reflect::Value::Map(v) => {
            let message_desc = kind.as_message().unwrap();
            vrl::value::Value::from(
                v.into_iter()
                    // TODO: handle unwrap
                    .map(|kv| {
                        (
                            kv.0.as_str().unwrap().to_string(),
                            to_vrl_with_kind(kv.1, &message_desc.map_entry_value_field().kind())
                                .unwrap(),
                        )
                    })
                    .collect::<BTreeMap<String, _>>(),
            )
        }
    };
    Ok(v)
}

pub fn to_vrl(prost_reflect_value: prost_reflect::Value) -> Result<vrl::value::Value, Error> {
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

pub fn to_vrl_by_ref(
    prost_reflect_value: &prost_reflect::Value,
) -> Result<vrl::value::Value, Error> {
    let v = match prost_reflect_value {
        prost_reflect::Value::Bool(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::I32(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::I64(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::U32(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::U64(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::F32(v) => vrl::value::Value::Float(
            NotNan::new(f64::from(v.clone()))
                .map_err(|_e| format!("Float number cannot be Nan"))?,
        ),
        prost_reflect::Value::F64(v) => vrl::value::Value::Float(
            NotNan::new(v.clone()).map_err(|_e| format!("F64 number cannot be Nan"))?,
        ),
        prost_reflect::Value::String(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::Bytes(v) => vrl::value::Value::from(v.clone()),
        prost_reflect::Value::EnumNumber(v) => vrl::value::Value::from(v.clone()), // TODO: maybe enum value should the string value
        prost_reflect::Value::Message(v) => {
            let mut obj_map = BTreeMap::new();
            for field_desc in v.descriptor().fields() {
                let field_value = v.get_field(&field_desc);
                let out = to_vrl_by_ref(field_value.as_ref())?;
                obj_map.insert(field_desc.name().to_string(), out);
            }
            vrl::value::Value::from(obj_map)
        }
        prost_reflect::Value::List(v) => {
            let vec = v
                .into_iter()
                .map(|o| to_vrl_by_ref(&o))
                .collect::<Result<Vec<_>, Error>>()?;
            vrl::value::Value::from(vec)
        }
        prost_reflect::Value::Map(v) => vrl::value::Value::from(
            v.into_iter()
                // TODO: handle unwrap
                .map(|kv| {
                    (
                        kv.0.as_str().unwrap().to_string(),
                        to_vrl_by_ref(kv.1).unwrap(),
                    )
                })
                .collect::<BTreeMap<String, _>>(),
        ),
    };
    Ok(v)
}

pub fn to_vrl_via_json(
    dynamic_message: prost_reflect::DynamicMessage,
) -> Result<vrl::value::Value, Error> {
    // To json string
    let mut serializer = serde_json::Serializer::new(vec![]);
    let options = prost_reflect::SerializeOptions::new().skip_default_fields(false);
    dynamic_message
        .serialize_with_options(&mut serializer, &options)
        .unwrap();
    let json_vec = serializer.into_inner();

    let json_value: serde_json::value::Value = serde_json::from_slice(json_vec.as_slice()).unwrap();

    let v = vrl::value::Value::from(json_value);
    Ok(v)
}
