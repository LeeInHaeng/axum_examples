vscode rust-analyzer 미동작 관련
================================
##### failed to find any projects in , rust-analyzer failed to discover workspace

- 하위 프로젝트의 Cargo.toml 인식 불가능 (깊은 depth 는 자동 인식이 안됨)

- .vscode 의 settings.json 수정 :  파일위치 (Window 기준) %AppData%/Roaming/Code/User/settings.json

```
"rust-analyzer.linkedProjects": [
  "relateive/path/to/the/project/directory/Cargo.toml",
]
```



axum docs
================================
- rust docs : https://docs.rs/axum/latest/axum/

- github examples : https://github.com/tokio-rs/axum/tree/main/examples



axum cargo
================================
```
cargo add axum

cargo add tower

cargo add tower_http
```

- Cargo.toml

```
tokio = { version = "1.0", features = ["full"] }
tower-http = { version = "0.5.2", features = ["cors"] }
```



other cargo
================================
- scraper 사용시 : cargo add reqwest , cargo add scraper


- json 관련 : cargo add serde , cargo add serde_json
```
serde = { version = "1.0", features = ["derive"] }
```


- url 파싱 관련 : cargo add url


fly commands
================================
- flyctl auth login

- fly launch

- 배포시 : fly deploy




svelte 관련
================================
- npm create svelte@latest ui

- cd ui

- tailwindcss 관련 : npm i -D tailwindcss postcss autoprefixer , npx tailwindcss init tailwind.config.cjs -p

--> tailwind.config.cjs , app.css , +layout.svelte 참고

- npm install

- npm run dev

- sveltekit static site 관련 : npm i -D @sveltejs/adapter-static

- svelte 로딩 관련 : npm i -D svelte-loading-spinners




example sites
================================
* https://mycodings.fly.dev/blog/2023-09-04-howto-rust-web-server-web-application-with-actix-web
  * vpngate.net 사이트에서 openvpn 설정파일을 쉽게 다운로드 하기 위한 스크래핑
  * src/services/openvpn
  * https://axum-examples-fly.fly.dev/openvpn