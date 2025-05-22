#!/bin/sh

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'

which rs-bloom2count | fgrep -q rs-bloom2count || exec sh -c '
	echo rs-bloom2count missing.;
	echo install the crate using cargo install.;
	exit 1
'

dd \
	if=/dev/urandom \
	of=/dev/stdout \
	bs=16 \
	count=32 \
	status=none |
	wazero \
		run \
		./rs-uuids2tinybloom.wasm |
	ENV_NUM_HASH=4 rs-bloom2count
