rootDir := justfile_directory()

start-server:
    sudo k3s server

deploy:
    k8s/deploy.sh

format:
    @cd "{{rootDir}}/api" && just format
    @cd "{{rootDir}}/web" && just format
    @cd "{{rootDir}}/markdown-to-pdf" && just format
