#!/bin/bash
target="${1:-debug}"
host="${2:-localhost}"
port="${3:-8080}"

echo "Rebuilding and running..."
bash ./remove_images.sh
bash ./remove_network.sh
bash ./create_network.sh
bash ./build_images.sh ${target}
bash ./run_containers.sh ${host} ${port}
echo "Done rebuilding and running."
