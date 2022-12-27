#! /bin/bash
set -e

rm -rf /tmp/configz
mkdir -p /tmp/configz

module="tests/$(basename "$1" .lua)"

echo "Testing: $module"
echo "----------------------------"

target/debug/configz --module "$module"

echo ""

