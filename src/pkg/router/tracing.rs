use tower_http::trace::{DefaultMakeSpan, HttpMakeClassifier, TraceLayer};

pub fn tracing() -> TraceLayer<HttpMakeClassifier> {
    TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true))
}
