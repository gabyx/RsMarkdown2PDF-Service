# Markdown-to-PDF Service

**Note: This is currently in development and not feature complete.**

<!--toc:start-->

- [Markdown-to-PDF Service](#markdown-to-pdf-service)
  - [Project Structure](#project-structure)
  - [Architecture](#architecture)
  - [Requirements](#requirements)
  - [Quick Instructions](#quick-instructions)
    - [Deploy](#deploy)
    - [Shutdown](#shutdown)
  - [Locally Building Components](#locally-building-components)
  - [Deploying Components to the Cluster (Kubernetes)](#deploying-components-to-the-cluster-kubernetes)
  - [Development](#development) - [Debugging in Rust](#debugging-in-rust) -
  [Database Inspection](#database-inspection) - [Githooks](#githooks) -
  [CI/CD](#cicd) - [Gitlab](#gitlab) - [Testing API Calls](#testing-api-calls)
  <!--toc:end-->

This is a demo project to showcase a small microservice architecture by exposing

- a simple [`web` frontend service](web/src/main.rs) which enables the user to
  upload Markdown files (single or multiple) and let it convert by sending it to
- the [`api` service](api/src/main.rs) which will add a job into the `rabbitmq`
  queue, such that
- the [`converter` service](markdown-to-pdf/src/main.rs) eventually (when idle)
  pulls a job from the `rabbitmq` queue and stores the result in the database.

## Project Structure

- [`components`](components): All components making up this project.
  - [`api`](components/api): The `api` service receiving converter jobs from the
    `web` UI.
  - [`converter`](components/converter): The converter service which converts
    jobs taken out from the RabbitMQ queue `md2pdf.jobs`.
  - [`web`](components/web): The simple web service which presents the UI to the
    user to upload `markdown` files.
- [`manifests`](manifests): All kubernetes (`k8s`) manifests
- [`tools`](tools): Tools and scripts are located here.
  - [`.nvim`](.nvim): Nvim setup when entering this project. Needs plugin
    [`klen/nvim-config-local`](https://github.com/klen/nvim-config-local).
  - [`.githooks`](.githooks): Githooks setup, which also runs inside the CI.
  - [`.gitlab`](.gitlab): CI setup with Gitlab.

## Architecture

The architecture is described [in more details here](/docs/architecture.md).

## Requirements

On `NixOS` use the `flake.nix` by doing `nix develop --command zsh` inside the
root of the repo. This will setup an isolated development shell with all tools
installed.

On other systems you need the following essentials:

- [`just`](https://github.com/casey/just): A better `make` alternative.
- [`docker`](https://docs.docker.com/get-docker) or
  [`podman`](https://podman.io/docs/installation): Manage containers for
  virtualization and using the `kind` cluster.

and either

- using the [`.devcontainer`](.devcontainer) setup with VS Code or over the CLI
  with `just start-devcontainer` or
- you develop locally and have the following tools installed and on your `PATH`:

  - [`cargo`](https://www.rust-lang.org/tools/install): Rust toolchain with
    `rustup toolchain install nightly`.
  - `libpq`: The PostgreSQL C library. Normally comes with packages such as
    `postgres` on \*nix systems.
  - [`tilt`](https://docs.tilt.dev/install.html): Auto-deploy changes directly
    to a running Kubernetes cluster when working in the repository and get
    instance feedback.
  - [`kind`](https://kind.sigs.k8s.io/docs/user/quick-start): A Kubernetes
    cluster which runs in containers managed by `docker` or `podman`.
  - [`kustomize`](https://kubectl.docs.kubernetes.io/installation/kustomize):
    Rendering Kubernetes YAML manifests to specify
    resources/provisioning/deployments in the Kubernetes cluster.
  - [`httpie`](https://httpie.io/docs/cli/installation): A http client which is
    easier/more intuitive to use than `curl` \[optional\].
  - [`k9s`](https://k9scli.io/topics/install): A command-line tool to visualize
    what is running in your Kubernetes cluster \[optional\].

## Quick Instructions

The following walks you through starting up a local Kubernetes cluster with
`kind`, inspecting the cluster and also shutting it down again.

**Note**: All commands given in the following are safe to use (virtualized or
locally scoped to the repository) to use and **will only** minimally fiddle with
your system.

### Deploy

The easiest way to run the `api` and corresponding other deployments (database
etc.) is using `tilt` on a local Kubernetes cluster, such as `kind`. The tool
`kind` is only doing the following two simple isolated parts on source file
changes:

- Building docker images and pushing them to a registry.
- Auto applying Kubernetes manifests in [`./manifests`](manifests).

Start the `kind-kikist` cluster (context: `kind-kikist`, with a local image
registry :partying_face:) with

```shell
just create-cluster
```

**Note**: `kind` will write to your `kubectl` config file located in
`~/.kube/config` or otherwise set by `KUBECONFIG` env. variable.

You can now start `k9s` to inspect the state of the cluster. No `api` pods
should be running yet.

With `tilt` installed and the `kind` Kubernetes cluster running, deploy all the
`api` etc. with:

```shell
just deploy-up
```

Open the `tilt` web browser [`http://localhost:10350`](http://localhost:10350)
to see the log & status of all running components, notably `api` and `postgres`.

### Shutdown

Killing the cluster is as simple as:

```shell
just delete-cluster
```

which will kill all resources and pods.

## Locally Building Components

All components can be build with e.g.

```shell
just [--set parallel true] build
```

which will run the build task over all components.

to build a single component `<component>` either run `just build` inside the
`components/<components>` directory or use

```shell
just component <component> build`
```

inside the repository root. All binaries are in the `target` directory inside
the repository root.

## Deploying Components to the Cluster (Kubernetes)

The tool `tilt` will run all services and build all docker containers to the
`ttl.sh` registry (ephemeral images).

It will watch for changes to any files (including the
[service manifests](manifests) and redeploy the services, configuration maps as
far as possible.

To start the loop run (after `just create-cluster`) do:

```shell
just deploy-up
```

which loads an optional user-defined settings file
[`manifests/.env.yaml`](manifests/.env.yaml.tmpl). You can use the local
registry for uploading the container images or `ttl.sh` and also configure if
you want to build a `debug` release for more log output on `trace` and `debug`
levels.

To remove all resources from the development cluster use:

```shell
just deploy-down
```

You can inspect continuously the state of the cluster with `k9s` and also watch
`tilt` what its doing by inspecting
[http://localhost:10350](http://localhost:10350).

## Development

### Debugging in Rust

- Either use VS Code with the rust extension or
- Debug in `neovim` as fancy-pancy as possible by using the
  [`nvim-dap.lua`](.nvim/nvim-dap.lua) file which is automatically loaded if you
  use the plugin `{ "klen/nvim-config-local" }` which will execute
  [`.nvim/nvim.lua`](.nvim/nvim.lua) when you open this repo in `nvim`. When you
  start the debugger (see plugin
  [`nvim-dap`](https://github.com/mfussenegger/nvim-dap)) it will prompt you
  which executable you want to debug.

### Database Inspection

Run `just start-db-tool` and make a connection with the `DATABASE_URL` in your
configured [components/api/.env](/components/api/.env.tmpl).

### Githooks

You can install Githooks by running the manual
[install here](https://github.com/gabyx/Githooks#quick-secure) and then running:

```shell
cd repository
git hooks install
git hooks config enable-containerized-hooks --global --set
```

in this repository. To show all running hooks run `git hooks list`. To disable
running hooks use either

- `GITHOOKS_DISABLE=1 <your-cmd>` or
- `git commit --no-verify ...` or
- `git hooks uninstall` to completely remove the hook run wrappers from
  `.git/hooks`.

### CI/CD

#### Gitlab

Either use the free Gitlab credits or start your own runner with `docker` by
running:

```shell
just start-gitlab-runner <token>
```

where the `<token>` is the Gitlab runner token obtained from setting up a
project specific runner in Gitlab. After starting the runner, the config mount
will be inside `.gitlab/local`.

### Testing API Calls

There is a simple `just test manual` command which tests some simple API calls
for manual debugging and investigations.
