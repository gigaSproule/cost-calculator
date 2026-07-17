FROM gitpod/workspace-full-vnc@sha256:06f30a41188b98dc23003471908c9d6a8d9a377a5e9cd3f95e0d61f2ce401e02
RUN sudo apt-get update && \
    sudo apt-get install -y libgtk-4-dev libadwaita-1-dev && \
    sudo apt-get clean && sudo rm -rf /var/cache/apt/* && sudo rm -rf /var/lib/apt/lists/* && sudo rm -rf /tmp/*
