# Nesting Containers with Podman

Run

```shell
podman volume create podman-vol
podman build -f Containerfile podman-test .

# Start the first container.
podman run --privileged --rm \
    -v "podman-vol:/var/lib/containers" \
    -it podman-test ./build.sh
```

to see that we can build `Containerfile` (`podman` engine) then execute the
built container and inside call `./build.sh` again which recursively nests
containers. Its just too cool that this works? 🤣

Up to 3 nested containers it works until then something crashes when setting up
new user namespaces. How to resolve it, no idea.
