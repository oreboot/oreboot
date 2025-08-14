#!/bin/sh

# # extract '# Desription' section from root README.md
csplit -f readme_ ../README.md '/^#/' '{1}' > /dev/null
# change headline to 'u-root' and fix relative links to point to GitHub
cat readme_* | sed \
  -e 's/oreboot README/\noreboot/' \
  -e 's/oreboot logo//' \
  -e 's/Documentation\///' \
  > README.md
rm readme_*

# fetch pandoc-uikit template
_TEMPLATE=pandoc-uikit-master
[ -d "$_TEMPLATE" ] ||
  curl -L https://github.com/diversen/pandoc-uikit/archive/master.tar.gz | tar -xzf -

# cat it all and pipe into pandoc :)
cat README.md getting-started.md | pandoc --metadata title="oreboot" --toc \
  -o index.html --template="$_TEMPLATE"/template.html -
