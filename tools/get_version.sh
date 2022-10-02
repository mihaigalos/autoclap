#!/bin/bash

grep ^version Cargo.toml | cut -d'=' -f2 | sed -e 's/\s*//' -e 's/"//g' | tr -d '\n'

