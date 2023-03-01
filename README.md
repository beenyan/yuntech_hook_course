# Build 教學

> 確保擁有環境 [Rust](https://www.rust-lang.org/) & npm & [node.js](https://nodejs.org/en/)

## 1. Install node_modules
```
pnpm install
```

## 2. 在 src-tauri 新增資料夾 assets-dev

## 3. 安裝 [Diesel](https://diesel.rs/) CLI tool
```
cargo install diesel_cli
```

## 4. 安裝 Diesel CLI
```
cargo install diesel_cli --no-default-features --features sqlite
```

## 5. 開發指令
```
pnpm tauri dev
```

## 5. Build 指令
```
pnpm tauri build
```

# Issues
> **sqlite3.dll** was not found
>* 方法 1 將 assets 底下的 sqlite3.dll 移至 C:\Windows\System32
>* 方法 2 自行編譯 sqlite3.dll 移至 C:\Windows\System32
