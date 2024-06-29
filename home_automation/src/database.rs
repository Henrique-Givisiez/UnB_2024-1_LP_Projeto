// DIESEL: Um ORM (Object-Relational Mapping) para Rust, que facilita a interação com bancos de dados SQL. Consultas, inserções, exclusões, etc..
// https://docs.rs/diesel/latest/diesel/
use diesel::prelude::*; // funcionalidas básicas do Diesel
// R2D2: Uma extensão do Diesel que fornece uma implementação do padrão de pool de conexões (connection pooling). Melhora o desempenho e a escalabilidade de aplicações que utilizam banco de dados SQL
// https://docs.rs/diesel/latest/diesel/r2d2/index.html
use diesel::r2d2::{self, ConnectionManager}; // gerenciar pool de conexões com o banco de dados 
use crate::models::{Device, User}; // estruturas de dados necessárias

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>; // define um alias para a pool de conexões

// Cria o gerenciador de conexão do banco de dados
// Configura e retorna um pool de conexões ao banco de dados baseado na database_url (que está definida nas variáveis de ambiente -> arquivo ".env").
pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Busca todos os dispositivos na tabela "devices", definida em "schema.rs"
pub fn get_all_devices(conn: &PgConnection) -> QueryResult<Vec<Device>> {
    use crate::schema::devices::dsl::*;
    devices.load::<Device>(conn)
}

// Busca um dispositivo específico pelo seu id na tabela "devices", definida em "schema.rs"
pub fn get_device_by_id(conn: &PgConnection, other_device_id: &str) -> QueryResult<Device> {
    use crate::schema::devices::dsl::*;
    devices.filter(device_id.eq(other_device_id)).first::<Device>(conn)
}

// Cria um novo registro (dispositivo) na tabela "devices", definida em "schema.rs"
pub fn create_device(conn: &PgConnection, new_device: &Device) -> QueryResult<usize> {
    use crate::schema::devices;
    diesel::insert_into(devices::table)
        .values(new_device)
        .execute(conn)
}

// Atualiza um dispositivo com base no id
pub fn update_device(conn: &PgConnection, other_device_id: &str, new_status: &str) -> QueryResult<usize> {
    use crate::schema::devices::dsl::*;
    diesel::update(devices.filter(device_id.eq(other_device_id)))
        .set(status.eq(new_status))
        .execute(conn)
}

// Deleta um dispositivo com base no id
pub fn delete_device(conn: &PgConnection, other_device_id: &str) -> QueryResult<usize> {
    use crate::schema::devices::dsl::*;
    diesel::delete(devices.filter(device_id.eq(other_device_id)))
        .execute(conn)
}

// Insere um usuário 
pub fn create_user(conn: &PgConnection, new_user: &User) -> QueryResult<usize> {
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)
}

// Busca um usuário pelo "username"
pub fn get_user_by_username(conn: &PgConnection, other_username: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    users.filter(username.eq(other_username)).first::<User>(conn)
}
