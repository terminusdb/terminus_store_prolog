docker build -t "terminusdb/terminus_store_prolog:${TRAVIS_TAG}" .
echo "$DOCKER_PASS" | docker login -u terminusdb --password-stdin
docker push "terminusdb/terminus_store_prolog:${TRAVIS_TAG}"
docker run -it --rm terminusdb/terminus_store_prolog:${TRAVIS_TAG} swipl -g "pack_remove(terminus_store_prolog), pack_install('https://github.com/terminusdb/terminus_store_prolog/archive/${TRAVIS_TAG}.zip', [interactive=false])." -g halt
