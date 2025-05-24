# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview


## ディレクトリ構成
### apps
アプリケーションのコードが置かれる

#### frontend
フロントエンドのコードが置かれる

#### services
バックエンドのコードが置かれる

### bin
プロジェクトコマンド。開発に便利なCLI

### cli
プロジェクトコマンドの実装

### db
データベース関連のファイルが置かれる

### development
ローカルの開発環境のDockerファイルなどが置かれる

### docs
アプリケーションの設計データが置かれる

#### typespec
APIに関するTypespec。フロントエンドのコードを生成する


## コード生成の注意
以下のフォルダーやファイルは自動生成されるのでコードの変更やファイルの追加はしないでください。
- apps/frontend/src/api
- apps/frontend/src/helpers
- apps/frontend/src/models
- apps/frontend/src/templateServiceClient.ts
