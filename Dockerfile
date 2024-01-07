# ビルドステージ
FROM solanalabs/rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/solana-airdrop

# ソースコードのコピー
COPY . .

# リリースビルドの作成
RUN cargo build --release

# 実行コマンドを設定
CMD ["./target/release/solana_airdrop"]
