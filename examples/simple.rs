use railpack_rs::{generate_build_plan, Config};

fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../railway-tests/rltest1".to_string());

    let config = Config::new(directory)
        .with_verbose(true)
        .with_railpack_version("dev");

    let result = generate_build_plan(&config).unwrap();
    println!("result: {:#?}", result);

    if let Some(plan) = result.plan() {
        println!("{:#?}", plan);
    } else {
        println!("No plan was generated or deserialization failed");
    }
}
