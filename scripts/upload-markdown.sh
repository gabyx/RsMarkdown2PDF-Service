#!/bin/bash

http --multipart PUT localhost:8000/api/job name=sample stylesheet=fancy.css file@sample.md
