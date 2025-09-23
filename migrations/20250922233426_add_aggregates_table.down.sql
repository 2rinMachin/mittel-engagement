create table event_summaries (
  post_id varchar(255) primary key,
  views int not null,
  likes int not null,
  shares int not null
);
