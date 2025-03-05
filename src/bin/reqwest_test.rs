#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() {
    // 定义你想要请求的 URL
    let url = "https://www.baidu.com";

    // 发送 GET 请求
    match reqwest::get(url).await {
        Ok(response) => {
            // 检查响应状态码
            if response.status().is_success() {
                // 打印响应体
                println!("Response body:\n{}", response.text().await.unwrap());
            } else {
                // 如果响应码不是 2xx，打印错误信息
                eprintln!("Request failed with status: {}", response.status());
            }
        },
        Err(e) => {
            // 如果请求失败，打印错误信息
            eprintln!("Request failed: {}", e);
        },
    }
}
