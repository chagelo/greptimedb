--- alter table to add new column with default timestamp values aware of session timezone test ---

CREATE TABLE test1 (i INTEGER, j TIMESTAMP time index, PRIMARY KEY(i));


ALTER TABLE test1 ADD COLUMN k INTEGER DEFAULT 100;
ALTER TABLE test1 ADD COLUMN l INTEGER DEFAULT 200;
ALTER TABLE test1 ADD COLUMN m TIMESTAMP DEFAULT '2024-01-30 00:00:00';
ALTER TABLE test1 ADD COLUMN n INTEGER DEFAULT 300;

--- set default value for k, l, n, n ---
ALTER TABLE test1 MODIFY COLUMN k SET DEFAULT 101;
ALTER TABLE test1 MODIFY COLUMN l SET DEFAULT 201, MODIFY COLUMN m SET DEFAULT '2024-01-30 00:00:00', MODIFY COLUMN n SET DEFAULT 300;

SHOW CREATE TABLE test1;

INSERT INTO test1 VALUES (1, '2024-01-30 12:00:00', DEFAULT, DEFAULT, DEFAULT, DEFAULT);

SELECT * FROM test1;

--- SET `CURRENT_TIMESTAMP` as default
ALTER TABLE test1 MODIFY COLUMN m SET DEFAULT CURRENT_TIMESTAMP;
SHOW CREATE TABLE test1;

ALTER TABLE test1 ADD COLUMN o INTEGER NOT NULL DEFAULT 400;

ALTER TABLE test1 MODIFY COLUMN o SET DEFAULT 401;

SHOW CREATE TABLE test1;

ALTER TABLE test1 MODIFY COLUMN o SET DEFAULT "not allow";

ALTER TABLE test1 MODIFY COLUMN o SET DEFAULT NULL;

DROP TABLE test1;
