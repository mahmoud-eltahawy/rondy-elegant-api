CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

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
       first_name         VARCHAR(20)        NOT NULL,
       middle_name        VARCHAR(20)        NOT NULL,
       last_name          VARCHAR(20)        NOT NULL,
       card_id            SMALLINT           NOT NULL,
       password           TEXT               NOT NULL,
       CONSTRAINT unique_employee_card_id UNIQUE(card_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       CONSTRAINT chk_employee_position CHECK(position in ('ADMIN', 'SUPER_USER', 'USER'))
);

CREATE INDEX IF NOT EXISTS idx_of_employee_first_name ON employee(first_name);

ALTER TABLE IF EXISTS department
ADD FOREIGN KEY(boss_id)
REFERENCES employee(id)
ON DELETE CASCADE;

CREATE TABLE IF NOT EXISTS machine(
       id                   UUID              PRIMARY KEY,
       name                 VARCHAR(100)      NOT NULL,
       CONSTRAINT unique_machine_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS problem(
       id                   UUID              PRIMARY KEY,
       title                VARCHAR(70)       NOT NULL,
       description          VARCHAR(350)      NOT NULL
);

CREATE TABLE IF NOT EXISTS shift (
       id                   UUID              PRIMARY KEY,
       department_id        UUID              NOT NULL,
       shift_order          SMALLINT          NOT NULL,
       shift_date           DATE              NOT NULL,
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       CONSTRAINT unique_shift_identity UNIQUE(shift_order,shift_date),
       CONSTRAINT chk_shift_order CHECK(shift_order BETWEEN 0 AND 4)
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                   UUID              PRIMARY KEY,
       shift_id             UUID              NOT NULL,
       problem_id           UUID              NOT NULL,
       machine_id           UUID              NOT NULL,
       maintainer_id        UUID              NOT NULL,
       begin_time           TIME              NOT NULL,
       end_time             TIME              NOT NULL,
       comment              VARCHAR(400)      NOT NULL,
       FOREIGN KEY(maintainer_id) REFERENCES employee(id) ON DELETE CASCADE,
       FOREIGN KEY(machine_id) REFERENCES machine(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_id) REFERENCES shift(id) ON DELETE CASCADE,
       FOREIGN KEY(problem_id) REFERENCES problem(id) ON DELETE CASCADE,
);
