#!/bin/bash

http --multipart PUT localhost:8000/api/job metadata='{ "name": "sample"}' file@sample.md
