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
tiberius = { version = "0.12.3", features = ["chrono"] }
serde = { version = "1.0", features = ["derive"] }
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

- 整数类型：`i16`, `i32`, `i64`, `u8`
- 浮点类型：`f32`, `f64`
- 字符串：`String`
- 布尔值：`bool`（通过`Bit`类型）
- 日期时间：`chrono::DateTime<chrono::Utc>`
- 数值类型：`Numeric`

## 错误处理

该库使用`anyhow`进行错误处理，当遇到不支持的列数据类型或反序列化失败时会返回适当的错误。

## 许可证

MIT

## 贡献

欢迎提交问题和拉取请求！ 