### Markdown support { #article_header .font-bold }
 Right now, when writing a post, I have to edit actual html page and make sure it is correct and look good.
 Tedious and annoying process. An answer here is **Markdown**

There are multiple approaches we can follow:
1. Send markdown to client side and use some client side markdown to html renderer, e.g: [md-tag](https://www.cssscript.com/markdown-to-html-tag/).
2. Server side markdown rendering.
3. I write markdown locally, use CLI tool to render html and add it as content of page.

Though Option 3 would be preferred one for simple blog, we plan to enable certain CRM capabilities, in future. Leading us to position of server side rendering.

### Lets begin
##### We have chosen Option 2. Server side markdown rendering
Quick search in the internet, fetched us a few options used in the community:
1. [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)
2. [comrak](https://github.com/kivikakk/comrak)
3. [markdown-rs](https://github.com/wooorm/markdown-rs)

Without diving deep, I can't figure out any serious difference between them, I will follow thousand monkeys here and pick most used in the community:
[pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark).



```rust
fn read_md_file(file_path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let md = fs::read_to_string(file_path.to_str().unwrap())?;
    Ok(md)
}

fn md_to_html(markdown_input: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
//Now we use the template:

async fn server_side_markdown_rendering() -> Html<String> {
    let file_content = read_md_file(Path::new("templates/brains/md-articles/support-markdown-files.md".into()))
        .expect("Failed to convert md file to html");
    let generated_html = md_to_html(file_content);
        
    let template = ServerSideMarkdown { md_generated_html: &generated_html };
    Html(template.render().unwrap())
}

```

And we add askama template with safe function - telling askama, there is no need to escape characters:
```html
{{ md_generated_html|safe }}
```

### Styling

 Great, we have html generated as article content. But wait? Where are styles? In my blog I'm using tailwindcss.
But markdown to html doesn't append any styles. Sadly CSS doesn't allow class composition. Though there are tools,
to solve such problems. Tailwindcss comes with css pre-processor tool. As of now, we will just use css selectors
to styles, that will affect only certain parts of document and copy paste it from tailwindcss:

```css
#article_content h3 {
    font-weight: 700;
}
#article_content p {
    margin-bottom: 0.75rem/* 12px */;
    color: rgb(243 244 246);
}

#article_content a:link {
    font-weight: 700;
    color: rgb(147 197 253);
    text-decoration-line: underline;
}

#article_content a:visited {
    color: green;
}

#article_content a:hover {
    color: hotpink;
}

#article_content a:active {
    color: blue;
}

#article_content ol {
    list-style-type: decimal;
    margin-left: 1.75rem;
    margin-bottom: 0.75rem;
}

#article_content pre {
    border-radius: 0.25rem;
    padding: 0.75rem;
    background-color: rgb(17 24 39 / var(--tw-bg-opacity));
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}
```

That should be it, now writing articles is easy, with markdown only.
