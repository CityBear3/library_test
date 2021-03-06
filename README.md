[![Rust](https://github.com/CityBear3/library_test/actions/workflows/rust.yml/badge.svg)](https://github.com/CityBear3/library_test/actions/workflows/rust.yml)
# library_test

## ライブラリの説明
コラッツ予想を試行することができます．
<br>
<br>

## 実行方法
### 1. 下記のコマンドでbuild．
```
make build
```

### 2. シンボリックリンクを作成．
```
ln -s /target/release/libtest_lib.dylib test_lib.so
```

### 3. サンプルスクリプトを実行．
```
python3 test.py
```

### 4. テストの実行．
```
cargo test
```
