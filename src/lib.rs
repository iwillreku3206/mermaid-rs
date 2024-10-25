use std::{error::Error, str::EscapeUnicode, sync::Arc};

use headless_chrome::{Browser, Tab};

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
            .evaluate(&format!("render('{}')", escape_string::escape(input)), true)?;
        let string = data.value.unwrap_or_default().to_string();
        let slice = string.trim_matches('"');
        Ok(slice.to_string())
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
