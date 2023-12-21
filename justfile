set positional-arguments
set shell := ["bash", "-cue"]
root_dir := justfile_directory()
parallel := "true" # Run tasks over components in parallel.
default_regex := ".*"

# Administrative stuff.
###############################################################################
start-server:
    sudo k3s server

# Start a docker registry locally which can be used with
# tils to upload images for k3s.
start-docker-registry:
    docker run -d -p 5000:5000 --name registry registry:latest

# Deploying the components.
###############################################################################
deploy *args:
    @cd {{root_dir}}/manifests" && tilt "$@"

deploy-up *args:
    @cd {{root_dir}}/manifests" && tilt up "$@"

deploy-down *args:
    @cd {{root_dir}}/manifests" && tilt down "$@"
    # In case anything keeps hanging.
    @kubectl delete all --all --namespace md2pdf

# Building the components.
###############################################################################
build *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{default_regex}}" build "${@:1}"
build-selection regex *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{regex}}" build "${@:2}"

build-image *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{default_regex}}" build-image "${@:1}"
build-image-selection regex *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{regex}}" build-image "${@:2}"

# Component functionality.
###############################################################################
component component task *args:
    @"{{root_dir}}/tools/run-component-task.sh" "{{component}}" "{{task}}" "${@:3}"

list-components regex=".*":
    @cd "{{root_dir}}" && find ./components -mindepth 1 -maxdepth 1 \
        -type d -regextype "posix-extended" -regex "./components/{{regex}}"

# Formatting.
###############################################################################
format regex=".*":
       tools/format.sh "{{parallel}}" "{{regex}}"
