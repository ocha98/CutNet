## これは何？
これはプログラムをネットを切断した状態で実行するものです

## 使い方
```
cutnet [Optins] -- <Program>
```

ルート権限が必要です

### オプション
- `-u`, `--userid`: 指定されたユーザーidで`Program`を実行します
- `-g`, `--groupid`: 指定されたグループidで`Program`を実行します

## 例
```
$ sudo ./cutnet -- ping 8.8.8.8
ping: connect: Network is unreachable
```
