#!/usr/bin/env bash

rm -r docs
cp -r dist docs
cp .nojekyll docs/
cp 404.html docs/
git add -A && git commit -m "-" && git push
