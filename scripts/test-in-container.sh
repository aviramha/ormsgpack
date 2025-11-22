#!/bin/sh

set -eu

image=$1
platform=$2
workspace=$3

command=$(cat <<'EOF'
    if command -v apk > /dev/null; then
        apk add --no-cache build-base curl
    fi
    curl --proto =https --tlsv1.2 -LsSf "https://astral.sh/uv/$UV_VERSION/install.sh" | sh
    export PATH=$PATH:$HOME/.local/bin
    uv sync --frozen --no-install-project
    uv pip install ormsgpack --no-index -f target/wheels
    uv run --no-sync pytest
EOF
)

docker \
    run \
    --rm \
    -e UV_CACHE_DIR=/work/.uv_cache \
    -e UV_NO_GROUP \
    -e UV_VERSION \
    -v "$workspace":/work \
    -w /work \
    --platform "$platform" \
    "$image" \
    sh -e -c "$command"
