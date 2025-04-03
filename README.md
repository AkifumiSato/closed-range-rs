# 閉区間検査のCLI実装例

## 概要

本リポジトリは閉区間を検査するCLI実装の例です。主に以下の機能を提供します。

- 閉区間の表示
- 値が閉区間に含まれるかのチェック
- ある閉区間が別の閉区間のサブセット（部分集合）かどうかのチェック

## 使い方

### 基本構文

```
closed_range_rs <下限値> <上限値> [サブコマンド]
```

### 例

1. 閉区間の表示

```bash
$ closed_range_rs 1 10
Range: [1, 10]
```

2. 値が閉区間に含まれるかチェック

```bash
$ closed_range_rs 1 10 contains 5
[1, 10] contains 5: true

$ closed_range_rs 1 10 contains 15
[1, 10] contains 15: false
```

3. ある閉区間が別の閉区間のサブセットかチェック

```bash
$ closed_range_rs 5 10 subset 1 15
[5, 10] is subset of [1, 15]: true

$ closed_range_rs 1 10 subset 5 15
[1, 10] is subset of [5, 15]: false
```

## 開発

### 必要条件

- Rust 1.83.0
