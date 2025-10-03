create table devices (
  id int primary key auto_increment,
  os varchar(50) not null,
  browser varchar(50) not null,
  screen_resolution varchar(50) not null,
  language varchar(50) not null,
  unique (os, browser, screen_resolution, language)
);

alter table events
add column device_id int;

alter table events add constraint fk_events_device foreign key (device_id) references devices (id);
