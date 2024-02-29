use std::{fs, path::Path};

use askama::Template;
use axum::{response::Html, routing::get, Router};
use pulldown_cmark::{html, Options, Parser};


pub struct Brain {
    pub brain_url: String,
    pub logo_url: String,
    pub description: String,
    pub md_file: Option<String>,
}

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

pub fn get_brain_router() -> Router {
    Router::new()
        .route("/searchPage", get(search_page))
        .route("/hostOnLambda", get(host_on_lambda_page))
        .route("/makeFileAndJustFile", get(make_file_and_just_file))
        .route("/serverSideMarkdown", get(server_side_markdown_rendering))
        .route("/enableDiagramViewer", get(enable_diagram_rendering))
        .route("/testStorySinglePageApp", get(single_page_application_using_askama_and_htmx))
}


#[derive(Template)]
#[template(path = "brains/searchPage.html")]
struct SearchPage;

#[derive(Template)]
#[template(path = "brains/hostOnLambdaPage.html")]
struct HostOnLambdaPage;

#[derive(Template)]
#[template(path = "brains/makeFileAndJustfile.html")]
struct MakeFileAndJustFile;

#[derive(Template)]
#[template(path = "brains/serverSideMarkdown.html")]
struct ServerSideMarkdown<'a> {
    md_generated_html: &'a String,
}

#[derive(Template)]
#[template(path = "brains/enableDiagramRendering.html")]
struct EnableDiagramRendering<'a> {
    md_generated_html: &'a String,
}

#[derive(Template)]
#[template(path = "brains/template_fragments/article_template.html")]
struct UnitTestStory {
    caption: String,
    md_sections: Vec<String>,
}

async fn search_page() -> Html<String> {
    let template = SearchPage {};
    Html(template.render().unwrap())
}

async fn host_on_lambda_page() -> Html<String> {
    let template = HostOnLambdaPage {};
    Html(template.render().unwrap())
}

async fn make_file_and_just_file() -> Html<String> {
    let template = MakeFileAndJustFile {};
    Html(template.render().unwrap())
}

async fn server_side_markdown_rendering() -> Html<String> {
    let file_content = read_md_file(Path::new("templates/brains/md-articles/support-markdown-files.md".into()))
        .expect("Failed to convert md file to html");
    let generated_html = md_to_html(file_content);
        
    let template = ServerSideMarkdown { md_generated_html: &generated_html };
    Html(template.render().unwrap())
}

async fn enable_diagram_rendering() -> Html<String> {
    let file_content = read_md_file(Path::new("templates/brains/md-articles/enable-draw-io-integration.md".into()))
        .expect("Failed to convert md file to html");
    let generated_html = md_to_html(file_content);
        
    let template = EnableDiagramRendering { md_generated_html: &generated_html };
    Html(template.render().unwrap())
}

fn section(path: &Path)-> String {
    let file_content = read_md_file(path.into())
        .expect("Failed to convert md file to html");
    md_to_html(file_content)
}

async fn single_page_application_using_askama_and_htmx() -> Html<String> {
    let md_sections = vec![
            section(Path::new("templates/brains/md-articles/single-page-app-experience-with-htmx.md")),
        ];
    let template = UnitTestStory { 
        caption: "Unit test story.".into(),
        md_sections
    };
    Html(template.render().unwrap())
}

