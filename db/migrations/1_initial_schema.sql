CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS cud_version(
       version_number              BIGSERIAL          PRIMARY KEY,
       target_id                   UUID               NOT NULL,
       other_target_id             UUID,
       target_table                SMALLINT           NOT NULL,
       cud                         SMALLINT           NOT NULL
);

CREATE TABLE IF NOT EXISTS department(
       id                 UUID               PRIMARY KEY,
       boss_id            UUID,
       department_id      UUID,
       name               VARCHAR(20)        NOT NULL,
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       CONSTRAINT unique_department_name UNIQUE(name),
       CONSTRAINT chk_department_department_id CHECK(department_id <> id)
);

CREATE TABLE IF NOT EXISTS employee(
       id                 UUID               PRIMARY KEY,
       department_id      UUID               NOT NULL,
       position           VARCHAR(12)        NOT NULL,
       first_name         VARCHAR(40)        NOT NULL,
       middle_name        VARCHAR(40)        NOT NULL,
       last_name          VARCHAR(40)        NOT NULL,
       card_id            SMALLINT           NOT NULL,
       password           TEXT               NOT NULL,
       CONSTRAINT unique_employee_card_id UNIQUE(card_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       CONSTRAINT chk_employee_position CHECK(position in ('SUPER_ADMIN','ADMIN', 'SUPER_USER', 'USER'))
);

ALTER TABLE IF EXISTS department
ADD FOREIGN KEY(boss_id)
REFERENCES employee(id)
ON DELETE CASCADE;

CREATE TABLE IF NOT EXISTS machine(
       id                   UUID              PRIMARY KEY,
       name                 VARCHAR(100)      NOT NULL,
       CONSTRAINT unique_machine_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS spare_part(
       id                   UUID              PRIMARY KEY,
       name                 VARCHAR(100)      NOT NULL,
       CONSTRAINT unique_spare_part_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS problem(
       id                         UUID                         PRIMARY KEY,
       writer_id                  UUID                         NOT NULL,
       department_id              UUID                         NOT NULL,
       title                      VARCHAR(70)                  NOT NULL,
       description                VARCHAR(350)                 NOT NULL,
       CONSTRAINT unique_department_problem UNIQUE(title,department_id),
       FOREIGN KEY(writer_id)     REFERENCES employee(id)      ON DELETE CASCADE,
       FOREIGN KEY(department_id) REFERENCES department(id)    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift (
       id                   UUID              PRIMARY KEY,
       shift_order          SMALLINT          NOT NULL,
       shift_date           DATE              NOT NULL,
       CONSTRAINT unique_shift_identity UNIQUE(shift_order,shift_date),
       CONSTRAINT chk_shift_order CHECK(shift_order BETWEEN 0 AND 4)
);

CREATE TABLE IF NOT EXISTS department_shift (
       id                   UUID              PRIMARY KEY,
       department_id        UUID              NOT NULL,
       shift_id             UUID              NOT NULL,
       CONSTRAINT unique_department_shift_id UNIQUE(department_id,shift_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_id)      REFERENCES shift(id)      ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                  UUID                 PRIMARY KEY,
       shift_id            UUID                 NOT NULL,
       writer_id           UUID                 NOT NULL,
       maintainer_id       UUID                 NOT NULL,
       machine_id          UUID                 NOT NULL,
       begin_time          TIME                 NOT NULL,
       end_time            TIME                 NOT NULL,
       FOREIGN             KEY(maintainer_id)   REFERENCES employee(id)          ON DELETE CASCADE,
       FOREIGN             KEY(writer_id)       REFERENCES employee(id)          ON DELETE CASCADE,
       FOREIGN             KEY(machine_id)      REFERENCES machine(id)           ON DELETE CASCADE,
       FOREIGN             KEY(shift_id)        REFERENCES department_shift(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS note(
       id                  UUID                  PRIMARY KEY,
       shift_id            UUID,
       shift_problem_id    UUID,
       content             varchar(500)          NOT NULL,
       FOREIGN             KEY(shift_id)         REFERENCES department_shift(id)  ON DELETE CASCADE,
       FOREIGN             KEY(shift_problem_id) REFERENCES shift_problem(id)     ON DELETE CASCADE,
       CONSTRAINT chk_note_home CHECK(
         shift_id IS NOT NULL AND shift_problem_id IS NULL
         OR
         shift_id IS NULL AND shift_problem_id IS NOT NULL
       )
);

CREATE TABLE IF NOT EXISTS shift_problem_problem(
       shift_problem_id     UUID              NOT NULL,
       problem_id           UUID              NOT NULL,
       PRIMARY KEY(shift_problem_id,problem_id),
       FOREIGN KEY(problem_id) REFERENCES problem(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_problem_id) REFERENCES shift_problem(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_problem_spare_part(
       shift_problem_id     UUID              NOT NULL,
       spare_part_id        UUID              NOT NULL,
       PRIMARY KEY(shift_problem_id,spare_part_id),
       FOREIGN KEY(spare_part_id) REFERENCES spare_part(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_problem_id) REFERENCES shift_problem(id) ON DELETE CASCADE
);

INSERT INTO cud_version(target_id,other_target_id,target_table,cud)
SELECT '00000000-0000-0000-0000-000000000000',
'00000000-0000-0000-0000-000000000000',0,0
WHERE (SELECT count(version_number) from cud_version) = 0;
