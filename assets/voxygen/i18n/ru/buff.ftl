## Regeneration

buff-heal = Лечение
    .desc = Постепенное восстановление здоровья
    .stat = Восстанавливает { $str_total } здоровья

## Potion

buff-potion = Зелье
    .desc = Питьё...

## Saturation

buff-saturation = Насыщение
    .desc = Восстановление здоровья за счет расходных материалов.

## Campfire

buff-campfire_heal = Исцеление у Костра
    .desc = Отдых у костра лечит { $rate }% в секунду.

## Energy Regen

buff-energy_regen = Восстановление Энергии
    .desc = Ускоренное восстановление энергии.
    .stat = Восстанавливает { $str_total } энергии.

## Health Increase

buff-increase_max_health = Повышение Максимального Здоровья
    .desc = Увеличение лимита здоровья.
    .stat =
        Повышает максимум здоровья
        на { $strength }.

## Energy Increase

buff-increase_max_energy = Повышение максимальной энергии
    .desc = Увеличение лимита энергии.
    .stat =
        Повышает максимум энергии
        на { $strength }.

## Invulnerability

buff-invulnerability = Неуязвимость
    .desc = Вы не можете получить урон от атак.
    .stat = Дарует неуязвимость.

## Protection Ward

buff-protectingward = Защитная Аура
    .desc = Вы в некоторой степени защищены от атак.

## Frenzied

buff-frenzied = Бешенство
    .desc = Кровь течёт быстрее, ускоряя ваше движение и понемногу исцеляя вас.

## Haste

buff-hastened = Ускорение
    .desc = Скорость передвижения и атак повышена.

## Bleeding

buff-bleed = Кровотечение
    .desc = Наносит регулярный урон.

## Curse

buff-cursed = Проклятие
    .desc = Вас прокляли.

## Burning

buff-burn = В Огне
    .desc = Вы горите живьём

## Crippled

buff-crippled = Увечье
    .desc = Ваше движение затруднено, так как ваши ноги сильно травмированы.

## Freeze

buff-frozen = Обморожение
    .desc = Скорость движения и атак снижена.

## Wet

buff-wet = Мокрый
    .desc = Ваши ноги не слушаются, остановка затруднена.

## Ensnared

buff-ensnared = Ловушка
    .desc = Лоза опутывает ваши ноги, затрудняя движение.

## Fortitude

buff-fortitude = Стойкость
    .desc = Вы можете противостоять ошеломляющим ударам, и чем больше вы получаете урона, тем легче ошеломляете других.

## Parried

buff-parried = Парирован
    .desc = Вашу атаку отразили, ваше восстановление замедлено.

## Potion sickness

buff-potionsickness = Отравление зельем
    .desc = Зелья исцеляют вас меньше, если вы недавно уже употребили другое зелье.
    .stat =
        Уменьшает исцеление от
        последующих зелий на { $strength }%.

## Reckless

buff-reckless = Безрассудный
    .desc = Ваши атаки стали сильнее, однако вы стали открытым для вражеских атак.

## Util

buff-text-over_seconds =
    более { $dur_secs ->
        [one] секунды
       *[other] { $dur_secs } секунд
    }
buff-text-for_seconds =
    на { $dur_secs ->
        [one] { $dur_secs } секунду
        [few] { $dur_secs } секунды
        [many] { $dur_secs } секунд
       *[other] { $dur_secs } секунд
    }
buff-remove = Нажмите, чтобы удалить
# Imminent Critical
buff-imminentcritical = Неотвратимый Критический Удар
    .desc = Ваша следующая атака нанесет противнику критический удар.
buff-mysterious = Таинственный Эффект
# Polymorped
buff-polymorphed = Полиморф
    .desc = Ваше тело меняет форму.
# Fury
buff-fury = Ярость
    .desc = Благодаря вашей ярости ваши удары генерируют больше комбо.
# Frigid
buff-frigid = Холод
    .desc = Заморозьте своих врагов.
# Berserk
buff-berserk = Берсерк
    .desc = Вы находитесь в состоянии ярости, в результате чего ваши атаки становятся более мощными и быстрыми, а скорость увеличивается. Однако при этом снижается способность к защите.
# Bloodfeast
buff-bloodfeast = Кровавый пир
    .desc = Вы восстанавливаете жизнь, атакуя истекающих кровью врагов.
# Salamander's Aspect
buff-salamanderaspect = Аспект Саламандры
    .desc = Вы не горите и быстро перемещаетесь по лаве.
# Agility
buff-agility = Ловкость
    .desc = Вы двигаетесь быстрее, но наносите меньше урона и получаете больше повреждений.
    .stat =
        Увеличивает скорость передвижения на { $strength }%.
        В свою очередь, ваши сила атаки и защита резко снижаются.
buff-poisoned = Яд
    .desc = Вы чувствуете, что ваша жизнь угасает...
buff-lifesteal = Вампиризм
    .desc = Высасывает жизни врагов.
buff-sunderer = Разрушитель
    .desc = Ваши атаки способны пробить защиту противника и пополнить запасы энергии.
buff-defiance = Вызов
    .desc = Вы можете выдерживать более мощные и ошеломляющие удары и генерировать комбо, получив удар, однако вы медленнее.
buff-heatstroke = Тепловой удар
    .desc = Вы перегрелись и теперь страдаете от теплового удара. Восстановление энергии и скорость передвижения снижены. Остынь.
buff-rooted = Корни
    .desc = Вы застряли на месте и не можете двигаться.
buff-winded = Обветренный
    .desc = Вы едва можете дышать, что мешает вам восстанавливать энергию и быстро двигаться.
buff-staggered = Ошеломление
    .desc = Вы потеряли равновесие и стали более восприимчивы к сильным атакам.
buff-tenacity = Стойкость
    .desc = Вы не только способны отражать более тяжелые атаки, но и заряжаетесь их энергией. Вы также стали медленнее.
buff-scornfultaunt = Презрительная Насмешка
    .desc = Вы презрительно насмехаетесь над своими врагами, получая прибавку к стойкости и выносливости. Однако ваша смерть усилит вашего убийцу.
buff-concussion = Сотрясение
    .desc = Вы получили сильный удар по голове, и вам трудно сосредоточиться, что мешает вам использовать некоторые сложные атаки.
buff-resilience = Resilience
    .desc = После того, как вы перенесли изнуряющую атаку, вы становитесь более устойчивы к будущим калечащим эффектам.
