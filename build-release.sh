#!/usr/bin/env bash

develop && trunk build --release

rm -r docs
cp -r dist docs
cp .nojekyll docs/
cp 404.html docs/
git add -A && git commit -m "-" && git push

cd dist/
tar -cvz * > site.tar.gz
mv site.tar.gz ../site.tar.gz 
cd ..
hut pages publish -d abhinandh.srht.site site.tar.gz

