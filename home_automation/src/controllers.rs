// crate::database -> funções da camada de acesso a dados. Contém as operações no banco de dados
use crate::database::{get_device_by_id, update_device, create_device, get_user_by_email, create_user};
// crate::models -> estruturas de dados necessárias
use crate::models::{ReadDevice, User, NewUser, NewDevice};
// diesel::PgConnection -> conexão ao banco de dados para poder realizar as operações
use diesel::PgConnection;
// bcrypt: biblioteca para fazer hash da senha
use bcrypt::{hash, verify, DEFAULT_COST};

// Função que implementa lógica de criar um dispositivo
pub fn handle_create_device(conn: &PgConnection, data: NewDevice) -> Result<(), String >{
    create_device(conn, &data).map_err(|e| e.to_string())?;
    Ok(())
}

// Função que implementa lógica de atualizar um dispositivo
pub fn handle_update_device(conn: &PgConnection, data: ReadDevice) -> Result<(), String> {
    // Se ID foi fornecido, procede com a busca e atualização
    match get_device_by_id(conn, &data.id) {
        Ok(_) => {
            // Se o dispositivo foi encontrado, tenta atualizar
            update_device(conn, &data.id, &data.status).map_err(|e| e.to_string())?;
            Ok(())
        },
        Err(_) => Err("Dispositivo não encontrado".to_string())
    }
}

// Função que implementa lógica de login
// SUGESTÃO: fazer um hash da senha para deixar mais seguro
pub fn handle_login_user(conn: &PgConnection, email: &str, password: &str) -> Result<User, String> {
    // busca as informações pelo nome do usuário pela função "get_user_by_username"
    let user = get_user_by_email(conn, email).map_err(|e| e.to_string())?;

    // Utilizaçao da função verify para verificar se a senha fornecida corresponde ao hash armazenado
    if verify(password, &user.password).map_err(|e| e.to_string())? {
        Ok(user) // SENHA CONFRE
    } else {
        Err("Credencias Invalidas".to_string()) // SENHA NÃO CONFERE
    }
}

// Função que implementa lógica do cadastro
// SUGESTÃO: colcoar mais campos, como e-mail por exemplo
// SUGESTÃO: fazer um hash da senha para deixar mais seguro
pub fn handle_create_user(conn: &PgConnection, mut new_user: NewUser) -> Result<(), String> {
    // DEFAULT_COST: complexidade do hash da senha -> neste caso, o custo da complexidade é padrão
    match hash(&new_user.password, DEFAULT_COST) {  // Função "hash" que irá fazer o hash da senha
        Ok(hashed_password) => {
            new_user.password = hashed_password;
        },
        Err(e) => return Err(e.to_string())
    }
    // função "create_user" criada no arquivo "database.rs" para criar usuário
    create_user(conn, new_user).map_err(|e| e.to_string())?;
    Ok(())
}
