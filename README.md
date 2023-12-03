# Gitlab local helpers
Collectction of helpers to make local pipeline debugging easy.

## gitlab-local-command
Script for generating command used in job script section with appropriate env variables declared in pipeline.
Usage:
```
# List jobs from pipeline
./gitlab-local-command list -f <pipeline_file>

# Generate command to run, using global and job-specific environment variables.
./gitlab-local-command generate -f <pipeline_file> -s <job_name>
```