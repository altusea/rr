fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = args
        .iter()
        .map(|arg| {
            if let Some(var_name) = arg.strip_prefix('$') {
                std::env::var(var_name).unwrap_or_else(|_| arg.clone())
            } else {
                arg.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}
