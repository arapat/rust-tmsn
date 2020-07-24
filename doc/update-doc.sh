cargo doc --no-deps
echo "Continue?"
read
git checkout gh-pages
rm -rf src tmsn implementors
mv target/doc/* .
git add .
git commit -m "updates"
git push
git checkout master
