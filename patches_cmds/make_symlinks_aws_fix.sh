#!/bin/sh
echo '#!/bin/sh'
find cumulus frontier node pallets proc-macro primitives runtime -type l -exec sh -c "echo ln -sf \`readlink {}\` {}" \; \
    | grep -v "Makefile" | grep -v ".rustfmt"
