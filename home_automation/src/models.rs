// Importações necessárias para o trabalho com o ORM Diesel e Serde para serialização
use diesel::Queryable;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::schema::devices;
use crate::schema::users;

/*
Para saber mais sobre structs, consultar: https://rust-br.github.io/rust-book-pt-br/ch05-00-structs.html
Para saber mais sobre traits, consultar: https://rust-br.github.io/rust-book-pt-br/ch10-02-traits.html
*/


// Definição da estrutura Device que interage com a tabela 'devices' no banco de dados
#[derive(Insertable, Deserialize, Serialize, Queryable, Debug, Clone)]
#[table_name="devices"] // Especifica a tabela do banco de dados associada a esta estrutura
pub struct Device {
    pub device_id: String, // Campo que armazena o ID do dispositivo
    pub status: String,    // Campo que armazena o status do dispositivo
}

// Estrutura simples para transferência de dados de dispositivo, usada principalmente em operações de API
#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceData {
    pub device_id: String, // Campo que armazena o ID do dispositivo
    pub status: String,    // Campo que armazena o status do dispositivo
}

// Estrutura User que representa a tabela 'users' no banco de dados
#[derive(Insertable, Queryable, Serialize, Deserialize, Clone, Debug)]
#[table_name = "users"] // Especifica a tabela do banco de dados associada a esta estrutura
pub struct User {
    pub id: i32,          // Campo que armazena o identificador único do usuário
    pub username: String, // Campo que armazena o nome de usuário
    pub password: String, // Campo que armazena a senha do usuário
}

// Estrutura utilizada para receber dados de login através de uma API
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String, // Campo que recebe o nome de usuário
    pub password: String, // Campo que recebe a senha
}
