#!/bin/sh
echo '#!/bin/sh'
find cumulus node pallets proc-macro primitives runtime -type l -exec sh -c "echo ln -sTf \`readlink {}\` {}" \; \
    | grep -v "Makefile" | grep -v ".rustfmt"
