use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(row_ext))]
struct StructData {
    ident: syn::Ident,
    generics: syn::Generics,
}

pub(crate) fn process_row_ext(input: DeriveInput) -> TokenStream {
    let StructData { ident, generics } =
        StructData::from_derive_input(&input).expect("failed to parse input");
    quote! {
        impl #generics #ident #generics {
        pub fn from_row(row: tiberius::Row) -> Result<Self, anyhow::Error>
    where
        Self: Sized + serde::Deserialize<'static>,
    {
        use tiberius::FromSql as _;
        let mut json_map = serde_json::Map::new();
        let cols = row
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect::<Vec<_>>();

        for (col, val) in cols.iter().zip(row.into_iter()) {
            match val {
                tiberius::ColumnData::I64(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::Number(v.into()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::I32(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::Number(v.into()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::I16(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::Number((v as i64).into()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::U8(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::Number((v as u64).into()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::F32(v) => match v {
                    Some(v) => {
                        if let Some(num) = serde_json::Number::from_f64(v as f64) {
                            json_map.insert(col.clone(), serde_json::Value::Number(num));
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::F64(v) => match v {
                    Some(v) => {
                        if let Some(num) = serde_json::Number::from_f64(v) {
                            json_map.insert(col.clone(), serde_json::Value::Number(num));
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Bit(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::Bool(v));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::String(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::String(String::new()));
                    }
                },
                tiberius::ColumnData::Numeric(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Guid(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Binary(v) => match v {
                    Some(v) => {
                        let base64_str = base64::encode(v);
                        json_map.insert(col.clone(), serde_json::Value::String(base64_str));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Xml(v) => match v {
                    Some(v) => {
                        json_map.insert(col.clone(), serde_json::Value::String(v.to_string()));
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::DateTime(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                            &tiberius::ColumnData::DateTime(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string().replace(" ", "T")),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::SmallDateTime(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                            &tiberius::ColumnData::SmallDateTime(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string()),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Time(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::Time::from_sql(
                            &tiberius::ColumnData::Time(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string()),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::Date(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::Date::from_sql(
                            &tiberius::ColumnData::Date(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string()),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::DateTime2(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                            &tiberius::ColumnData::DateTime2(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string().replace(" ", "T")),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
                tiberius::ColumnData::DateTimeOffset(v) => match v {
                    Some(v) => {
                        let p_datetime = tiberius::time::time::OffsetDateTime::from_sql(
                            &tiberius::ColumnData::DateTimeOffset(Some(v)),
                        );
                        if let Ok(Some(p_datetime)) = p_datetime {
                            json_map.insert(
                                col.clone(),
                                serde_json::Value::String(p_datetime.to_string()),
                            );
                        } else {
                            json_map.insert(col.clone(), serde_json::Value::Null);
                        }
                    }
                    None => {
                        json_map.insert(col.clone(), serde_json::Value::Null);
                    }
                },
            }
        }

        let json_value = serde_json::Value::Object(json_map);

        let deserialized: Self = serde_json::from_value(json_value)?;
        Ok(deserialized)
    }
        }
    }
}
