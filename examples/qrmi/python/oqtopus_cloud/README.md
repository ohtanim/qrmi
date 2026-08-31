# Oqtopus Cloud QRMI - Examples in Python

## Prerequisites

* Python 3.11, 3.12 or 3.13
* [QRMI python package installation](../../../../README.md)

## Install dependencies

```shell-session
$ source ~/py311_qrmi_venv/bin/activate
$ pip install -r ../requirements.txt
```

## Set environment variables

Because QRMI is an environment variable driven software library, all configuration parameters must be specified in environment variables. The required environment variables are listed below. This example assumes that a `.env` file is available under the current directory.

| Environment variables | Descriptions |
| ---- | ---- |
| {device_id}_QRMI_OQTOPUS_API_TOKEN | Oqtopus Cloud API token |
| {device_id}_QRMI_OQTOPUS_BASE_URL | IQM Server API endpoint |
| {device_id}_QRMI_JOB_ACQUISITION_TOKEN | (optional) pre‐set session ID |
| {device_id}_QRMI_OQTOPUS_TIMEOUT_SECS | (optional) request timeout in seconds |
| {device_id}_QRMI_OQTOPUS_PROXY_URL | (optional) proxy URL |


## How to run

```shell-session
$ python example.py -h
usage: example.py [-h] device_id

An example of Oqtopus Cloud QRMI

positional arguments:
  device_id   Oqtopus device ID

options:
  -h, --help  show this help message and exit
```
For example,
```shell-session
export qulacs_QRMI_OQTOPUS_API_TOKEN=your api token
export qulacs_QRMI_OQTOPUS_BASE_URL=https://demo-api.oqtopus.io

python example.py qulacs
```
