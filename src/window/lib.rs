use webview::Content;
use webview::WebView;

pub fn create_window(text: &str, title: &str, debug: bool) {
    let view = WebView::new(
        title,
        Content::Html(text),
        800,
        600,
        true,
        debug,
    ).unwrap();

    view.join();
}
