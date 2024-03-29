## ハッシュ関数
### 求められる性質
- 出力サイズが一定
- 一方向性
  - 元のデータの候補が十分にあるときのみ成立する
- 衝突困難性：異なる2個のデータで同じハッシュ値になるものを見つけるのが難しい
  - 第二現像計算困難性を含む
    - 第二現像計算困難性：あるデータが与えられたとき，そのハッシュ値と同じハッシュ値になる別のデータを見つけるのが難しい

→データの正しさや改ざん検知に使う

### パスワードのハッシュ値をそのまま保存するのは安全ではない
レインボーテーブル攻撃に弱い

### ソルト
ユーザごとに異なる乱数を用意し，(パスワード，ソルト)の組のハッシュを保存

### ストレッチング
ハッシュ値のハッシュ値をもとめる操作を何度も繰り返す

FIXME
- PBKDF2とは
- HMACとは
- PPAPとは
- SHA-256
- SHA-3


## PKI(公開鍵基盤)
人や組織に紐づく公開鍵の対応を保証する仕組み

### 認証局
人や組織に紐づく公開鍵を保証する機関

### 公開鍵証明書(サーバ証明書)
認証局が発行する証明書．人と公開鍵の紐づくを確認した証
X509という規格でフォーマットが定められている．
公開鍵と対応する秘密鍵をもつ所有者の属性情報に認証局の署名をつけたもの

### ルート認証局
自分で正当性を証明する認証局．ここで発行される証明書を自己署名証明書という
トラストアンカーと呼ばれる

### フィンガープリント
証明書のハッシュ値

### 証明書失効リスト(Certificate Revocation List)
PKIで失効した公開鍵証明書の一覧．
秘密鍵の漏洩や，署名鍵の漏洩があった場合，公開鍵証明書は意味がなくなる．
悪用される前に失効届を出す必要あり

#### CRLの問題点
サイズが増大し続ける

#### OCSP(Online Certificate Status Protocol)
証明書が失効していないかを確認する

```mermaid
sequenceDiagram
    A->>Server X: Request
    Server X-->>A: Certificate Cx
    A->>OSCP Responder: 失効確認
    OSCP Responder->>A: OSCP Response
```

OSCPレスポンダは有効性検証局やVA(Validation Authority)が管理・運用している

デメリット：アクセスしようとしているサイトの情報がOSCPレスポンダに渡るのでプライバシーの問題が発生する

#### OCSPステープリング
サーバで最新の失効情報を保持して起き，クライアントがサーバに接続した際に証明書と失効情報を返す．サイトのアクセス情報が漏洩しなくなり，通信速度も向上する

```mermaid
sequenceDiagram
    Server X->>OSCP Responder: 失効確認
    OSCP Responder->>Server X: OSCP Response
    認証局->>OSCP Responder: OSCP Response
```

```mermaid
sequenceDiagram
A->>Server X: Request
    Server X-->>A: Certificate Cx + OSCP Response
```

### その他
- CRLSets
- OneSRL
- CRLite

#### ドメイン証明
認証局が公開鍵証明書を発行する際に，申請者が申請ドメインの管理者であると判断して認証する方法

メール認証
```mermaid
sequenceDiagram
管理者->> 認証局: admin@sample.example
認証局->> 管理者: sample.exampleの公開鍵証明書を発行
```
admin@のメールは管理者しか使えないのでsample.exampleの管理者であると判断できる

サーバ認証
```mermaid
sequenceDiagram
サーバ->> 認証局: 公開鍵証明書の発行依頼
認証局->> サーバ: トークンX
サーバ->> 認証局: 特定のページにトークンXを記す
認証局->> サーバ: Xを確認したので発行
```

- ACME(Automatic Certificate Management Environment)
- Let's Encrypt

#### ドメイン認証の問題点
ずさんな認証局がadminの文字列の存在のみで証明書を発行するケースがある


- 組織認証
- 拡張認証

### 証明書の透明性(Certificate Transparency)
SCT(Signed Certificate Timestamp)：ログサーバによる署名がついたタイムスタンプ
```mermaid
sequenceDiagram
サーバ->> 認証局: 証明書の発行依頼
認証局->> CTのログサーバ: 証明書の登録依頼
CTのログサーバ->> 認証局: ログに追記・SCTを発行
認証局->> サーバ: SCTつき証明書
```

### CTが普及することのメリット
- SCTがついていなければ怪しいと判断できる
- ログサーバを監視することで自分のサイトの不正な証明書が発行されていないかを監視できる

### TLS
ブラウザでインターネットにアクセスするときの暗号プロトコル

#### TLS1.3の特徴
- 性能の向上
  - ハンドシェイク効率化
- 安全性の向上
  - 暗号化アルゴリズムの整備
  - 新しい鍵導出アルゴリズム
  - 形式検証
  - 認証付き暗号
  - 前方秘匿性
  
#### ハンドシェイク効率化

TLS1.2：暗号通信の前に3回データをやり取りしている
```mermaid
sequenceDiagram
クライアント->> サーバ: ClientHello
サーバ->> クライアント: ServerHello, Certificate, ServerKeyExchange, ServerHelloDone
クライアント->> サーバ: ClientKeyExchage, Finished
サーバ->> クライアント: Finished, Enc(Application data)
クライアント->> サーバ: Enc(Application data)
```

1. 通信開始のためのパラメータ送信
2. 暗号通信のパラメータを決定(ServerHello)，サーバ証明書(Certificate)とマスターシークレット(master secret)のための情報(SecretKeyExchange)を送る．最後にServerHelloDoneで完了
3. サーバ証明書を検証して問題なければマスターシークレットのための情報(ClientKeyExchange)を送り，完了(Finished)
4. サーバもマスターシークレットを作り完了通知を送り(Finished)，暗号通信を開始


TLS1.3：暗号通信の前のデータのやり取り回数が減っている
```mermaid
sequenceDiagram
クライアント->> サーバ: ClientHello(KS,PSK)
サーバ->> クライアント: ServerHello(KS, PSK), Enc(EncryptedExtensions),Enc(Certificate), Enc(Finished)
サーバ->> クライアント: Enc(Application data)
クライアント->> サーバ: Enc(Finished),Enc(Application data)
```

1. 通信開始を表明(ClientHello)．このときDH鍵共有のための秘密情報(KS)と事前鍵共有(PKS)を送信する
2. サーバもKSやPKSを送信(ServerHello)して鍵を共有．その後サーバパラメータやサーバ証明書を暗号化して送信
3. 認証などが終わったらサーバはアプリのための暗号通信を開始する
4. クライアントも認証処理が終わったらアプリのための暗号通信を開始する

#### 暗号化アルゴリズムの整備
- ストリーム暗号ChaCha20を導入
- 楕円曲線暗号や署名が必須に(Curve25519)

#### 新しい鍵導出アルゴリズム
- HMACを用いた鍵導出関数HKDF(HMAC-based Key Derivation Function)の導入
  - 短いシードや補助入力から，秘密鍵として利用できる複数の安全な疑似乱数を取得する関数

#### 形式手法による安全性検証
あるプロトコルが与えられたとき，それが安全か否かをコンピュータで半自動的に判定する方法．そのプログラムを自動検証ツールという

- モデル検証
- 定理証明