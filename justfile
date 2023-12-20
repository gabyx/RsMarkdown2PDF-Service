root_dir := justfile_directory()
parallel := "false" # Run tasks over components in parallel.

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
deploy *ARGS:
    @cd "{{root_dir}}/manifests" && tilt {{ARGS}}

deploy-up *ARGS:
    @cd "{{root_dir}}/manifests" && tilt up {{ARGS}}

deploy-down *ARGS:
    @cd "{{root_dir}}/manifests" && tilt down {{ARGS}}
    # In case anything keeps hanging.
    @kubectl delete all --all --namespace md2pdf

# Building the components.
###############################################################################
build regex=".*":
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" "{{parallel}}" "{{regex}}" build

build-image regex=".*":
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" "{{parallel}}" "{{regex}}" build-image

# Component functionality.
###############################################################################
component component task:
    @{{root_dir}}/tools/run-component-task.sh "{{component}}" "{{task}}"

list-components regex=".*":
    @cd "{{root_dir}}" && find ./components -mindepth 1 -maxdepth 1 \
        -type d -regextype "posix-extended" -regex "./components/{{regex}}"

# Formatting.
###############################################################################
format regex=".*":
       tools/format.sh "{{parallel}}" "{{regex}}"
