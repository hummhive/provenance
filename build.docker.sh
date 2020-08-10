docker build -f Dockerfile.binaries -t humm/provenance.binaries .

docker run -v "$PWD/dist":/dist humm/provenance.binaries
