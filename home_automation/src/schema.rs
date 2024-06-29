// @generated automatically by Diesel CLI.
// Define a estrutura das tabelas devices e users no banco de dados e especifica como essas tabelas podem ser utilizadas em consultas SQL.
diesel::table! {                // Define nova tabela no esquema do Diesel
    devices (device_id) {       // Device_id -> chave primária
        device_id -> Varchar,
        status -> Varchar,      // status dos dispositivos como ativo, inativo, esperando, etc..
    }
}

diesel::table! {                // Define nova tabela no esquema do Diesel
    users (id) {                // id -> chave primária
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

// Permite que as tabelas especificadas não relacionadas sejam referenciadas na mesma consulta do Diesel.
// Por exemplo, uma operação do tipo "JOIN"
// Consultar documentação para mais detalhes
diesel ::allow_tables_to_appear_in_same_query!(
    devices,
    users,
);
