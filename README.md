# Markdown to PDF Service.

This is a demo project to showcase a small microservice architecture by exposing

- a simple [`web` frontend service](web/src/main.rs) which enables the user to
  upload a Markdown file and let it convert in
- the backend [`markdown-to-pdf` service](markdown-to-pdf/src/main.rs) which is
- controlled by the backend [`api` service](api/src/main.rs).

## Run Instructions

The easiest way to run this is using `tilt` and on a working Kubernetes cluster,
such as `k3s`.

With `tilt` installed and a Kubernetes cluster running:

```shell
cd k8s
./deploy.sh
```

## Local Development Loop for Fast Feedback

All `rust` projects can be run locally using `cargo run`.

The `.env` file configures the tools to connect to the services running in the
Kubernetes cluster.

## Development Loop using `tilt` (Kubernetes)

The tool `tilt` will run all services and build all docker containers to the
`ttl.sh` registry (ephemeral images).

It will watch for changes to any files (including the [service manifests](k8s)
and redeploy the services, configuration maps as far as possible.

## Development

### Githooks

You can install Githooks by running the manual
[install here](https://github.com/gabyx/Githooks#quick-secure) and then running:

```shell
git hooks install
git hooks config enable-containerized-hooks --globally --set
```

in this repository. To show all running hooks run `git hooks list`. To disable
running hooks use either

- `GITHOOKS_DISABLE=1 <your-cmd>` or
- `git commit --no-verify ...` or
- `git hooks uninstall` to completely remove the hook run wrappers from
  `.git/hooks`.
