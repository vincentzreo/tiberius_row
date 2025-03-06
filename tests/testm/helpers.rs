use tiberius::Query;
use tiberius_db_tester::DBTester;

use tiberius::FromSql as _;

pub fn convert_tiberius_columndata_to_string(data: tiberius::ColumnData) -> String {
    match data {
        tiberius::ColumnData::U8(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::I16(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::I32(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::I64(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::F32(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::F64(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::Bit(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::String(data) => match data {
            Some(data) => data.to_string(),
            None => "".to_string(),
        },
        tiberius::ColumnData::Guid(data) => match data {
            Some(data) => data.to_string(),
            None => "".to_string(),
        },
        tiberius::ColumnData::Binary(data) => match data {
            Some(data) => {
                format!("{:?}", String::from_utf8_lossy(&data))
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::Numeric(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::Xml(data) => match data {
            Some(data) => {
                format!("{}", data)
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::DateTime(data) => match data {
            Some(data) => {
                let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                    &tiberius::ColumnData::DateTime(Some(data)),
                );
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::SmallDateTime(data) => match data {
            Some(data) => {
                let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                    &tiberius::ColumnData::SmallDateTime(Some(data)),
                );
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::Time(data) => match data {
            Some(data) => {
                let p_datetime =
                    tiberius::time::time::Time::from_sql(&tiberius::ColumnData::Time(Some(data)));
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::Date(data) => match data {
            Some(data) => {
                let p_datetime =
                    tiberius::time::time::Date::from_sql(&tiberius::ColumnData::Date(Some(data)));
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::DateTime2(data) => match data {
            Some(data) => {
                let p_datetime = tiberius::time::time::PrimitiveDateTime::from_sql(
                    &tiberius::ColumnData::DateTime2(Some(data)),
                );
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
        tiberius::ColumnData::DateTimeOffset(data) => match data {
            Some(data) => {
                let p_datetime = tiberius::time::time::OffsetDateTime::from_sql(
                    &tiberius::ColumnData::DateTimeOffset(Some(data)),
                );
                if let Ok(Some(p_datetime)) = p_datetime {
                    format!("{}", p_datetime)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        },
    }
}

#[tokio::test]
async fn test_create_table() -> anyhow::Result<()> {
    let test_app = DBTester::new("101.95.95.58", 1433, "sa", "Ibm123", "migrates/test.sql");
    let mut client = test_app.get_client().await;
    let query = Query::new("SELECT * FROM test");
    let rows = query.query(&mut client).await?.into_first_result().await?;
    // println!("{:?}", rows);
    assert!(rows.len() > 0);
    for r in rows {
        let cols = r
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect::<Vec<_>>();
        for (col, val) in cols.iter().zip(r.into_iter()) {
            println!("{}: {}", col, convert_tiberius_columndata_to_string(val));
        }
    }
    Ok(())
}
