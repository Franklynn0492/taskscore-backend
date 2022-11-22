#!/bin/bash
target="${1:-debug}"

echo "Rebuilding and running..."
bash ./remove_images.sh
bash ./remove_network.sh
bash ./create_network.sh
bash ./build_images.sh ${target}
bash ./run_containers.sh
echo "Done rebuilding and running."
