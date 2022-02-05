import::all!();

#[ze::ze]
fn main() -> eyre::Result<()> {
    // Dynamic JSON data

    let original: i64 = 2;
    let wrapped = Json::from(original);
    let serialized = wrapped.to_string();
    let deserialized: Json = serialized.parse()?;
    let unwrapped = deserialized.as_i64().unwrap();

    assert_eq!(original, unwrapped);
    assert_eq!(wrapped, deserialized);
    assert_eq!(serialized, "2");

    // Inline JSON expressions
    let config = json!({
        "api_url": "https://api.stackexchange.com/2.3/questions?pagesize=1&order=desc&sort=month&site=stackoverflow&filter=m9wl3qGIs-zpx*"
    });
}
