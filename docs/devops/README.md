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

   For each job we don't want to wait till a new executor instance is
   provisioned for us. This basically means we restrict our self to bare-metal
   containerized use cases where no virtual machines are involved.

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

The requirements point towards containerized CI solutions where the jobs somehow
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

![runner-uses-hosts-docker-engine](runner-uses-hosts-docker-engine.drawio.svg)

A job might look like this:

```yaml
lint:
  image: generic:1.0.0
  services:
    - name: docker:24-dind
      alias: docker
  script:
    - docker run -v "$CI_BUILD_DIR:/data" -w "/data" alpine:latest ls -a /data
```

The Gitlab runner inside the CI container starts the container `generic:1.0.0`
and the service container `docker:24-dind` which lets the `lint.sh` script use
`docker run -v "$CI_BUILD_DIR:/workspace" ...`. The path `$CI_BUILD_DIR=/builds`
is interpreted however on the Docker engine which is in this case the
`docker:24-dind` which works because the Gitlab runner started the two job
containers with the same mount for `$CI_BUILD_DIR=/builds` which is further more
a dynamically created docker volume on the host.

The setup works, is however not very secure as the Docker engine on the host is
used and privilege escalation can done be from inside the job containers
(because they are run privileged because we use `docker run`).

**Open Questions/Notes:**

- I am not sure how container image caching should be done for `docker:24-dind`.
  Could we create a `docker volume create image-cache` on the host and let
  instruct the runner to mount this volume `image-cache` also to the job's
  service container `docker:24-dind`?

- This setup is equivalent
  [to this visualization from nestybox](https://blog.nestybox.com/2020/10/21/gitlab-dind.html#setups-that-currently-dont-work)
  but without using `sysbox`.

## Solution 2: Gitlab Runner Using Own Container Engine.

For demonstration purposes the
[`tools/start-gitlab-runner-podman.sh`](../../tools/start-gitlab-runner.sh)
starts a container running a Gitlab runner including a `podman` engine.

It uses the [`pipglr`-image](https://gitlab.com/qontainers/pipglr) as the CI
container which provides the following architecture:

![runner-uses-hosts-docker-engine](runner-uses-nonhost-podman.drawio.svg)

The CI Container contains `podman` as well as the `gitlab-runner`. Podman in CI
Container stores its images into a mounted volume on the host `pipglr-storage`.
The volume `pipglr-cache` is used for the `gitlab-runner` which stores its build
checkouts etc. in this volume. The runner starts a job by directly launching it
with its contained `podman` executable. The runner mounts the
`/home/runner/podman.sock` to the job container together with `/build`.

The caveat happens when the job tries to run some `docker run` commands and
wants to mount the `/build` directory. This does not work as the path `/build`
does not exist on CI Container. But a volume name `auxvol` exists and can be
mounted to `/data`. Going over a volume means that any thing needed must be
copied first to this `/auxvol/<some-non-concurrent-accessed-path>` then mounted
with `-v auxvol/<some-non-concurrent-accessed-path>` (**note this is a
sub-directory volume mount** which only works with `podman` but should soon work
also with `docker` ([see here](https://github.com/moby/moby/issues/32582)).

```yaml
lint:
  stage: format
  needs: []
  image: alpine:latest
  rules:
    - *defaults-rules
  variables:
    CONTAINER_HOST: unix://var/run/podman.sock
  script:
    - apk add podman
    - ls /auxvol
    - mkdir -p /auxvol/a/b/c
    - echo "asdf" > /auxvol/a/b/c/A.txt
    - podman run -v "auxvol/a/b/c:/data" -w "/data" alpine:latest ls /data
```

**Open Questions/Notes:**

- This setup is somewhat the exact same as
  [described here](https://blog.nestybox.com/2020/10/21/gitlab-dind.html#gitlab-runner--docker-in-a-system-container)
  but instead of docker we use `podman` and more isolated by doing
  `--runtime=sysbox`. The same caveat described above is hidden in the details.

- How can this `copy` to `auxvol` setup be improved? Is there a better
  alternative? Have a dedicated runner as solution 2 with all tools needed for
  the repo and use the `shell` executor? So the CI container would be repository
  specific. You can still have multiple such runners which you can select with
  tags inside the `pipeline.yaml`. Would that work?

## Conclusion

The two solutions work with both their own stupid caveats.

- Solution 1 has no troubles mounting `/build` but is insecure and can be made
  secure with `sysbox`.
- where as solution 2
