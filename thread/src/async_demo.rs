use trpl::Html;

pub async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let text = response.text().await;

    Html::parse(&text)
        .select_first("title")
        .map(|element| element.inner_html())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_demo() {
        trpl::run(async {
            let url = "https://www.rust-lang.org";
            let title = page_title(url).await;
            if let Some(title) = title {
                print!("Title of {} is {}", url, title)
            }
        });
    }
}
