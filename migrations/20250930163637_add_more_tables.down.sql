alter table events
drop foreign key fk_events_device;

alter table events
drop column device_id;

drop table devices;
