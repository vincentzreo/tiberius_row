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
        let mut yml_str = "".to_string();
        let cols = row
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect::<Vec<_>>();
        for (col, val) in cols.iter().zip(row.into_iter()) {
            match val {
                tiberius::ColumnData::I64(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::I32(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::I16(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::U8(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::F32(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::F64(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::Bit(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::String(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: {}\n", col, "")),
                },
                tiberius::ColumnData::Numeric(v) => match v {
                    Some(v) => yml_str.push_str(&format!("{}: {}\n", col, v)),
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                tiberius::ColumnData::DateTime(v) => match v {
                    Some(v) => {
                        let base_date = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
                        let days_duration = chrono::Duration::days(v.days().into());
                        let seconds = v.seconds_fragments() as f64 / 300.0; // Assuming 300 fragments per second
                        let seconds_duration = chrono::Duration::seconds(seconds.trunc() as i64);
                        let nano_seconds_duration = chrono::Duration::nanoseconds(
                            ((seconds.fract()) * 1_000_000_000.0) as i64,
                        );

                        let naive_datetime = base_date.and_hms_opt(0, 0, 0).unwrap()
                            + days_duration
                            + seconds_duration
                            + nano_seconds_duration;

                        yml_str.push_str(
                            format!(
                                "{}: {}\n",
                                col,
                                chrono::DateTime::<chrono::Utc>::from_utc(
                                    naive_datetime,
                                    chrono::Utc
                                )
                            )
                            .as_str(),
                        );
                    }
                    None => yml_str.push_str(&format!("{}: none\n", col)),
                },
                _ => {
                    return Err(anyhow::anyhow!(format!(
                        "unsupported column data type: {:?}",
                        val
                    )));
                }
            }
        }
        let deserialized: Self = serde_yaml::from_str(Box::leak(Box::new(yml_str)))?;
        Ok(deserialized)
    }
        }
    }
}
