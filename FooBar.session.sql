select * from games;

insert into players
(id, display_name, token_hash)
values ("123", "matt", "tokentoken");

select * from players;

insert into moves
(game_id, move_index, render)
values ("abc", 0, "FOOBAR")