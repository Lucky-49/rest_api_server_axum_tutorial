use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>; //Будет использоваться для сокращения в функциях возвращающих Result, Error

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    //Указать название возвращаемых ошибок
    ConfigMissingEnv(&'static str), //ошибка загрузки переменной конфигурации
}