#!/bin/bash

echo "Rebuilding and running..."
bash ./remove_images.sh
bash ./build_images.sh
bash ./run_containers.sh
echo "Done rebuilding and running."
