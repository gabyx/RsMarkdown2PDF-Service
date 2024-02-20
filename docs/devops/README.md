# DevOps Requirements

This document is an elaboration to push the boundaries of today's (?) CI setups
to fulfil the following requirements. The term _job_ is referred to a Gitlab CI
job for simplicity.

## Requirements

1. _CI jobs are reproducible._

   This is a no-brainer. If the architecture imposes reproducibility than
   maintenance is reduced and finding errors in the CI jobs is less burdensome.

2. _CI jobs are encapsulated and deterministic._

   We ideally do not want that successive jobs run in the same context. That
   means a job should have its own (cached) checkout and runs in a environment
   which is not shared with other jobs. This enables deterministic behavior and
   is a direct requirement of requirement 1.

3. _CI jobs are identical to development workflow or have neglectable
   differences._

   What a developer runs on their machine and what CI runs in their jobs must be
   minimally different. The best would be that the same scripts can be run in
   the repository as the developer executes in their setups.

4. _CI jobs are started quickly._

   For each job we don't to wait till a new executor instance is provisioned for
   us. This basically means we restrict our self to bare-metal containerized use
   cases where no virtual machines are involved.

5. _CI jobs are scripted in any language._

   That means e.g. with Gitlab CI or Github Actions. The pipeline YAML
   configurations are only glue code to start the actual CI code maintained in
   the repo for a job. This makes the CI code independent of the provider (and
   also due to requirement 3).

6. _CI jobs have full control over a container engine (`podman` or `docker`) or
   a way to start sub-jobs_.

   If the code of a job can run normal tools and at the same time also spawn new
   containers with e.g. `docker run` or also `docker build`, the whole CI and
   tooling inside the repository gets extremely less complicated. An example
   would be this `lint.sh` script which developers can also run on their setup
   (even in a `.devcontainer`).

   ```shell
    #!/usr/bin/env bash

    assert_no_diffs
    docker run -v "$(pwd):/workspace" -w "/workspace" --rm lint-rust:1.0.0
    docker run -v "$(pwd):/workspace" -w "/workspace" --rm lint-docs:1.0.0
    assert_no_diffs
   ```

   _Optional:_ Also the container engine should have their images cached and
   probably shared with other CI runs or jobs. The granularity is not so
   important but the higher up the cache may be the less secure things might
   get.

The requirements point towards containerized CI solution where the jobs somehow
have full control over a container engine. We explore the solutions with the
Gitlab CI provider and its runner `gitlab-runner` which can be installed and can
execute and launch CI jobs.

The following brain-goo leaking out of my ears was taken from this
[blog post](https://blog.nestybox.com/2020/10/21/gitlab-dind.html) which
explains how to make a containerized CI setup secure.

## Solution 1: Gitlab Runner using Host's Container Engine - Docker-in-Docker with Service Container

For demonstration purposes the
[`tools/start-gitlab-runner-docker.sh`](../../tools/start-gitlab-runner.sh)
starts a container running a Gitlab runner which communicates to the container
engine on the host (via the mounted `/var/run/docker.sock` inside the CI
container.)

![docker-in-docker-service](docker-in-docker-service.drawio.svg)

A job might look like this:

```yaml
lint:
  image: generic:1.0.0
  services:
    - name: docker:24-dind
      alias: docker
  script:
    - lint.sh
```

The Gitlab runner inside the CI container starts the container `generic:1.0.0`
and the service container `docker:24-dind` which lets the `lint.sh` script use
`docker run -v "$CI_BUILD_DIR:/workspace" ...`. The path `$CI_BUILD_DIR=/builds`
is interpreted however on the Docker engine which is in this case the
`docker:24-dind` which works because the Gitlab runner started the two job
containers with the same mount for `$CI_BUILD_DIR=/builds`.

The setup works, is however not very secure as the Docker engine on the host is
used and privilege escalation can be from inside the job containers (because
they are run privileged because we use `docker run`).

**Open Questions:** I am not sure how container image caching should be done for
`docker:24-dind`. Could we create a `docker volume create image-cache` on the
host and let instruct the runner to mount this volume `image-cache` also to the
job's service container `docker:24-dind`?

## Solution 2: Gitlab Runner Using Own Container Engine.

Use the `pipglr` which provides the following architecture:
