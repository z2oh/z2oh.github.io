mod handlebars;
pub use handlebars::HandlebarsTransformer;
pub trait Transformer {
    fn transform(&self, input: String) -> anyhow::Result<String>;
}

