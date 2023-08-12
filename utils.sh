#!/bin/sh

TARGET=target
EXE=../target/debug/ahc022

if [ $1 = "test" ]; then
	if [ ! -d $TARGET ]; then
		echo "Compiling ..."
		cargo build
	fi

	cd ./tools/
	for file in `ls ./in`; do
		echo "Testing file $file ..."
		cargo run --release --bin tester $EXE < ./in/$file > ./out/$file

		if [ $? -ne 0 ]; then
			echo "Test finished unsuccessfully ..."
			exit 1
		fi
	done
else
	echo "I do not know the command \"$1\""
fi
