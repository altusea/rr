use anyhow::{Context, Result};
use rand::Rng;

fn might_fail(input: i32) -> Result<i32> {
    let mut rng = rand::thread_rng(); // 获取线程局部的随机数生成器
    let n: f64 = rng.gen_range(0.0..1.0); // 生成一个 0 到 1 之间的浮点数
    if n < 0.5 {
        Err(anyhow::anyhow!("Input cannot be negative"))
    } else {
        Ok(input)
    }
}

fn process_data(data: i32) -> Result<()> {
    let result = might_fail(data)
        .with_context(|| format!("Failed to process data: {}", data))?;

    // 假设我们在这里进行了一些处理
    println!("Processed data: {}", result);
    Ok(())
}

fn main() {
    let data = 10;

    match process_data(data) {
        Ok(_) => println!("Data processed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    }
}