use std::{error::Error, sync::Arc};

use escape_string::escape;
use headless_chrome::{Browser, Tab};
use unescape::unescape;

#[derive(Clone)]
pub struct Mermaid {
    _browser: Browser,
    tab: Arc<Tab>,
}

impl Mermaid {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::default()?;
        let mermaid_js = include_str!("../payload/mermaid.min.js");
        let html_payload = include_str!("../payload/index.html");

        let tab = browser.new_tab()?;
        tab.navigate_to(&format!("data:text/html;charset=utf-8,{}", html_payload))?;
        tab.evaluate(mermaid_js, false)?;

        Ok(Self {
            _browser: browser,
            tab,
        })
    }

    pub fn render(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let data = self
            .tab
            .evaluate(&format!("render('{}')", escape(input)), true)?;
        let string = data.value.unwrap_or_default().to_string();
        let slice = unescape(string.trim_matches('"')).unwrap_or_default();
        Ok(slice.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mermaid_instance_without_crashing() {
        let mermaid = Mermaid::new();
        assert!(mermaid.is_ok());
    }

    #[test]
    fn render_mermaid() {
        let mermaid = Mermaid::new().unwrap();
        let rendered = mermaid.render("graph TB\na-->b");
        assert!(rendered.is_ok());
        // TODO: Perform visual image comparison
        assert!(rendered.unwrap().starts_with("<svg"));
    }
}
