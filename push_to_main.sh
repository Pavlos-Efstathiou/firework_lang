CHANGES=$(cat changes.md)
VERSION=$1

printf "\n\n# v${VERSION}\n### Changes\n${CHANGES}" | tee -a CHANGELOG.MD
# Requires cargo-edit
cargo set-version $VERSION
git add -A
git commit -m "v${VERSION}"
git push origin main
cargo publish
rm changes.md