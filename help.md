#### 09/10/24, 18:15

## instalando rust

```rs
apt install rust cargo
```

1. clone o repositorio do git

2. abra e deh o `cargo new _nome_do_app_`

3. mova tudo da pasta do app para a pasta do git

4. cole no cargo


```toml
#rocket_contrib = "0.5.0-rc.1"
[dependencies]
rocket = "0.5.0"
[dependencies.rocket_dyn_templates]
version = "0.1.0"
features = ["handlebars", "tera"]
```

5. crie um sistema parecido com

```sh 
src/
├── controllers
│   ├── home_controller.rs
│   └── mod.rs
├── main.rs
└── views
    └── tera
        ├── index.html.tera
        └── pages
            └── home.html.tera
```

6. deh um `cargo build --release` ou `cargo run` para rodar de vez.
<br>um compila o outro compila e roda.

