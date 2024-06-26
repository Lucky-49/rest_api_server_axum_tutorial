//Структура Ctx представляет контекст запроса или приложения
#[derive(Clone, Debug)] //Этот атрибут генерирует реализации методов Clone и Debug для структуры Ctx, позволяя ей быть клонируемой и выводимой при отладке
pub struct Ctx {
    user_id: u64,
}

///impl Ctx: определяются методы для структуры Ctx
impl Ctx {
    pub fn new(user_id: u64) -> Self { //Создает новый экземпляр Ctx с указанным идентификатором пользователя
        Self { user_id }
    }
}


impl Ctx {
    pub fn user_id(&self) -> u64 { //Этот метод позволяет получить идентификатор пользователя из структуры Ctx
        self.user_id
    }
}