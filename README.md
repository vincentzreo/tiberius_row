# Tiberius Row

`tiberius_row` 是一个Rust过程宏库，用于简化从Tiberius SQL Server客户端获取的行数据到Rust结构体的转换过程。

## 功能特点

- 提供`Row_Ext`派生宏，自动为结构体实现从Tiberius行数据到Rust结构体的转换
- 支持多种SQL数据类型，包括整数、浮点数、字符串、日期时间等
- 使用serde进行数据反序列化，提供类型安全的数据访问
- 自动处理NULL值
- 提供友好的错误处理

## 安装

在你的`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
tiberius_row = "0.1.0"
base64 = "0.22.1"
tiberius = { version = "0.12.3", features = ["chrono", "rust_decimal", "time"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.140"
anyhow = "1.0"
```

## 使用方法

1. 首先，为你的结构体派生必要的特性：

```rust
use serde::Deserialize;
use tiberius_row::Row_Ext;

#[derive(Deserialize, Row_Ext)]
struct User {
    id: i32,
    name: String,
    email: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}
```

2. 然后，你可以直接从Tiberius的Row转换为你的结构体：

```rust
use anyhow::Result;
use tiberius::Client;

async fn get_user(client: &mut Client<Connection>, user_id: i32) -> Result<User> {
    let query = "SELECT id, name, email, created_at FROM users WHERE id = @P1";
    let stream = client.query(query, &[&user_id]).await?;
    let row = stream.into_row().await?.expect("No user found");
    
    // 使用派生的from_row方法
    let user = User::from_row(row)?;
    Ok(user)
}
```

## 支持的数据类型

下表展示了SQL Server数据类型与Rust类型的对应关系：

| SQL Server 数据类型 | Rust 类型                    | 说明                           |
|-------------------|----------------------------|--------------------------------|
| INT               | i32                        | 32位整数                        |
| BIGINT            | i64 或 i128                | 64位整数                        |
| SMALLINT          | i16                        | 16位整数                        |
| TINYINT           | u8                         | 8位无符号整数                    |
| BIT               | bool                       | 布尔值                          |
| DECIMAL/NUMERIC   | tiberius::numeric::Decimal | 高精度数值                       |
| FLOAT             | f64                        | 64位浮点数                       |
| REAL              | f32                        | 32位浮点数                       |
| DATE              | chrono::NaiveDate          | 日期                            |
| DATETIME          | chrono::NaiveDateTime      | 日期时间                         |
| DATETIME2         | chrono::NaiveDateTime      | 高精度日期时间                    |
| TIME              | chrono::NaiveTime          | 时间                            |
| CHAR/VARCHAR      | String                     | 固定/可变长度字符串                |
| NCHAR/NVARCHAR    | String                     | Unicode固定/可变长度字符串         |
| TEXT/NTEXT        | String                     | 长文本                          |
| BINARY/VARBINARY  | String (Base64编码)         | 二进制数据，转换为Base64字符串      |
| UNIQUEIDENTIFIER  | String                     | GUID/UUID                      |
| XML               | String                     | XML数据                         |

## 错误处理

该库使用`anyhow`进行错误处理，当遇到不支持的列数据类型或反序列化失败时会返回适当的错误。

## 许可证

MIT

## 贡献

欢迎提交问题和拉取请求！

---

# Tiberius Row (English Version)

`tiberius_row` is a Rust procedural macro library designed to simplify the conversion of row data from the Tiberius SQL Server client to Rust structures.

## Features

- Provides a `Row_Ext` derive macro that automatically implements conversion from Tiberius row data to Rust structures
- Supports various SQL data types including integers, floating-point numbers, strings, date-time, etc.
- Uses serde for data deserialization, providing type-safe data access
- Automatically handles NULL values
- Provides friendly error handling

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
tiberius_row = "0.1.0"
base64 = "0.22.1"
tiberius = { version = "0.12.3", features = ["chrono", "rust_decimal", "time"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.140"
anyhow = "1.0"
```

## Usage

1. First, derive the necessary traits for your structure:

```rust
use serde::Deserialize;
use tiberius_row::Row_Ext;

#[derive(Deserialize, Row_Ext)]
struct User {
    id: i32,
    name: String,
    email: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}
```

2. Then, you can convert directly from a Tiberius Row to your structure:

```rust
use anyhow::Result;
use tiberius::Client;

async fn get_user(client: &mut Client<Connection>, user_id: i32) -> Result<User> {
    let query = "SELECT id, name, email, created_at FROM users WHERE id = @P1";
    let stream = client.query(query, &[&user_id]).await?;
    let row = stream.into_row().await?.expect("No user found");
    
    // Use the derived from_row method
    let user = User::from_row(row)?;
    Ok(user)
}
```

## Supported Data Types

The following table shows the correspondence between SQL Server data types and Rust types:

| SQL Server Data Type | Rust Type                    | Description                    |
|---------------------|------------------------------|--------------------------------|
| INT                 | i32                          | 32-bit integer                 |
| BIGINT              | i64 or i128                  | 64-bit integer                 |
| SMALLINT            | i16                          | 16-bit integer                 |
| TINYINT             | u8                           | 8-bit unsigned integer         |
| BIT                 | bool                         | Boolean value                  |
| DECIMAL/NUMERIC     | tiberius::numeric::Decimal   | High-precision numeric value   |
| FLOAT               | f64                          | 64-bit floating-point number   |
| REAL                | f32                          | 32-bit floating-point number   |
| DATE                | chrono::NaiveDate            | Date                           |
| DATETIME            | chrono::NaiveDateTime        | Date and time                  |
| DATETIME2           | chrono::NaiveDateTime        | High-precision date and time   |
| TIME                | chrono::NaiveTime            | Time                           |
| CHAR/VARCHAR        | String                       | Fixed/variable-length string   |
| NCHAR/NVARCHAR      | String                       | Unicode fixed/variable-length string |
| TEXT/NTEXT          | String                       | Long text                      |
| BINARY/VARBINARY    | String (Base64 encoded)      | Binary data, converted to Base64 string |
| UNIQUEIDENTIFIER    | String                       | GUID/UUID                      |
| XML                 | String                       | XML data                       |

## Error Handling

This library uses `anyhow` for error handling, returning appropriate errors when encountering unsupported column data types or deserialization failures.

## License

MIT

## Contributions

Issues and pull requests are welcome! 