use web_view::Content::Html;
use web_view::WebViewBuilder;

pub fn create_window(text: &str, title: &str, debug: bool) {
    WebViewBuilder::new()
        .title(title)
        .content(Html(text))
        .size(800, 600)
        .debug(debug)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .run()
        .unwrap();
}
