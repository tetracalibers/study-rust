## Rust + WASM で Hello World

### 必要なもの

Node.jsとRustの実行環境が整っていることが前提

```
cargo install wasm-pack
```

### ビルド手順

`wasm`ディレクトリに移動

```
cd wasm
```

`wasm-pack`でRustコードをWASMにビルド

```
wasm-pack build --target web
```

ルートディレクトリ（`wasm-hello`）に戻る

```
cd ..
```

JS側の依存をインストール

```
yarn
```

### 実行手順

```
yarn dev
```

ブラウザに表示された「Greet」ボタンをクリックすると、アラートが表示される。
