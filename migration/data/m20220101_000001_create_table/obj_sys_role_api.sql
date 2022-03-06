/*
SQLyog Ultimate
MySQL - 10.6.4-MariaDB : Database - poem-admin-demo
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
/*Data for the table `sys_role_api` */

insert  into `sys_role_api`(`id`,`role_id`,`api`,`method`,`created_by`,`created_at`) values 
('00VS1JOLQ716C5C21U731BHCKH','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5E21U715SH0QI','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-basic','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5G21U72V92G77','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-basic-menu','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5I21U7234K67K','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5K21U711GFOLQ','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5M21U71R41QMQ','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5O21U722EKUQM','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5Q21U73C7H1NS','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5S21U71R3Q448','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-dict','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C5U21U728JN2JD','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6021U73HD4RUN','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/get_all','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6221U701Q43IJ','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6421U71R63ME2','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6621U705H8M3M','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6821U72RADEBS','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/type/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6A21U72C7LOBO','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6C21U72K4LC3R','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6E21U7136H8B1','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6G21U706Q0MHN','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6I21U71SHCENM','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6K21U723B30TT','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-menAuth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6M21U73DQ23MR','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/get_auth_list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6O21U728ALAP5','00UHIKGRA7JVIEU25NNGI8KTJU','system_auth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6Q21U71JR1MR4','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-user','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6S21U73FAP0I6','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C6U21U72OCEIEC','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/edit','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7021U73JM9R8L','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/add','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7221U73BTDDFG','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7421U70MPGJ75','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/reset_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7621U72U518M2','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/update_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7821U7306UO0L','00UHIKGRA7JVIEU25NNGI8KTJU','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7A21U72H11IQG','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/fresh_token','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7C21U71HI3KNJ','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/update_avatar','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7E21U73H7P5LS','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/get_profile','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7G21U739VH4S5','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/update_profile','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7I21U71S1L238','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-dept','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7K21U71H99M9S','00UHIKGRA7JVIEU25NNGI8KTJU','system/dept/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7M21U71AVIH14','00UHIKGRA7JVIEU25NNGI8KTJU','system/dept/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7O21U73MERF3P','00UHIKGRA7JVIEU25NNGI8KTJU','system/dept/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7Q21U72ADMPKG','00UHIKGRA7JVIEU25NNGI8KTJU','system/dept/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7S21U73EG0I3G','00UHIKGRA7JVIEU25NNGI8KTJU','system/dept/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C7U21U70F9PQ94','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-post','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8021U72FU6NUI','00UHIKGRA7JVIEU25NNGI8KTJU','system/post/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8221U73R74CPP','00UHIKGRA7JVIEU25NNGI8KTJU','system/post/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8421U72DQJI2J','00UHIKGRA7JVIEU25NNGI8KTJU','system/post/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8621U73BH06TF','00UHIKGRA7JVIEU25NNGI8KTJU','system/post/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8821U70S38SDQ','00UHIKGRA7JVIEU25NNGI8KTJU','system/post/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8A21U71IACR6K','00UHIKGRA7JVIEU25NNGI8KTJU','M-sys-role','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8C21U72GVROBQ','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8E21U73EIF1H5','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8G21U71JS64JK','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8I21U73ET729R','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8K21U70SRULDM','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/set_data_scope','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8M21U70KR4VJF','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/change_status','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8O21U71L72A15','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8Q21U72BNV5LD','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/cancel_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8S21U70HBGT9T','00UHIKGRA7JVIEU25NNGI8KTJU','system/role/add_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C8U21U704EI5MT','00UHIKGRA7JVIEU25NNGI8KTJU','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C9021U73JL134U','00UHIKGRA7JVIEU25NNGI8KTJU','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C9221U7019BCJH','00UHIKGRA7JVIEU25NNGI8KTJU','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VS1JOLQ716C9421U718NU781','00UHIKGRA7JVIEU25NNGI8KTJU','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-03 15:07:14'),
('00VTAR3G593I133HLE0VHOQLKJ','00VTAOT7820JCROC5CSKO4PVT5','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I135HLE0TAECU5A','00VTAOT7820JCROC5CSKO4PVT5','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I137HLE0UIMACEK','00VTAOT7820JCROC5CSKO4PVT5','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I139HLE0TKAJC33','00VTAOT7820JCROC5CSKO4PVT5','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I13BHLE0SOR9PAK','00VTAOT7820JCROC5CSKO4PVT5','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I13DHLE0VKGD96S','00VTAOT7820JCROC5CSKO4PVT5','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I13FHLE0UT10D15','00VTAOT7820JCROC5CSKO4PVT5','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR3G593I13HHLE0UJI514F','00VTAOT7820JCROC5CSKO4PVT5','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:18'),
('00VTAR64JFHB0AGKDDIFSA7KHI','00VTAPCFV0MIB252HAA89LBD46','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AIKDDIEFB7BM2','00VTAPCFV0MIB252HAA89LBD46','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AKKDDIDGIARU0','00VTAPCFV0MIB252HAA89LBD46','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AMKDDIEP7EL49','00VTAPCFV0MIB252HAA89LBD46','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AOKDDICB7NDNE','00VTAPCFV0MIB252HAA89LBD46','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AQKDDIDBPN6OC','00VTAPCFV0MIB252HAA89LBD46','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0ASKDDIE1G5GKL','00VTAPCFV0MIB252HAA89LBD46','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR64JFHB0AUKDDICO47IUI','00VTAPCFV0MIB252HAA89LBD46','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:23'),
('00VTAR8U7LL9UOOR622LB1TTVQ','00VTAQD9Q3RFU5HT172L8SMDVF','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UOQR622KF41MTK','00VTAQD9Q3RFU5HT172L8SMDVF','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UOSR622LRB7H7P','00VTAQD9Q3RFU5HT172L8SMDVF','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UOUR622NO9PBG6','00VTAQD9Q3RFU5HT172L8SMDVF','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UP0R622MTJKRO9','00VTAQD9Q3RFU5HT172L8SMDVF','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UP2R622M4685FR','00VTAQD9Q3RFU5HT172L8SMDVF','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UP4R622LEDAS9H','00VTAQD9Q3RFU5HT172L8SMDVF','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTAR8U7LL9UP6R622KNBEPR2','00VTAQD9Q3RFU5HT172L8SMDVF','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:29'),
('00VTARFSDCA4FGLPGBJHQRQ41Q','00VTAQSSFRU2961VV33V1C6R9R','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FGNPGBJJ5TKF0P','00VTAQSSFRU2961VV33V1C6R9R','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FGPPGBJG0HAA0V','00VTAQSSFRU2961VV33V1C6R9R','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FGRPGBJHB3UBOR','00VTAQSSFRU2961VV33V1C6R9R','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FGTPGBJG1FT7NV','00VTAQSSFRU2961VV33V1C6R9R','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FGVPGBJJ8Q0GLJ','00VTAQSSFRU2961VV33V1C6R9R','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FH1PGBJH50NE3P','00VTAQSSFRU2961VV33V1C6R9R','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTARFSDCA4FH3PGBJGEAEHKQ','00VTAQSSFRU2961VV33V1C6R9R','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:08:43'),
('00VTB0KCKE00JCN3GS5QBN2Q1D','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JCP3GS5PDRRF3F','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-basic','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JCR3GS5PFD8O3S','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-basic-menu','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JCT3GS5O8VCQDG','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JCV3GS5R04KL97','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JD13GS5O2RTDIR','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JD33GS5RFN56J4','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JD53GS5RM2LU9T','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JD73GS5PT6O50G','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-dict','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JD93GS5OVBQ631','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDB3GS5QLAJUDK','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/get_all','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDD3GS5OVUPVFD','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDF3GS5Q9O79VK','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDH3GS5P0U7UUJ','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDJ3GS5P0D51H1','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/type/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDL3GS5OK9ST81','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDN3GS5OI3QL4F','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDP3GS5RLO3DMC','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDR3GS5RNG44KL','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDT3GS5O66LS3L','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JDV3GS5PCFJE1Q','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-menAuth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JE13GS5ORPGQRE','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/get_auth_list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JE33GS5R6EP98Q','00UHKP89CT1NDVFN6Q0E8LO7NT','system_auth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JE53GS5OV0V9IJ','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-user','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JE73GS5O2TV4UE','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JE93GS5O8SC6M5','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/edit','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEB3GS5O56ENBK','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/add','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JED3GS5PS2F3DT','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEF3GS5RI7UPIR','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/reset_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEH3GS5R259D53','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/update_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEJ3GS5QHL2F9O','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/fresh_token','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEL3GS5R58JI5H','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/update_avatar','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEN3GS5OGIEBGR','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/get_profile','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEP3GS5PU03R99','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/update_profile','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JER3GS5P7GDRH8','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/change_status','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JET3GS5RUQNHO9','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-dept','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JEV3GS5RPG1AL4','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dept/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JF13GS5OLOMP5C','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dept/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JF33GS5RALUM1K','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dept/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JF53GS5R53B4HL','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dept/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JF73GS5P8URGSL','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dept/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JF93GS5QV9TQE2','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-post','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFB3GS5OLHVF7C','00UHKP89CT1NDVFN6Q0E8LO7NT','system/post/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFD3GS5OPCR001','00UHKP89CT1NDVFN6Q0E8LO7NT','system/post/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFF3GS5PVMENCG','00UHKP89CT1NDVFN6Q0E8LO7NT','system/post/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFH3GS5QO5VE0E','00UHKP89CT1NDVFN6Q0E8LO7NT','system/post/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFJ3GS5OF1JQ78','00UHKP89CT1NDVFN6Q0E8LO7NT','system/post/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFL3GS5QCVLGM3','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-role','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFN3GS5RMODK67','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFP3GS5QN065S0','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFR3GS5PTE1C18','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFT3GS5P8PN54A','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JFV3GS5OD3PGPR','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/set_data_scope','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JG13GS5PP6C8RM','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/change_status','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JG33GS5RVSBIV5','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JG53GS5O96VCCL','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/cancel_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JG73GS5O0EV6BL','00UHKP89CT1NDVFN6Q0E8LO7NT','system/role/add_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JG93GS5OQ85LDE','00UHKP89CT1NDVFN6Q0E8LO7NT','M-monitor','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGB3GS5PL6D4RN','00UHKP89CT1NDVFN6Q0E8LO7NT','M-monitor-logininfor','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGD3GS5PIK6DVN','00UHKP89CT1NDVFN6Q0E8LO7NT','system/login-log/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGF3GS5OPDMGU8','00UHKP89CT1NDVFN6Q0E8LO7NT','monitor/logininfor/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGH3GS5QR6K5LK','00UHKP89CT1NDVFN6Q0E8LO7NT','monitor/logininfor/clean','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGJ3GS5QVMER3J','00UHKP89CT1NDVFN6Q0E8LO7NT','M-monitor-operlog','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGL3GS5RR4C724','00UHKP89CT1NDVFN6Q0E8LO7NT','system/oper_log/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGN3GS5QIPDC7I','00UHKP89CT1NDVFN6Q0E8LO7NT','system/oper_log/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGP3GS5Q7F9FL8','00UHKP89CT1NDVFN6Q0E8LO7NT','system/oper_log/clean','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGR3GS5PH12E5T','00UHKP89CT1NDVFN6Q0E8LO7NT','sys-operlog-query','button','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGT3GS5OSPK99S','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-online','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JGV3GS5QPHSSQ9','00UHKP89CT1NDVFN6Q0E8LO7NT','system/online/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JH13GS5QM6G60G','00UHKP89CT1NDVFN6Q0E8LO7NT','system/online/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JH33GS5OQ3K4DQ','00UHKP89CT1NDVFN6Q0E8LO7NT','M-sys-job','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JH53GS5ONS6SFD','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JH73GS5P6U3B6E','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JH93GS5O994KUA','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/change_status','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHB3GS5QJ04790','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/run_task_once','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHD3GS5OLJV8C2','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHF3GS5Q3LNVFF','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHH3GS5RH3N1JD','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHJ3GS5RVR0TOP','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job_log/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHL3GS5RVNSV62','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job_log/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHN3GS5RIFGUMS','00UHKP89CT1NDVFN6Q0E8LO7NT','system/job_log/clean','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHP3GS5OANLEEO','00UHKP89CT1NDVFN6Q0E8LO7NT','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHR3GS5RQMRBI2','00UHKP89CT1NDVFN6Q0E8LO7NT','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHT3GS5QNPAB26','00UHKP89CT1NDVFN6Q0E8LO7NT','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JHV3GS5RBB9F3P','00UHKP89CT1NDVFN6Q0E8LO7NT','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JI13GS5Q21G8BB','00UHKP89CT1NDVFN6Q0E8LO7NT','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JI33GS5Q2TVGFV','00UHKP89CT1NDVFN6Q0E8LO7NT','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JI53GS5OJVUG13','00UHKP89CT1NDVFN6Q0E8LO7NT','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VTB0KCKE00JI73GS5P25UHRB','00UHKP89CT1NDVFN6Q0E8LO7NT','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-04 15:14:20'),
('00VUR2CEH7QLBU0BI1MRC125PG','00UHIKGRA7JVIF025NNH39CPMT','M-sys','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBU2BI1MQDI1UK2','00UHIKGRA7JVIF025NNH39CPMT','M-sys-basic','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBU4BI1MOTDTU22','00UHIKGRA7JVIF025NNH39CPMT','M-sys-basic-menu','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBU6BI1MO3VFJ0J','00UHIKGRA7JVIF025NNH39CPMT','M-sys-dict','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBU8BI1MQJ1EJJV','00UHIKGRA7JVIF025NNH39CPMT','system_auth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUABI1MOJMODSA','00UHIKGRA7JVIF025NNH39CPMT','M-sys-user','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUCBI1MQNHE9BK','00UHIKGRA7JVIF025NNH39CPMT','M-sys-role','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUEBI1MRGUII78','00UHIKGRA7JVIF025NNH39CPMT','system/menu/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUGBI1MOOI69NC','00UHIKGRA7JVIF025NNH39CPMT','system/menu/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUIBI1MP3U3JS4','00UHIKGRA7JVIF025NNH39CPMT','system/dict/type/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUKBI1MOTT67JK','00UHIKGRA7JVIF025NNH39CPMT','system/dict/type/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUMBI1MQV4P0D1','00UHIKGRA7JVIF025NNH39CPMT','system/dict/data/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUOBI1MQ1TGTEK','00UHIKGRA7JVIF025NNH39CPMT','M-sys-menAuth','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUQBI1MRT8IMG4','00UHIKGRA7JVIF025NNH39CPMT','system/menu/get_auth_list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUSBI1MRDVP585','00UHIKGRA7JVIF025NNH39CPMT','system/user/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBUUBI1MQATRCFE','00UHIKGRA7JVIF025NNH39CPMT','system/user/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBV0BI1MR4155UN','00UHIKGRA7JVIF025NNH39CPMT','system/user/reset_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBV2BI1MRU3V9S8','00UHIKGRA7JVIF025NNH39CPMT','system/user/update_passwd','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBV4BI1MPDN96I7','00UHIKGRA7JVIF025NNH39CPMT','system/user/fresh_token','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBV6BI1MRTLA476','00UHIKGRA7JVIF025NNH39CPMT','system/user/update_avatar','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBV8BI1MRPCPAU1','00UHIKGRA7JVIF025NNH39CPMT','system/user/get_profile','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVABI1MRRAFTTM','00UHIKGRA7JVIF025NNH39CPMT','system/user/update_profile','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVCBI1MRGDFB15','00UHIKGRA7JVIF025NNH39CPMT','M-sys-dept','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVEBI1MRH2OAKG','00UHIKGRA7JVIF025NNH39CPMT','system/dept/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVGBI1MO709B5L','00UHIKGRA7JVIF025NNH39CPMT','system/dept/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVIBI1MQGJCFNK','00UHIKGRA7JVIF025NNH39CPMT','system/dept/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVKBI1MQT9TT30','00UHIKGRA7JVIF025NNH39CPMT','system/dept/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVMBI1MRC7E0EU','00UHIKGRA7JVIF025NNH39CPMT','system/dept/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVOBI1MQF1QPVU','00UHIKGRA7JVIF025NNH39CPMT','M-sys-post','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVQBI1MPRI4MLK','00UHIKGRA7JVIF025NNH39CPMT','system/post/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVSBI1MRIVN2SF','00UHIKGRA7JVIF025NNH39CPMT','system/post/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLBVUBI1MQQ8FT1R','00UHIKGRA7JVIF025NNH39CPMT','system/post/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC00BI1MON7RN38','00UHIKGRA7JVIF025NNH39CPMT','system/post/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC02BI1MP1D1BRC','00UHIKGRA7JVIF025NNH39CPMT','system/post/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC04BI1MQ0R789U','00UHIKGRA7JVIF025NNH39CPMT','system/role/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC06BI1MOIVG48S','00UHIKGRA7JVIF025NNH39CPMT','system/role/get_by_id','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC08BI1MOC2P82M','00UHIKGRA7JVIF025NNH39CPMT','system/role/add','POST','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0ABI1MRO6Q466','00UHIKGRA7JVIF025NNH39CPMT','system/role/edit','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0CBI1MQ3MN54A','00UHIKGRA7JVIF025NNH39CPMT','system/role/set_data_scope','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0EBI1MOBL7UCD','00UHIKGRA7JVIF025NNH39CPMT','system/role/delete','DELETE','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0GBI1MPARO6PH','00UHIKGRA7JVIF025NNH39CPMT','system/role/cancel_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0IBI1MQC9Q089','00UHIKGRA7JVIF025NNH39CPMT','system/role/add_auth_user','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0KBI1MPF0UN4G','00UHIKGRA7JVIF025NNH39CPMT','M-test','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0MBI1MQJOPRCL','00UHIKGRA7JVIF025NNH39CPMT','M-test-datascope','','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0OBI1MQQTSDC0','00UHIKGRA7JVIF025NNH39CPMT','test/data_scope/list','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0QBI1MQ6HP26T','00UHIKGRA7JVIF025NNH39CPMT','S-must-enabled','0','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0SBI1MRU158BA','00UHIKGRA7JVIF025NNH39CPMT','sysreme/user/change_role','PUT','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC0UBI1MRP2R0HN','00UHIKGRA7JVIF025NNH39CPMT','system/dict/data/get_by_type','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC10BI1MQ4HNL4B','00UHIKGRA7JVIF025NNH39CPMT','system/menu/get_routers','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58'),
('00VUR2CEH7QLC12BI1MPABP1LV','00UHIKGRA7JVIF025NNH39CPMT','system/user/get_info','GET','00TV87DDOBJPU75J4TGUOC3NNG','2022-03-05 19:13:58');

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
