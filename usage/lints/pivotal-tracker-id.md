# Pivotal Tracker Lints

## Setup

You need the binaries on your path and them installed into a git
repository.

``` bash
set -euo pipefail

unset GIT_AUTHORS_EXEC
unset GIT_AUTHORS_CONFIG
PATH="$PWD/target/release/:$PATH"
cd "$(mktemp -d)"

git init .

rm -rf .git/hooks/*
ln -s "$(command -v pb-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v pb-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v pb-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

We need a author configuration that is current

``` bash

echo "---
bt:
    name: Billie Thompson
    email: billie@example.com
    signingkey: 0A46826A
se:
    name: Someone Else
    email: someone@example.com
ae:
    name: Anyone Else
    email: anyone@example.com" > authors.yml

git-authors -c authors.yml se
echo "git-authors.yml" > .gitignore
git add .
git commit -m "Add a git ignore"

```

## Default setting

Without enabling the lint can make commits without an ID

``` bash
echo "Hello, world!" > demo.txt
git add .
git commit -m "Enabling a lint"
```

## Enabling this specific lint

To enable it run

``` bash
pb-git-hooks lint enable pivotal-tracker-id-missing
```

After enabling the lint you can't commit without a issue id

``` bash
echo "Hello, world!" > demo.txt
git add demo.txt

if git commit -m "I am not made" ; then
    echo "This never happens" 
    exit 1
fi
```

But you can with one

``` bash
echo "Goodbye, world!" > demo.txt
git add demo.txt

git commit -m "Enabled the lint

[#87654321]
"
```

## Disabling this specific lint

You can also disable the lint

``` bash
pb-git-hooks lint disable pivotal-tracker-id-missing
```

You'll be able to commit without an ID

``` bash
echo "Hello again, world!" > demo.txt
git add demo.txt
git commit -m "Disabling the lint"
```