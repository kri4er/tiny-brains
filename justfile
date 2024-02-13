alias b := build-debug
alias br := build-release
alias d := deploy
alias w := watch
alias bad := build-and-deploy

build-debug:
    cargo build

build-release:
    #!/usr/bin/env zsh
    source ../.venv/bin/activate
    cargo lambda build --release --arm64

watch:
    cargo watch -w styles -w templates -w src -x run

deploy:
    #!/usr/bin/env zsh
    source ../.venv/bin/activate
    (
    cargo lambda deploy --include templates --include styles --include assets --include tailwind.config.js \
    --iam-role arn:aws:iam::${AWS_ACCOUNT_ID}:role/personal-blog-v1-lambda-role --enable-function-url
    )

build-and-deploy:
    just build-release | just deploy
