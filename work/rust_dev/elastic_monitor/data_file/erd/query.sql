use sys_mon_neo;


SELECT * FROM NOSQL_CLUSTER_TYPES;



SELECT 
	nct.cluster_name
,	MAX(nct.user_id) AS user_id
,	MAX(nct.user_pw_enc) AS user_pw_enc
,	MAX(nct.system_version) AS system_version
,	MAX(nct.ssl_option) AS ssl_option	
,	MAX(CASE WHEN nlm.metric_type = 'shard' THEN nlm.limit_value ELSE 0 END) AS shard_limit
,	MAX(CASE WHEN nlm.metric_type = 'disk' THEN nlm.limit_value ELSE 0 END) AS disk_limit
,	MAX(CASE WHEN nlm.metric_type = 'cpu' THEN nlm.limit_value ELSE 0 END) AS cpu_limit
,	MAX(CASE WHEN nlm.metric_type = 'jvm' THEN nlm.limit_value ELSE 0 END) AS jvm_limit
FROM NOSQL_CLUSTER_TYPES nct
INNER JOIN NOSQL_MON_GROUP nmg ON nct.group_name = nmg.group_name
INNER JOIN NOSQL_LIMIT_METRICS nlm ON nlm.metric_name = nmg.metric_name
WHERE nct.system_type = 'ES'
GROUP BY nct.cluster_name;


select *
FROM NOSQL_CLUSTER_TYPES nct
INNER JOIN NOSQL_MON_GROUP nmg ON nct.group_name = nmg.group_name
INNER JOIN NOSQL_LIMIT_METRICS nlm ON nlm.metric_name = nmg.metric_name
WHERE nct.system_type = 'ES';


select * FROM NOSQL_CLUSTER_TYPES;




update NOSQL_CLUSTER_TYPES
set group_name = 'common_es_metric_set'
where system_type = 'ES'
and cluster_name in ('nd.elk.logrecord.dev', 'product-history-cluster');


SELECT * FROM NOSQL_HOST_INFO;
SELECT * FROM NOSQL_LIMIT_METRICS;
SELECT * FROM NOSQL_MON_GROUP;
SELECT * FROM NOSQL_CLUSTER_TYPES;




        
CREATE TABLE NOSQL_APPLIED_INFOS
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  metric_type  VARCHAR(25)  NOT NULL,
  metric_name  VARCHAR(100) NOT NULL,
  reg_dt       DATETIME     NOT NULL,
  chg_dt       DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, metric_type)
);

CREATE TABLE NOSQL_CLUSTER_TYPES
(
  system_type    VARCHAR(25)    NOT NULL,
  cluster_name   VARCHAR(100)   NOT NULL,
  user_id        VARCHAR(100)   NULL    ,
  user_pw_enc    VARBINARY(255) NULL    ,
  system_version VARCHAR(25)    NULL    ,
  ssl_option     INT            NOT NULL,
  reg_dt         DATETIME       NOT NULL,
  chg_dt         DATETIME       NULL    ,
  group_name     VARCHAR(100)   NULL    ,
  PRIMARY KEY (system_type, cluster_name)
);

CREATE TABLE NOSQL_HOST_INFO
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  host_ip      VARCHAR(100) NOT NULL,
  host_port    INT          NOT NULL,
  reg_dt       DATETIME     NOT NULL,
  chg_dt       DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, host_ip, host_port)
);

CREATE TABLE NOSQL_INDEX_SCHEDULE
(
  system_type   VARCHAR(25)  NOT NULL,
  cluster_name  VARCHAR(100) NOT NULL,
  index_pattern VARCHAR(200) NOT NULL,
  presv_period  INT          NOT NULL,
  reg_dt        DATETIME     NOT NULL,
  chg_dt        DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, index_pattern)
);

CREATE TABLE NOSQL_LIMIT_METRICS
(
  metric_name VARCHAR(100) NOT NULL,
  metric_type VARCHAR(25)  NOT NULL,
  limit_value DOUBLE       NOT NULL,
  reg_dt      DATETIME     NOT NULL,
  chg_dt      DATETIME     NULL    ,
  PRIMARY KEY (metric_name)
);

CREATE TABLE NOSQL_MON_GROUP
(
  group_name  VARCHAR(100) NOT NULL,
  metric_name VARCHAR(100) NOT NULL,
  apply_yn    INT          NOT NULL,
  reg_dt      DATETIME     NOT NULL,
  chg_dt      DATETIME     NULL    ,
  PRIMARY KEY (group_name, metric_name)
);

ALTER TABLE NOSQL_HOST_INFO
  ADD CONSTRAINT FK_NOSQL_CLUSTER_TYPES_TO_NOSQL_HOST_INFO
    FOREIGN KEY (system_type, cluster_name)
    REFERENCES NOSQL_CLUSTER_TYPES (system_type, cluster_name);

ALTER TABLE NOSQL_INDEX_SCHEDULE
  ADD CONSTRAINT FK_NOSQL_CLUSTER_TYPES_TO_NOSQL_INDEX_SCHEDULE
    FOREIGN KEY (system_type, cluster_name)
    REFERENCES NOSQL_CLUSTER_TYPES (system_type, cluster_name);

ALTER TABLE NOSQL_CLUSTER_TYPES
  ADD CONSTRAINT FK_NOSQL_MON_GROUP_TO_NOSQL_CLUSTER_TYPES
    FOREIGN KEY (group_name)
    REFERENCES NOSQL_MON_GROUP (group_name, metric_name);

ALTER TABLE NOSQL_LIMIT_METRICS
  ADD CONSTRAINT FK_NOSQL_MON_GROUP_TO_NOSQL_LIMIT_METRICS
    FOREIGN KEY (metric_name)
    REFERENCES NOSQL_MON_GROUP (metric_name);

        
      








drop table NOSQL_CLUSTER_TYPES;
drop table NOSQL_HOST_INFO;
drop table NOSQL_LIMIT_METRIC;



# wmp-user-super
# Sh@#156452


# es-infra-elastic

select * from NOSQL_CLUSTER_TYPES;

update NOSQL_CLUSTER_TYPES
set group_name = 'test_es_metric_set'
where system_type = 'ES';


select * from NOSQL_MON_GROUP;

select * from NOSQL_LIMIT_METRIC;
select * from NOSQL_LIMIT_METRICS;

insert into NOSQL_LIMIT_METRICS values ('cpu_90', 'cpu',90.0,NOW(),null);
insert into NOSQL_LIMIT_METRICS values ('disk_80', 'disk',80.0,NOW(),null);
insert into NOSQL_LIMIT_METRICS values ('jvm_90', 'jvm',90.0,NOW(),null);
insert into NOSQL_LIMIT_METRICS values ('es_shard_80', 'shard',80.0,NOW(),null);

insert into NOSQL_LIMIT_METRICS values ('cpu_20', 'cpu',20.0,NOW(),null);
insert into NOSQL_LIMIT_METRICS values ('disk_20', 'disk',20.0,NOW(),null);
insert into NOSQL_LIMIT_METRICS values ('jvm_20', 'jvm',20.0,NOW(),null);

select * from NOSQL_MON_GROUP;

insert into NOSQL_MON_GROUP values ('common_es_metric_set', 'cpu_90', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('common_es_metric_set', 'disk_80', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('common_es_metric_set', 'jvm_90', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('common_es_metric_set', 'es_shard_80', 1, NOW(), null);

insert into NOSQL_MON_GROUP values ('test_es_metric_set', 'cpu_20', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('test_es_metric_set', 'disk_20', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('test_es_metric_set', 'jvm_20', 1, NOW(), null);
insert into NOSQL_MON_GROUP values ('test_es_metric_set', 'es_shard_80', 1, NOW(), null);


select * from NOSQL_MON_GROUP;


alter table NOSQL_CLUSTER_TYPES add column group_name varchar(100);


select * from NOSQL_HOST_INFO;
select * from NOSQL_APPLIED_INFOS;


insert into NOSQL_APPLIED_INFOS values ('ES','es_infra_monitoring','disk','common_es_disk', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','es_infra_monitoring','cpu','common_es_cpu', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','es_infra_monitoring','jvm','common_es_jvm', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','es_infra_monitoring','shard','common_es_shard', NOW(), null);

insert into NOSQL_APPLIED_INFOS values ('ES','nd.elk.dev','disk','common_es_disk', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','nd.elk.dev','cpu','common_es_cpu', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','nd.elk.dev','jvm','common_es_jvm', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','nd.elk.dev','shard','common_es_shard', NOW(), null);

insert into NOSQL_APPLIED_INFOS values ('ES','ad-es-dev','disk','common_es_disk', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','ad-es-dev','cpu','common_es_cpu', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','ad-es-dev','jvm','common_es_jvm', NOW(), null);
insert into NOSQL_APPLIED_INFOS values ('ES','ad-es-dev','shard','common_es_shard', NOW(), null);


insert into NOSQL_LIMIT_METRIC values ('common_es_disk', 'ES', 'disk', 80.0, NOW(), null);
insert into NOSQL_LIMIT_METRIC values ('common_es_cpu', 'ES', 'cpu', 90.0, NOW(), null);
insert into NOSQL_LIMIT_METRIC values ('common_es_jvm', 'ES', 'jvm', 90.0, NOW(), null);

insert into NOSQL_LIMIT_METRIC values ('30_es_disk', 'ES', 'jvm', 30.0, NOW(), null);
insert into NOSQL_LIMIT_METRIC values ('50_es_jvm', 'ES', 'jvm', 50.0, NOW(), null);
insert into NOSQL_LIMIT_METRIC values ('10_es_cpu', 'ES', 'cpu', 10.0, NOW(), null);


insert into NOSQL_LIMIT_METRIC values ('common_es_shard', 'ES', 'shard', 80.0, NOW(), null);

select * from NOSQL_LIMIT_METRIC;

select * from NOSQL_APPLIED_INFOS;

update NOSQL_APPLIED_INFOS
set metric_name = '30_es_disk'
where system_type = 'ES' and cluster_name = 'es_infra_monitoring' and metric_type = 'disk';

select * from NOSQL_LIMIT_METRIC;

delete from NOSQL_LIMIT_METRIC where metric_name <> 'common_es_disk';



select 
* 
from NOSQL_CLUSTER_TYPES nct
inner join NOSQL_HOST_INFO nhi on nct.system_type = nhi.system_type and nct.cluster_name = nhi.cluster_name
order by nct.cluster_name;


select
	nct.cluster_name
,	nct.user_id
,	nct.user_pw_enc
,	nct.system_version
,	nct.ssl_option
,	nlm.metric_type
,	nlm.limit_value
from NOSQL_CLUSTER_TYPES nct
inner join NOSQL_APPLIED_INFOS nai on nct.system_type = nai.system_type and nct.cluster_name = nai.cluster_name
inner join NOSQL_LIMIT_METRIC nlm on nai.system_type = nlm.system_type and nai.metric_type = nlm.metric_type and nai.metric_name = nlm.metric_name
order by nct.cluster_name;


select
	nct.cluster_name
,	max(nct.user_id) as user_id
,	max(nct.user_pw_enc) as user_pw_enc
,	max(nct.system_version) as system_version
,	max(nct.ssl_option) as ssl_option
from NOSQL_CLUSTER_TYPES nct
inner join NOSQL_APPLIED_INFOS nai on nct.system_type = nai.system_type and nct.cluster_name = nai.cluster_name
inner join NOSQL_LIMIT_METRIC nlm on nai.system_type = nlm.system_type and nai.metric_type = nlm.metric_type and nai.metric_name = nlm.metric_name
group by nct.cluster_name
order by nct.cluster_name;


select
	nct.cluster_name
,	max(nct.user_id) as user_id
,	max(nct.user_pw_enc) as user_pw_enc
,	max(nct.system_version) as system_version
,	max(nct.ssl_option) as ssl_option
,	max(CASE WHEN nai.metric_type = 'shard' THEN nlm.limit_value ELSE 0 END) as shard_limit
,	max(CASE WHEN nai.metric_type = 'disk' THEN nlm.limit_value ELSE 0 END) as disk_limit
,	max(CASE WHEN nai.metric_type = 'cpu' THEN nlm.limit_value ELSE 0 END) as cpu_limit
,	max(CASE WHEN nai.metric_type = 'jvm' THEN nlm.limit_value ELSE 0 END) as jvm_limit
from NOSQL_CLUSTER_TYPES nct
inner join NOSQL_APPLIED_INFOS nai on nct.system_type = nai.system_type and nct.cluster_name = nai.cluster_name
inner join NOSQL_LIMIT_METRIC nlm on nai.system_type = nlm.system_type and nai.metric_type = nlm.metric_type and nai.metric_name = nlm.metric_name
where nct.system_type = 'ES'
group by nct.cluster_name;
#order by nct.cluster_name;

select * from NOSQL_APPLIED_INFOS;


SELECT
	nct.cluster_name
,	MAX(nct.user_id) AS user_id
,	MAX(nct.user_pw_enc) AS user_pw_enc
,	MAX(nct.system_version) AS system_version
,	MAX(nct.ssl_option) AS ssl_option
-- ,	max(CASE WHEN nai.metric_type = 'shard' THEN nlm.limit_value ELSE 0 END) as shard_limit
-- ,	max(CASE WHEN nai.metric_type = 'disk' THEN nlm.limit_value ELSE 0 END) as disk_limit
-- ,	max(CASE WHEN nai.metric_type = 'cpu' THEN nlm.limit_value ELSE 0 END) as cpu_limit
-- ,	max(CASE WHEN nai.metric_type = 'jvm' THEN nlm.limit_value ELSE 0 END) as jvm_limit
FROM NOSQL_CLUSTER_TYPES nct
INNER JOIN NOSQL_MON_GROUP nmg ON nct.group_name = nmg.group_name
-- inner join NOSQL_APPLIED_INFOS nai on nct.system_type = nai.system_type and nct.cluster_name = nai.cluster_name
-- inner join NOSQL_LIMIT_METRIC nlm on nai.system_type = nlm.system_type and nai.metric_type = nlm.metric_type and nai.metric_name = nlm.metric_name
where nct.system_type = 'ES'
group by nct.cluster_name;



SELECT 
	nct.cluster_name
,	MAX(nct.user_id) AS user_id
,	MAX(nct.user_pw_enc) AS user_pw_enc
,	MAX(nct.system_version) AS system_version
,	MAX(nct.ssl_option) AS ssl_option	
,	max(CASE WHEN nlm.metric_type = 'shard' THEN nlm.limit_value ELSE 0 END) as shard_limit
,	max(CASE WHEN nlm.metric_type = 'disk' THEN nlm.limit_value ELSE 0 END) as disk_limit
,	max(CASE WHEN nlm.metric_type = 'cpu' THEN nlm.limit_value ELSE 0 END) as cpu_limit
,	max(CASE WHEN nlm.metric_type = 'jvm' THEN nlm.limit_value ELSE 0 END) as jvm_limit
FROM NOSQL_CLUSTER_TYPES nct
INNER JOIN NOSQL_MON_GROUP nmg ON nct.group_name = nmg.group_name
INNER JOIN NOSQL_LIMIT_METRICS nlm ON nlm.metric_name = nmg.metric_name
where nct.system_type = 'ES'
group by nct.cluster_name;


#where nai.metric_type = 'cpu';


CREATE TABLE NOSQL_MON_GROUP
(
  group_name  VARCHAR(100) NOT NULL,
  metric_name VARCHAR(100) NOT NULL,
  apply_yn    INT          NOT NULL,
  reg_dt      DATETIME     NOT NULL,
  chg_dt      DATETIME     NULL    ,
  PRIMARY KEY (group_name, metric_name)
);



CREATE TABLE NOSQL_APPLIED_INFOS
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  metric_type  VARCHAR(25)  NOT NULL,
  metric_name  VARCHAR(100) NOT NULL,
  reg_dt       DATETIME     NOT NULL,
  chg_dt       DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, metric_type)
);



select * from NOSQL_CLUSTER_TYPE;

CREATE TABLE NOSQL_CLUSTER_TYPES
(
  system_type    VARCHAR(25)    NOT NULL,
  cluster_name   VARCHAR(100)   NOT NULL,
  user_id        VARCHAR(100)   NULL    ,
  user_pw_enc    VARBINARY(255) NULL    ,
  system_version VARCHAR(25)    NULL    ,
  ssl_option     INT            NOT NULL,
  reg_dt         DATETIME       NOT NULL,
  chg_dt         DATETIME       NULL    ,
  PRIMARY KEY (system_type, cluster_name)
);

CREATE TABLE NOSQL_HOST_INFO
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  host_ip      VARCHAR(100) NOT NULL,
  host_port    INT          NOT NULL,
  reg_dt       DATETIME     NOT NULL,
  chg_dt       DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, host_ip, host_port)
);

CREATE TABLE NOSQL_INDEX_SCHEDULE
(
  system_type   VARCHAR(25)  NOT NULL,
  cluster_name  VARCHAR(100) NOT NULL,
  index_pattern VARCHAR(200) NOT NULL,
  presv_period  INT          NOT NULL,
  reg_dt        DATETIME     NOT NULL,
  chg_dt        DATETIME     NULL    ,
  PRIMARY KEY (system_type, cluster_name, index_pattern)
);

CREATE TABLE NOSQL_LIMIT_METRIC
(
  metric_name VARCHAR(100) NOT NULL,
  system_type VARCHAR(25)  NOT NULL,
  metric_type VARCHAR(25)  NOT NULL,
  limit_value DOUBLE       NOT NULL,
  reg_dt      DATETIME     NOT NULL,
  chg_dt      DATETIME     NULL    ,
  PRIMARY KEY (metric_name, system_type, metric_type)
);



CREATE TABLE NOSQL_LIMIT_METRICS
(
  metric_name VARCHAR(100) NOT NULL,
  metric_type VARCHAR(25)  NOT NULL,
  limit_value DOUBLE       NOT NULL,
  reg_dt      DATETIME     NOT NULL,
  chg_dt      DATETIME     NULL    ,
  PRIMARY KEY (metric_name)
);




################################################################################################################################################################     






CREATE TABLE NOSQL_CLUSTER_TYPES
(
  system_type    VARCHAR(25)    NOT NULL,
  cluster_name   VARCHAR(100)   NOT NULL,
  user_id        VARCHAR(100)   NULL    ,
  user_pw_enc    VARBINARY(255) NULL    ,
  system_version VARCHAR(25)    NULL    ,
  ssl_option	 INT 			NOT NULL,
  PRIMARY KEY (system_type, cluster_name)
);

CREATE TABLE NOSQL_HOST_INFO
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  host_ip      VARCHAR(100) NOT NULL,
  host_port    INT          NOT NULL,
  PRIMARY KEY (system_type, cluster_name, host_ip, host_port)
);


CREATE TABLE NOSQL_INDEX_SCHEDULE
(
  system_type   VARCHAR(25)  NOT NULL,
  cluster_name  VARCHAR(100) NOT NULL,
  index_pattern VARCHAR(200) NOT NULL,
  presv_period  INT          NOT NULL,
  PRIMARY KEY (system_type, cluster_name, index_pattern)
);


        
CREATE TABLE NOSQL_APPLIED_INFOS
(
  system_type  VARCHAR(25)  NOT NULL,
  cluster_name VARCHAR(100) NOT NULL,
  metric_name  VARCHAR(100) NOT NULL,
  
  PRIMARY KEY (system_type, cluster_name, metric_name)
);


CREATE TABLE NOSQL_LIMIT_METRIC
(
  metric_name VARCHAR(100) NOT NULL,
  mtric_type  VARCHAR(25)  NOT NULL,
  limit_value DOUBLE       NOT NULL,
  
  PRIMARY KEY (metric_name)
);


insert into NOSQL_LIMIT_METRIC values ('common_es_disk', 'disk', 80.0);
insert into NOSQL_LIMIT_METRIC values ('common_es_disk', 'disk', 80.0);
insert into NOSQL_LIMIT_METRIC values ('common_es_disk', 'disk', 80.0);



        
      


select * from NOSQL_CLUSTER_TYPES;

## 1
CREATE TABLE NOSQL_CLUSTER_TYPES 
(
	system_type varchar(25) not null,
    cluster_name varchar(100) not null,
    user_id varchar(100),
    user_pw_enc varbinary(255),
    system_version varchar(25),
    
    PRIMARY KEY (system_type, cluster_name)
);

select * from NOSQL_CLUSTER_TYPES;


## 2
CREATE TABLE NOSQL_INDEX_SCHEDULE
(
	system_type varchar(25) not null,
    cluster_name varchar(100) not null,
    index_pattern varchar(200) not null,
    presv_period int not null,

	primary key (system_type, cluster_name, index_pattern)
);


select * from NOSQL_INDEX_SCHEDULE;



## 3
CREATE TABLE NOSQL_HOST_INFO
(
	system_type varchar(25) not null,
    cluster_name varchar(100) not null,
    host_ip varchar(100) not null,
    host_port int not null,
    
    primary key(system_type, cluster_name, host_ip, host_port)
);


select * from NOSQL_HOST_INFO;



## 4
CREATE TABLE NOSQL_LIMIT_METRIC
(
	system_type varchar(25) not null,
    metric_type varchar(25) not null,
    limit_value double not null
)







CREATE TABLE TEST_USER (
  id INT AUTO_INCREMENT,
  name VARCHAR(100),
  email VARCHAR(100),
  PRIMARY KEY (id)
);

select * from NOSQL_CLUSTER_TYPE where system_type='ES';




CREATE TABLE NOSQL_CLUSTER_TYPE
(
	system_type VARCHAR(25),
    cluster_name VARCHAR(100),
    user_id varchar(100) NULL,
    user_pw varchar(200) NULL,
    system_version varchar(25) NULL,
    ssl_option int NULL,
    
    PRIMARY KEY (system_type, cluster_name)
);

insert into NOSQL_CLUSTER_TYPE values ('ES','vertical_es','elastic','CfanZsBNpRzBQaFAcvXn',7,0);


SELECT * FROM NOSQL_CLUSTER_TYPE;

SELECT * FROM NOSQL_HOST_INFOS WHERE cluster_name = 'vertical_es';

insert into NOSQL_HOST_INFOS values ('ES', 'vertical_es', '10.107.11.66', 9200);
insert into NOSQL_HOST_INFOS values ('ES', 'vertical_es', '10.107.11.56', 9200);
insert into NOSQL_HOST_INFOS values ('ES', 'vertical_es', '10.107.11.59', 9200);
insert into NOSQL_HOST_INFOS values ('ES', 'vertical_es', '10.107.11.71', 9200);
insert into NOSQL_HOST_INFOS values ('ES', 'vertical_es', '10.107.11.75', 9200);


select * from ES_INDEX_SCHEDULE;

CREATE TABLE ES_INDEX_SCHEDULE
(
	cluster_name VARCHAR(100),
    
    PRIMARY KEY (cluster_name)
);

select * from ES_INDEX_SCHEDULE;

delete from ES_INDEX_SCHEDULE where cluster_name = 'test_es';

insert into ES_INDEX_SCHEDULE values ('vertical_es');
insert into ES_INDEX_SCHEDULE values ('nd.elk.dev');

-- SELECT
-- 	nc.cluster_name
-- ,	nc.user_id
-- ,	nc.user_pw
-- ,	ni.host_ip
-- ,	ni.host_port
-- INNER JOIN NOSQL_CLUSTER_TYPE nc ON ei.cluster_name = nc.cluster_name
-- INNER JOIN NOSQL_HOST_INFOS ni ON ei.cluster_name = ni.cluster_name
-- WHERE nc.system_type = 'ES';

SELECT
	host_ip
,	host_port
FROM NOSQL_HOST_INFOS
WHERE cluster_name = 'vertical_es' AND system_type ='ES';



select * from NOSQL_CLUSTER_TYPE;


CREATE TABLE NOSQL_HOST_INFOS
(
	system_type VARCHAR(25),
    cluster_name VARCHAR(100),
    host_ip VARCHAR(100) NOT NULL,
    host_port int NOT NULL,
    
    PRIMARY KEY (system_type, cluster_name, host_ip, host_port)
);





select 
	nct.cluster_name
,	nct.user_id
,	nct.user_pw
,	nct.system_version
,	nct.ssl_option	
from NOSQL_CLUSTER_TYPE as nct
inner join NOSQL_HOST_INFOS as nhi on nct.cluster_name = nhi.cluster_name and nct.system_type = nhi.system_type
where n.cluster_name = 'product-history-cluster';

#order by n.cluster_name;


select * from NOSQL_CLUSTER_TYPE;


select 
	cluster_name
,	user_id
,	user_pw
,	system_version
,	ssl_option
from NOSQL_CLUSTER_TYPE where system_type = 'ES'; 


select 
	host_ip 
,	host_port
from NOSQL_HOST_INFOS 
where system_type = 'ES'
AND cluster_name = 'product-history-cluster';



select * from NOSQL_CLUSTER_TYPE where system_type='ES';

select 
* 
from NOSQL_CLUSTER_TYPE t
inner join NOSQL_HOST_INFOS i on t.cluster_name = i.cluster_name
where t.system_type = 'ES'
and host_ip = '10.107.13.132';


ALTER TABLE NOSQL_CLUSTER_TYPE ADD COLUMN schedule_yn TINYINT(1) NOT NULL DEFAULT 0;

alter table NOSQL_CLUSTER_TYPE drop column schedule_yn;


select * from NOSQL_HOST_INFOS;

select distinct(cluster_name),count(*) from NOSQL_HOST_INFOS
where system_type = 'ES'
group by cluster_name;






