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

<br>
<hr>
22:55<br>
cnosegui resolver o problema da biblioteca templates N está importando o minininja.
<br>Verifiquei <a href="https://github.com/rwf2/Rocket/blob/master/contrib/dyn_templates/Cargo.toml">neste link</a> como chatgpt indicou olhar no repo do rocket, e verifiquei que tinha que atualizar manualmente a lib pra versao mais atual. <br>
Depois disso, rodou de boas.<br>

<hr>

#### 12/10/24, 08:55
atual config do sistema:

```sh
src/
├── controllers
│   ├── home_controller.rs
│   └── mod.rs
├── main.rs
└── views
    ├── base
    │   ├── footer.html.j2
    │   ├── layout.html.j2
    │   └── nav.html.j2
    ├── home
    │   ├── base
    │   │   ├── error
    │   │   │   └── _404.html.j2
    │   │   ├── footer.html.j2
    │   │   ├── layout.html.j2
    │   │   └── nav.html.j2
    │   └── index.html.j2
    ├── index.html.j2
    ├── sim.html.j2
    └── static
        ├── app.js
        ├── css
        │   └── style.css
        └── js
```

eu tentei de diversas maneiras instalar o `cargo-watch` para ter um autoreload no app, mas não conseguir pelo celular, msm usando arch.<br>
creio que rode pelo pc, contudo, vi que o rust não compila html.j2, eu edito ele e ele é aceito, ele muda no server, é interessante. Logo vejo que não preciso disso.<br>
mudei o template de `tera` para `minininja`, em homenagem ao jinja do python. creio que seja melhor.
<br>
O `cargo-watch` funcionou de boas no pc, com `cargo install cargo-watch --locked`.<br>

O repsitório de
```sh
├── home
│   ├── base
│   │   ├── error
│   │   │   └── _404.html.j2
│   │   ├── footer.html.j2
│   │   ├── layout.html.j2
│   │   └── nav.html.j2
│   └── index.html.j2
```
serve apenas para Reinan se orientar na construção dos dir dos templates.

<hr>
### 22:29

- Infelizmente tive uma desilução com Rust em Vercel e vi que o Vercel só aceita Functions API em Rust, mas não um projeto MVC em Rust.<br>
Infelizmente, este projeto irá ser descontinuado aqui.
