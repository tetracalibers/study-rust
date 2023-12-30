#!/usr/bin/env bash

# 出力ディレクトリの変数を定義
OUTDIR="tests/expected"
# 出力ディレクトリが存在しないかどうかをテストし、必要であれば作成
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" >$OUTDIR/hello1.txt
echo "Hello" "there" >$OUTDIR/hello2.txt
echo -n "Hello  there" >$OUTDIR/hello1.n.txt
echo -n "Hello" "there" >$OUTDIR/hello2.n.txt
