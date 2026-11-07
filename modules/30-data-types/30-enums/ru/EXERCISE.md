Готовое перечисление `Measurement` содержит варианты `Missing` и `Value(i32)`. Допишите `measurement(available: bool, value: i32) -> Measurement`. Если `available` равно `true`, верните вариант с переданным числом. Иначе верните `Missing`, независимо от числа.

Ноль является обычным допустимым числом. Например, `measurement(true, 0)` возвращает `Measurement::Value(0)`.
