rootDir := justfile_directory()

start-server:
    sudo k3s server

deploy:
    k8s/deploy.sh

build:
    @cd "{{rootDir}}/api" && just build
    @cd "{{rootDir}}/markdown-to-pdf" && just build
    @cd "{{rootDir}}/web" && just build

build-component path:
    @{{rootDir}}/tools/build-component.sh "{{path}}"

format:
    @cd "{{rootDir}}/api" && just format
    @cd "{{rootDir}}/web" && just format
    @cd "{{rootDir}}/markdown-to-pdf" && just format
