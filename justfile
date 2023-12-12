rootDir := justfile_directory()

start-server:
    sudo k3s server

deploy *ARGS:
    @cd "{{rootDir}}/manifests" && tilt {{ARGS}}

deploy-up *ARGS:
    @cd "{{rootDir}}/manifests" && tilt up {{ARGS}}

deploy-down *ARGS:
    @cd "{{rootDir}}/manifests" && tilt down {{ARGS}}

build:
    @cd "{{rootDir}}/api" && just build
    @cd "{{rootDir}}/converter" && just build
    @cd "{{rootDir}}/web" && just build

build-component path:
    @{{rootDir}}/tools/build-component.sh "{{path}}"

format:
    tools/format.sh
