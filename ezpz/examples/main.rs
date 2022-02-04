#[ezpz::pz]
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
    let api_url = config["api_url"].as_str().unwrap();

    // Strongly-typed JSON

    #[derive(Deserialize)]
    struct ApiResponse {
        items: Vec<ApiResponseQuestion>,
    }

    #[derive(Deserialize)]
    struct ApiResponseQuestion {
        tags: Vec<String>,
        title: String,
        link: String,
        score: i64,
    }

    let response = fetch(api_url)?;
    let response = response.text()?;
    let response: ApiResponse = json::from_str(&response)?;

    let question = &response.items[0];
    let title = &question.title;
    let tag = &question.tags[0];
    let score = &question.score;
    let link = &question.link;

    println!("+{score} [{tag}] {title} ({link})");
}
