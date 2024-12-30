create table if not exists mouse(
       id int PRIMARY KEY,
       model text not null,
       brand text not null,
       price float not null,
       color text not null
)