create table events (
  id int primary key auto_increment,
  user_id varchar(255),
  post_id varchar(255) not null,
  kind enum ('view', 'like', 'share') not null,
  timestamp timestamp not null
);
