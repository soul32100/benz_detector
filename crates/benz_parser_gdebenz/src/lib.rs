/// Парсер сайта gdebenz.ru — получение АЗС по радиусу и статусов топлива.

pub mod client;
pub mod dto;
pub mod mapper;
pub mod provider;

pub use provider::GdeBenzProvider;
