# Markdown-to-PDF Service

**Note: This is currently in development and not feature complete.**

<!--toc:start-->

- [Requirements](#requirements)
- [Project Structure](#project-structure)
- [Run Instructions](#run-instructions)
- [Local Development Loop for Fast Feedback](#local-development-loop-for-fast-feedback)
- [Development Loop using `tilt` (Kubernetes)](#development-loop-using-tilt-kubernetes)
- [Development](#development)
  - [Debugging in Rust](#debugging-in-rust)
  - [Githooks](#githooks)
  - [CI/CD](#cicd) - [Gitlab](#gitlab)
  <!--toc:end-->

This is a demo project to showcase a small microservice architecture by exposing

- a simple [`web` frontend service](web/src/main.rs) which enables the user to
  upload Markdown files (single or multiple) and let it convert by sending it to
- the [`api` service](api/src/main.rs) which will add a job into the `rabbitmq`
  queue, such that
- the [`converter` service](markdown-to-pdf/src/main.rs) eventually (when idle)
  pulls a job from the `rabbitmq` queue and stores the result in the database.

## Requirements

- Either use the `flake.nix` by doing `nix develop --command zsh` which setups a
  development shell with all tools installed,
- Or you need to have the following installed:
  - `rust` with `rustup toolchain install nightly`
  - `libpq` must be installed. Comes with `postgres`.
  - `tilt`, `kustomize`, `httpie`, `docker`, `kind`

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

## Run Instructions

The easiest way to run this is using `tilt` and on a working Kubernetes cluster,
such as `kind`. Start the `kind-md2pdf` cluster with

```shell
just create-cluster
```

With `tilt` installed and a Kubernetes cluster running, deploy all pods with:

```shell
just deploy-up
```

## Local Development Loop for Fast Feedback

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

inside the repository root.

## Development Loop using `tilt` (Kubernetes)

The tool `tilt` will run all services and build all docker containers to the
`ttl.sh` registry (ephemeral images).

It will watch for changes to any files (including the
[service manifests](manifests) and redeploy the services, configuration maps as
far as possible.

To start the loop run:

```shell
just deploy-up
```

and to remove all resources from the development cluster use:

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
