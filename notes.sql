insert into series(slug, title, cr_id)
values('epic-style', 'Epic Style', 'ZWJ23AJEB')
returning *

insert into season(series_id, slug, title_romaji, cr_id)
values(1, 'epic-style-2', 'Epikku Sutairu 2', 'VTJEB2028')
returning *

insert into episode(season_id, number, slug, title, cr_id)
values(1, 1, 'where-epic-goes', 'Where Epic Goes', 'ZWJ23SJEC')

insert into media(episode_id, host, quality, sub_lang, sub_burned, url, time)
values(1, 1, 'adaptive', 'en_us', true, 'https://pl.crunchyroll.com/fucku12', '2016-06-22 19:10:25-07')
returning *
