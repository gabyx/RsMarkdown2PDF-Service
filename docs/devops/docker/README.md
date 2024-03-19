# Nesting Containers with Podman

## Running as Root

Run

```shell
just run "original" "root"
```

to see that we can build `Containerfile` (`podman` engine) then execute the
built container and inside call `./run.sh` again which recursively nests
containers. You can also use the `alpine` image with:

```shell
just run "custom" "root"
```

Its just too cool that this works? 🤣

## Running as Non-Root

```shell
just run "custom" "podman"
```
