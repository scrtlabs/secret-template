#!/bin/bash
label=123098124091824
set -x
secretcli tx compute instantiate $codeId '{"count":55}' --label "${label}" --from 'SecretIDE-Deployment' -y
