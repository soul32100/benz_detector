/// Парсер сайта benzest.ru — получение АЗС, цен, тегов и комментариев.

pub mod client;
pub mod dto;
pub mod mapper;
pub mod provider;

pub use provider::BenzEstProvider;
