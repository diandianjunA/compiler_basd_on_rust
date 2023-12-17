pub mod span;

pub struct SourceText {
    text: String,
}

impl SourceText {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn line_index(&self, position: usize) -> usize {
        self.text[..=position].lines().count() - 1
    }

    pub fn get_line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap()
    }

    pub fn line_start(&self, index: usize) -> usize {
        self.text.lines().take(index).map(|line| line.len() + 1).sum()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_text_test(){
        let text = SourceText::new("let x = 5;\nlet x=6\nlet x=7".to_string());
        println!("{}",text.line_start(2));
    }
}