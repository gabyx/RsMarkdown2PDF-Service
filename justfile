root_dir := justfile_directory()
parallel := "false" # Run tasks over components in parallel.

start-server:
    sudo k3s server

# Deploying the components.
deploy *ARGS:
    @cd "{{root_dir}}/manifests" && tilt {{ARGS}}

deploy-up *ARGS:
    @cd "{{root_dir}}/manifests" && tilt up {{ARGS}}

deploy-down *ARGS:
    @cd "{{root_dir}}/manifests" && tilt down {{ARGS}}


# Building the components.
build regex=".*":
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" "{{parallel}}" "{{regex}}" build

build-image regex=".*":
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" "{{parallel}}" "{{regex}}" build-image

# Formatting.
format regex=".*":
       tools/format.sh "{{parallel}}" "{{regex}}"

# Private stuff not for direct execution.
[private]
component component task:
    @{{root_dir}}/tools/run-component-task.sh "{{component}}" "{{task}}"

list-components regex=".*":
    @cd "{{root_dir}}" && find ./components -mindepth 1 -maxdepth 1 \
        -type d -regextype "posix-extended" -regex "./components/{{regex}}"
