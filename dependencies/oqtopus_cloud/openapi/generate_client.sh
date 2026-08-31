#!/bin/bash

set -e

podman run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
  -i /local/openapi_1.yaml \
  -g rust \
  -o /local/oqtopus_cloud \
  --additional-properties=packageName=oqtopus-client,library=reqwest,supportAsync=true,useSingleRequestParameter=false

cat >> oqtopus_cloud/Cargo.toml << 'EOF'

[lints.clippy]
needless_return = "allow"
empty_docs = "allow"
derivable_impls = "allow"
EOF

echo "generated + patched oqtopus_cloud/Cargo.toml"
