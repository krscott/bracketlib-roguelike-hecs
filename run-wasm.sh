#!bash

# exit when any command fails
set -e
trap 'echo ''; echo Error at $(basename "$0"):${LINENO}: $BASH_COMMAND' ERR

# set working directory to this script's directory
cd "${0%/*}"

pyhttpserver ()
{
    local port="${1:-8080}";
    if [[ $(python -V) == "Python 2"* ]]; then
        python -m SimpleHTTPServer "$port";
    else
        python -m http.server "$port";
    fi
}

./build-wasm.sh

cd dist/wasm

pyhttpserver