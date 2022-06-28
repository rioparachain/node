#!/bin/sh
echo '#!/bin/sh'
find cumulus node pallets runtime -type l -exec sh -c "echo ln -sf \`readlink {}\` {}" \; \
    | grep -v "Makefile" | grep -v ".rustfmt"
