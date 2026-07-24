/// Типы тегов (дополнительных статусов) автозаправочной станции.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Тег АЗС — характеристика, полученная из API, комментариев или вручную.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StationTag {
    /// Только по топливным картам.
    CardsOnly,
    /// Принимают наличные и карты.
    CashAndCards,
    /// Лимит отпуска топлива в литрах (20, 30, 40, 60).
    FuelLimit(u32),
    /// Без лимита.
    NoLimit,
    /// Станция закрыта.
    Closed,
    /// Можно заливать в канистры.
    CanisterOk,
    /// Только в бак (канистры запрещены).
    TankOnly,
    /// Чёт/нечет — проезд по номерам.
    EvenOdd,
    /// Большая очередь (без указания длины).
    BigQueue,
    /// Очередь 20+ машин.
    Queue20Plus,
    /// Очередь 50+ машин.
    Queue50Plus,
}

/// Источник получения тега.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagSource {
    /// Из API сайта (наивысший приоритет).
    Api,
    /// Извлечён из комментариев пользователей.
    Comment,
    /// Добавлен пользователем вручную.
    Manual,
}

/// Тег с метаданными — источником и временем обновления.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationTagInfo {
    pub tag: StationTag,
    pub source: TagSource,
    pub updated_at: DateTime<Utc>,
}
