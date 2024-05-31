#!/bin/bash
test "$1" = "-h" -o "$1" = "--help" && { echo Syntax: $0; echo runs cargo ziggy run on all crashes; exit 0; }
test -d output || { echo Error: you are not in the fuzz/ directory that contains the output/ directory; exit 1; }
rmdir output/*/crashes/* 2>/dev/null
#PATTERN=`sysctl kernel.core_pattern 2>/dev/null | sed 's/.* = *//'`
#PID=`sysctl kernel.core_uses_pid 2>/dev/null | sed 's/.* = *//'`
#OK=1
#test -z "$PATTERN" -o "$PATTERN" = core || { OK=; echo "Warning: core pattern is not set to 'core', run 'sudo afl-system-config'"; }
#test -z "$PID" -o "$PID" = "0" || { OK=; echo "Warning: core pid pattern is set, run 'sudo afl-system-config'"; }
#if [ -n "$OK" ]; then
#  ulimit -c unlimited
#else
#  ulimit -c 0
#  sleep 2
#fi
DIRCNT=`ls -d output/* 2>/dev/null|grep -vw target|wc -l`
DIR=`ls -d output/*|grep -vw target`
test -d "$DIR/crash_output/" || mkdir $DIR/crash_output || exit 1
mkdir .foobar$$ 2>/dev/null
cargo ziggy run -i .foobar$$
rm -rf .foobar$$
FILENO=`ls target/runner/debug/*-runtime-fuzz 2>/dev/null | wc -l`
test "$FILENO" = "1" || { echo Error: multiple run runtimes or build failed; exit 1; }
FILE=`ls target/runner/debug/*-runtime-fuzz 2>/dev/null`
export RUST_BACKTRACE=1
export RUSTFLAGS=
for crash in `find $DIR/crashes/ -type f`; do
  out=`echo $crash | tr ' \t/:' _`
  test -f $DIR/crash_output/$out.txt || {
    echo
    "$FILE" $crash 2>&1 | tee $DIR/crash_output/$out.txt
    echo Saved output to $DIR/crash_output/$out.txt
    #test -f core && { mv core $out.core; echo Saved core file to $DIR/crash_output/$out.core; }
  }
done
echo
echo
echo Full list of found issues:
echo ==========================
grep -h ' panicked at ' $DIR/crash_output/*.txt | sort -u

echo grep might fail because arguments too big, but here is a little tldr of the situation
echo ==========================
find $DIR/crash_output/ -name "*.txt" -print0 | xargs -0 rg ' panicked at '

