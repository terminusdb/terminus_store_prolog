docker build -t "terminusdb/terminus_store_prolog:${TRAVIS_TAG}" .
echo "$DOCKER_PASS" | docker login -u terminusdb --password-stdin
docker push "terminusdb/terminus_store_prolog:${TRAVIS_TAG}"
