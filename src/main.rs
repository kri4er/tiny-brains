mod brains;

use axum::{
    response::Html,
    routing::{get, post},
    http::StatusCode,
    Router,
    extract::Form,
};
use std::sync::Mutex;
use lazy_static::lazy_static;
use askama::Template;
use tower_http::services::ServeFile;

use brains::{Brain, get_brain_router};

lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref BRAINS: Mutex<Vec<Brain>> = Mutex::new(vec![]);
}

fn static_assets() -> Router {
    Router::new()
        .route_service("/logo.png", ServeFile::new("assets/logo.png"))
        .route_service("/styles/index.css", ServeFile::new("styles/index.css"))
}

fn init_brains() {
    let mut brains = BRAINS.lock().unwrap();
    /*
    brains.push(Brain {
        brain_url: "/brains/basic_set_up".into(),
        logo_url: "https://www.adobe.com/express/create/logo/media_1c11583be022dbe61efe8a253626810bc79b2f5ae.png?width=750&format=png&optimize=medium".into(),
        description: "Basic set up: rust + Askama + htmx + live updates using cargo watch".into() 
    });
    */
    brains.push(Brain {
        brain_url: "/brains/hostOnLambda".into(),
        logo_url: "https://banner2.cleanpng.com/20180330/orq/kisspng-amazon-com-aws-lambda-amazon-web-services-serverle-half-life-5abe90ed7fb630.4332638115224383815231.jpg".into(),
        description: "Deployment: Deploy axum service on AWS lambda".into(),
        md_file: None,
    });
    brains.push(Brain {
        brain_url: "/brains/makeFileAndJustFile".into(),
        logo_url: "https://interrupt.memfault.com/img/gnu-make-guidelines/gnu-make.png".into(),
        description: "Justfile - a simpler makefile".into(),
        md_file: None,
    });
    brains.push(Brain {
        brain_url: "/brains/serverSideMarkdown".into(),
        logo_url: "https://www.markdownguide.org/assets/images/markdown-mark-white.svg".into(),
        description: "Markdown support".into(),
        md_file: Some("".into()),
    });
    brains.push(Brain {
        brain_url: "/brains/enableDiagramViewer".into(),
        logo_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/3/3e/Diagrams.net_Logo.svg/2048px-Diagrams.net_Logo.svg.png".into(),
        description: "Diagram viewer".into(),
        md_file: Some("".into()),
    });
    brains.push(Brain {
        brain_url: "/brains/testStorySinglePageApp".into(),
        logo_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/3/3e/Diagrams.net_Logo.svg/2048px-Diagrams.net_Logo.svg.png".into(),
        description: "Unit Test Story: Single Page Application".into(),
        md_file: Some("".into()),
    });
}

fn root_router() -> Router {
    Router::new()
        .route("/", get(index_page))
        .route("/index", get(index_page))
        .route("/health", get(health))
        .route("/list", get(render_tasks))
        .route("/recently_added", get(recently_added))
        .route("/add", post(add_task))
        .route("/update", post(add_task_htmx))
        .nest("/assets", static_assets())
        .nest("/brains", get_brain_router())
        .fallback(handle_404)

}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    init_brains();

    let app = root_router();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage;

#[derive(Template)]
#[template(path = "todos.html")]
struct TodoListTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "recently_added.html")]
struct RecentlyAddedBrains<'a> {
    brains: &'a Vec<Brain>,
}

#[derive(Template)]
#[template(path = "pageNotFound.html")]
struct NotFound {
}

async fn handle_404() ->Html<String> {
    let template = NotFound {};
    Html(template.render().unwrap())
}

async fn index_page() -> Html<String> {
    let template = IndexPage {};
    Html(template.render().unwrap())
}

async fn render_tasks() -> Html<String> {
    let tasks = TASKS.lock().unwrap();
    let template = TodoListTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn recently_added() -> Html<String> {
    let brains = BRAINS.lock().unwrap();
    let template = RecentlyAddedBrains { brains: &brains };
    Html(template.render().unwrap())
}

async fn add_task(Form(input): Form<AddTask>) -> StatusCode {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    StatusCode::SEE_OTHER
}

async fn add_task_htmx(Form(input): Form<AddTask>) -> Html<String> {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    let template = TodoListTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

#[derive(serde::Deserialize)]
struct AddTask {
    task: String,
}

