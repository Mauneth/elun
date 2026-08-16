mod builder;
mod config;
mod helper;
mod import;
mod install;
mod recipe;
mod remove;
mod selftest;
mod update;
mod ver;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("install") => {
            let pkgs = &args[2..];
            if pkgs.is_empty() {
                println!("¿Que paquete desea instalar?");
                return;
            }
            let mut fails = 0;
            for pkg in pkgs {
                fails += install::install_package(pkg);
            }
            if fails > 0 {
                println!("{fails} paquete(s) fallaron al instalar");
            }
        }
        Some("remove") => {
            let pkgs = &args[2..];
            if pkgs.is_empty() {
                println!("¿Que paquete desea remover?");
                return;
            }
            let mut fails = 0;
            for pkg in pkgs {
                if !remove::remove_package(pkg) {
                    fails += 1;
                }
            }
            if fails > 0 {
                println!("{fails} de {} no se pudieron remover", pkgs.len());
            }
        }
        Some("build") => {
            let (ok, fail) = selftest::run();
            println!("Autorevisión: {ok} tests OK, {fail} fallidos");
            if fail > 0 || ok < 30 {
                eprintln!("No se compila: la suite de tests no pasa (mínimo 30).");
                return;
            }
            let pkgs = &args[2..];
            if pkgs.is_empty() {
                println!("¿Que paquete desea compilar?");
                return;
            }
            let mut fails = 0;
            for pkg in pkgs {
                if !builder::build_package(pkg) {
                    eprintln!("Fallo al compilar '{pkg}'");
                    fails += 1;
                }
            }
            if fails > 0 {
                println!("{fails} de {} paquete(s) fallaron", pkgs.len());
            }
        }
        Some("new") => {
            let recetas = &args[2..];
            if recetas.is_empty() {
                println!("¿Que receta desea crear?");
                return;
            }
            for receta in recetas {
                helper::new_recipe(receta);
            }
        }
        Some("import") => {
            let puertos = &args[2..];
            if puertos.is_empty() {
                println!("¿Que port de CRUX desea importar?");
                return;
            }
            for port in puertos {
                import::import_pkg(port);
            }
        }
        Some("search") => println!("elun search: en progreso"),
        Some("list") => {
            let items = config::installed();
            if items.is_empty() {
                println!("(nada instalado)");
            }
            for (n, b, v) in items {
                println!("{n} v{v} ({b})");
            }
        }
        Some("update") => {
            let pkgs = &args[2..];
            if pkgs.is_empty() {
                println!("¿Qué paquete desea actualizar?");
                return;
            }
            update::update_packages(&pkgs.to_vec());
        }
        Some("check") => {
            let pkgs = &args[2..];
            if pkgs.is_empty() {
                println!("¿Qué paquete desea verificar?");
                return;
            }
            for pkg in pkgs {
                update::check_package(pkg);
            }
        }
        Some("test") => {
            let (ok, fail) = selftest::run();
            println!("Autorevisión: {ok} tests OK, {fail} fallidos");
        }
        Some("version") | Some("--version") => println!("elun 0.1"),
        Some("help") | None => {
            println!("elun install <paquete>: Instala un paquete a tu sistema");
            println!("elun remove <paquete>: Remueve un paquete de tu sistema");
            println!("elun search <paquete>: Buscar un paquete en los repositorios");
            println!("elun new <receta>: Crea una nueva receta con el helper");
            println!("elun import <port>: Importa un port de CRUX como receta");
            println!("elun build <receta>: Compila, empaqueta y publica una receta");
            println!("elun list: Lista los paquetes instalados");
            println!("elun update <paquete>: Actualiza un paquete instalado");
            println!("elun check <paquete>: Verifica un paquete instalado");
            println!("elun test: Corre la suite de autorevisión (≥30 tests)");
            println!("elun version: Muestra la version");
        }
        Some(_) => println!("Comando desconocido"),
    }
}
