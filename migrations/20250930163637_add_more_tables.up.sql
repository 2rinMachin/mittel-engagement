create table devices (
  id int primary key auto_increment,
  os varchar(50),
  browser varchar(50),
  screen_resolution varchar(50),
  language varchar(50),
  unique (os, browser, screen_resolution, language)
);

alter table events
add column device_id int;

alter table events add constraint fk_events_device foreign key (device_id) references devices (id);
