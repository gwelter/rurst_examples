struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &'a str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

//don't neet an explicit lifetime here because the compiler can infer it since the function have only one parameter
fn take_and_return_content(content: &str) -> &str {
    content
}

fn take_and_return_content2<'a, 'b>(content: &'a str, _extra: &'b str) -> &'a str {
    content
}

pub fn run_example() {
    let mut tweet = Tweet {
        content: "initial content",
    };
    let old_content = tweet.replace_content("new content");
    println!("Old content: {}", old_content);
    println!("New content: {}", tweet.content);
    println!(
        "Returned content: {}",
        take_and_return_content(tweet.content)
    );
    println!(
        "Returned content: {}",
        take_and_return_content2(tweet.content, "extra")
    );
}
