// crate::database -> funções da camada de acesso a dados. Contém as operações no banco de dados
use crate::database::{get_device_by_id, update_device, create_device, get_user_by_username, create_user};
// crate::models -> estruturas de dados necessárias
use crate::models::{Device, DeviceData, User};
// diesel::PgConnection -> conexão ao banco de dados para poder realizar as operações
use diesel::PgConnection;

// Função que implementa lógica de atualizar um dispositivo
pub fn handle_update_device(conn: &PgConnection, data: DeviceData) -> Result<(), String> {
    // função "get_devicy_by_id" criada no arquivo "database.rs" para consultar o dispositivo pelo "device_id"
    match get_device_by_id(conn, &data.device_id) {
        // se o dispositivo foi encontrado, atualiza o dispositivo com novos dados
        Ok(_) => {
            // função "update_device" criado no arquivo "database.rs" para atualizar dispositivo
            update_device(conn, &data.device_id, &data.status).map_err(|e| e.to_string())?;
            Ok(())
        },
        // se o dispositivo não foi encontrado, cria um novo dispositivo com os dados fornecidos
        Err(_) => {
            let new_device = Device {
                device_id: data.device_id,
                status: data.status,
            };
            // função "update_device" criado no arquivo "database.rs" para criar dispositivo
            create_device(conn, &new_device).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}

// Função que implementa lógica de login
// SUGESTÃO: fazer um hash da senha para deixar mais seguro
pub fn handle_login_user(conn: &PgConnection, username: &str, password: &str) -> Result<User, String> {
    // busca as informações pelo nome do usuário pela função "get_user_by_username"
    let user = get_user_by_username(conn, username).map_err(|e| e.to_string())?;
    // senha confere
    if user.password == password {
        Ok(user)
        // senha não confere
    } else {
        Err("Invalid credentials".to_string())
    }
}

// Função que implementa lógica do cadastro
// SUGESTÃO: colcoar mais campos, como e-mail por exemplo
// SUGESTÃO: fazer um hash da senha para deixar mais seguro
pub fn handle_create_user(conn: &PgConnection, new_user: User) -> Result<(), String> {
    // função "create_user" criada no arquivo "database.rs" para criar usuário
    create_user(conn, &new_user).map_err(|e| e.to_string())?;
    Ok(())
}
