#!/bin/bash

set -euo pipefail

DAY_NUM="$1"

touch input/day$DAY_NUM.txt src/bin/day$DAY_NUM.rs src/bin/examples/day$DAY_NUM.txt
code input/day$DAY_NUM.txt src/bin/day$DAY_NUM.rs src/bin/examples/day$DAY_NUM.txt