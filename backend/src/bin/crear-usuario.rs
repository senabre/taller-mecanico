//! Orden de administración para dar de alta usuarios, restablecer su contraseña o desactivarlos
//! (spec-000, Assumptions: no hay pantalla de usuarios en la v1).
//!
//!   cargo run --bin crear-usuario -- --login luis --nombre "Luis" --rol mecanico
//!   cargo run --bin crear-usuario -- --login luis --desactivar
//!
//! La contraseña se lee de la entrada estándar, nunca de los argumentos, para que no quede en el
//! historial del terminal.

use std::io::{self, BufRead, IsTerminal, Write};
use std::process;

use sqlx::postgres::PgPoolOptions;
use taller::auth::{Rol, hash_contrasena};

struct Argumentos {
    login: String,
    nombre: Option<String>,
    rol: Option<Rol>,
    desactivar: bool,
}

#[tokio::main]
async fn main() {
    let args = leer_argumentos().unwrap_or_else(|mensaje| salir(&mensaje));
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| salir("Falta la variable de entorno DATABASE_URL"));
    let db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap_or_else(|e| salir(&format!("No se pudo conectar con la base de datos: {e}")));

    if args.desactivar {
        let mut tx = db.begin().await.unwrap_or_else(|e| salir(&e.to_string()));
        let filas = sqlx::query("UPDATE usuario SET activo = false WHERE login = $1")
            .bind(&args.login)
            .execute(&mut *tx)
            .await
            .unwrap_or_else(|e| salir(&e.to_string()))
            .rows_affected();
        if filas == 0 {
            salir(&format!("No existe el usuario {}", args.login));
        }
        sqlx::query(
            "DELETE FROM sesion WHERE usuario_id = (SELECT id FROM usuario WHERE login = $1)",
        )
        .bind(&args.login)
        .execute(&mut *tx)
        .await
        .unwrap_or_else(|e| salir(&e.to_string()));
        tx.commit().await.unwrap_or_else(|e| salir(&e.to_string()));
        println!("Usuario {} desactivado", args.login);
        return;
    }

    let (Some(nombre), Some(rol)) = (args.nombre, args.rol) else {
        salir("Faltan --nombre y --rol (o usa --desactivar)");
    };
    let contrasena = leer_contrasena().unwrap_or_else(|mensaje| salir(&mensaje));

    sqlx::query(
        "INSERT INTO usuario (nombre, login, hash_contrasena, rol, activo)
         VALUES ($1, $2, $3, $4, true)
         ON CONFLICT (login) DO UPDATE
            SET nombre = EXCLUDED.nombre,
                hash_contrasena = EXCLUDED.hash_contrasena,
                rol = EXCLUDED.rol,
                activo = true",
    )
    .bind(&nombre)
    .bind(&args.login)
    .bind(hash_contrasena(&contrasena))
    .bind(rol)
    .execute(&db)
    .await
    .unwrap_or_else(|e| salir(&format!("No se pudo guardar el usuario: {e}")));

    println!("Usuario {} guardado ({})", args.login, texto_rol(rol));
}

fn leer_argumentos() -> Result<Argumentos, String> {
    let mut login = None;
    let mut nombre = None;
    let mut rol = None;
    let mut desactivar = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--login" => login = args.next(),
            "--nombre" => nombre = args.next(),
            "--rol" => {
                let texto = args.next().unwrap_or_default();
                rol = Some(rol_desde_texto(&texto).ok_or_else(|| {
                    format!("Rol desconocido «{texto}»: usa recepcion, mecanico, almacen o jefe")
                })?);
            }
            "--desactivar" => desactivar = true,
            otro => return Err(format!("Argumento desconocido «{otro}»")),
        }
    }
    let login = login
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .ok_or("Falta --login")?;
    Ok(Argumentos {
        login,
        nombre,
        rol,
        desactivar,
    })
}

fn leer_contrasena() -> Result<String, String> {
    if io::stdin().is_terminal() {
        eprint!("Contraseña: ");
        io::stderr().flush().ok();
    }
    let mut linea = String::new();
    io::stdin()
        .lock()
        .read_line(&mut linea)
        .map_err(|e| format!("No se pudo leer la contraseña: {e}"))?;
    let contrasena = linea.trim_end_matches(['\r', '\n']).to_string();
    if contrasena.is_empty() {
        return Err("La contraseña no puede estar vacía".to_string());
    }
    Ok(contrasena)
}

fn rol_desde_texto(texto: &str) -> Option<Rol> {
    match texto {
        "recepcion" => Some(Rol::Recepcion),
        "mecanico" => Some(Rol::Mecanico),
        "almacen" => Some(Rol::Almacen),
        "jefe" => Some(Rol::Jefe),
        _ => None,
    }
}

fn texto_rol(rol: Rol) -> &'static str {
    match rol {
        Rol::Recepcion => "recepción",
        Rol::Mecanico => "mecánico",
        Rol::Almacen => "almacén",
        Rol::Jefe => "jefe de taller",
    }
}

fn salir(mensaje: &str) -> ! {
    eprintln!("{mensaje}");
    process::exit(1);
}
