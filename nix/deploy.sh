#!/usr/bin/env nix-shell
#! nix-shell -p bash

echo $DOCKER_PASSWORD | docker login -u $DOCKER_USERNAME --password-stdin

docker load -i result
docker tag xena/printerfacts:latest xena/printerfacts:$GITHUB_SHA
docker push xena/printerfacts:$GITHUB_SHA
dhall-to-yaml-ng --omit-empty --file printerfacts.dhall > $GITHUB_WORKSPACE/deploy.yml
doctl kubernetes cluster kubeconfig show kubermemes > $GITHUB_WORKSPACE/.kubeconfig
kubectl --kubeconfig=/github/workspace/.kubeconfig apply -n apps -f /github/workspace/deploy.yml

