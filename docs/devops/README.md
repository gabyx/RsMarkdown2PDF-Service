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
