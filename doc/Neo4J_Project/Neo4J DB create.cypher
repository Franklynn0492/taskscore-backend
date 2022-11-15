CREATE DATABASE taskscore IF NOT EXISTS;

CREATE OR REPLACE ROLE taskscore_admin_role AS COPY OF admin;

CREATE OR REPLACE USER tsadmin
	SET PASSWORD 'G3n3ricPwd' CHANGE NOT REQUIRED
	SET STATUS ACTIVE
	SET HOME DATABASE taskscore;
	
GRANT ROLE taskscore_admin_role TO tsadmin;

GRANT ALL ON HOME DATABASE TO taskscore_admin_role;

:USE taskscore;

