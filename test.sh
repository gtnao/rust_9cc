#!/bin/bash

set -e

function cleanup() {
	rm -f tmp tmp.s tmp.out
}

function expect() {
	cargo run -- "${2}" >tmp.s 2>/dev/null
	cc -o tmp tmp.s
	set +e
	(
		./tmp
		echo $?
	) >tmp.out
	set -e
	if [ "$(cat tmp.out)" = "${1}" ]; then
		echo "${2} => ${1} ok"
	else
		echo "${2} => ${1} ng"
		echo "NG!"
		cleanup
		exit 1
	fi
}

expect "0" "0"
expect "1" "1"
expect "2" "1+1"
expect "10" "2*3+4"
expect "26" "2*3+4*5"
expect "5" "50/10"
expect "9" "6*3/2"
expect "45" "(2+3)*(4+5)"
expect "153" "1+2+3+4+5+6+7+8+9+10+11+12+13+14+15+16+17"

expect "0" "0 < 0"
expect "0" "1 < 0"
expect "1" "0 < 1"
expect "0" "0 > 0"
expect "0" "0 > 1"
expect "1" "1 > 0"

expect "0" "4 == 5"
expect "1" "5 == 5"
expect "1" "4 != 5"
expect "0" "5 != 5"

expect "1" "4 <= 5"
expect "1" "5 <= 5"
expect "0" "6 <= 5"

expect "0" "4 >= 5"
expect "1" "5 >= 5"
expect "1" "6 >= 5"

echo "OK!"

cleanup
