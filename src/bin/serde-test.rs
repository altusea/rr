use serde::{Deserialize, Serialize};
// 定义一个结构体来表示用户信息
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // 创建一个 User 实例
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    };

    // 序列化 User 实例到 JSON 字符串
    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized User: {}", serialized);

    // 反序列化 JSON 字符串到 User 实例
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized User: {:?}", deserialized);
}
