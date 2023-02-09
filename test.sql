
---------------------------------------------------------------------------------
------------------------------------- testing data --------------------------------------
---------------------------------------------------------------------------------
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 1');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 2');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 3');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 4');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 5');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 6');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 7');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 8');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 9');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 11');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 12');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 13');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 14');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 15');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 16');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 17');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 18');
insert into spare_part(id,"name") values(gen_random_uuid(),'قطعة 19');


insert into department (id,"name") values('ffade865-925f-43ee-8379-884dd05ca5eb','الافران');
insert into department (id,"name") values('0ec10642-9477-49a5-86fc-1e6df9f8f06b','المجففات');
insert into department (id,"name") values('e3f92589-ac8b-4c7a-aa55-7e9998d420cd','الانكجت');
insert into department (id,"name") values('9c9d1b7b-b7be-4496-8b78-ef77a9d857b0','المتابعة');
insert into department (id,"name") values('55de09d3-3136-4ac5-884d-db8c2cba2caa','الفرز');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'ffade865-925f-43ee-8379-884dd05ca5eb',1,'USER','محمد','احمد','محمود',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'ffade865-925f-43ee-8379-884dd05ca5eb',2,'SUPER_USER','احمد','عبد السلام','عبد الهادي',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'ffade865-925f-43ee-8379-884dd05ca5eb',3,'USER','علاء','هلال','صبري',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'ffade865-925f-43ee-8379-884dd05ca5eb',4,'USER','حسين','اشرف','هاني',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'ffade865-925f-43ee-8379-884dd05ca5eb',5,'ADMIN','حسن','عبد الباقي','حسين',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'0ec10642-9477-49a5-86fc-1e6df9f8f06b',6,'SUPER_USER','اشرف','سعد','سمير',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'0ec10642-9477-49a5-86fc-1e6df9f8f06b',7,'USER','عبد الباقي','سمير','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'0ec10642-9477-49a5-86fc-1e6df9f8f06b',8,'USER','سعد','محمد','محمود',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'0ec10642-9477-49a5-86fc-1e6df9f8f06b',9,'USER','سمير','احمد','محمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'0ec10642-9477-49a5-86fc-1e6df9f8f06b',10,'ADMIN','عمر','محمد','المنجد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'e3f92589-ac8b-4c7a-aa55-7e9998d420cd',11,'SUPER_USER','شعبان','سمير','محمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'e3f92589-ac8b-4c7a-aa55-7e9998d420cd',12,'USER','سيد','عبد الجواد','محمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'e3f92589-ac8b-4c7a-aa55-7e9998d420cd',13,'USER','منتصر','هلال','رمضان',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'e3f92589-ac8b-4c7a-aa55-7e9998d420cd',14,'USER','مصطفي','حسين','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'e3f92589-ac8b-4c7a-aa55-7e9998d420cd',15,'ADMIN','طارق','يحي','مهدي',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'9c9d1b7b-b7be-4496-8b78-ef77a9d857b0',16,'SUPER_USER','يحي','عاكف','علاء',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'9c9d1b7b-b7be-4496-8b78-ef77a9d857b0',17,'USER','مهدي','عباس','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'9c9d1b7b-b7be-4496-8b78-ef77a9d857b0',18,'USER','يوسف','رمضان','الطحاوي',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'9c9d1b7b-b7be-4496-8b78-ef77a9d857b0',19,'USER','رمضان','عباس','محمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'9c9d1b7b-b7be-4496-8b78-ef77a9d857b0',20,'USER','رجب','عبد الله','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',21,'ADMIN','عبد الله','سعد','سمير',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',22,'SUPER_USER','عبد الرحمن','علاء','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',23,'USER','عبد التواب','ذاكر','احمد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',24,'USER','حكيم','محمد','عبد التواب',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',25,'USER','طه','شعبان','سمير',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');

insert into employee (id,department_id,card_id,"position",first_name,middle_name,last_name,"password")
values(gen_random_uuid(),'55de09d3-3136-4ac5-884d-db8c2cba2caa',26,'USER','معتز','هلال','سيد',
'$2a$08$RFxIlKzJ4RGVVnfaVEocYuJvIZjQLF9w8BVGskwdTDf.K0hBzsOOy');



insert into machine (id,"name") values(gen_random_uuid(),'فرن 1');
insert into machine (id,"name") values(gen_random_uuid(),'فرن 2');
insert into machine (id,"name") values(gen_random_uuid(),'فرن 3');
insert into machine (id,"name") values(gen_random_uuid(),'فرن 4');
insert into machine (id,"name") values(gen_random_uuid(),'فرن 5');
insert into machine (id,"name") values(gen_random_uuid(),'فرن 6');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 1');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 2');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 3');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 4');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 5');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 6');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 7');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 8');
insert into machine (id,"name") values(gen_random_uuid(),'مجفف 9');
