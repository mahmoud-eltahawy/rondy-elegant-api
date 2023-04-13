CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS department(
       id                 UUID               PRIMARY KEY,
       boss_id            UUID,
       name               VARCHAR(20)        NOT NULL,
       deleted            BOOL               NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_department_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS employee(
       id                 UUID               PRIMARY KEY,
       department_id      UUID               NOT NULL,
       position           VARCHAR(12)        NOT NULL,
       first_name         VARCHAR(40)        NOT NULL,
       middle_name        VARCHAR(40)        NOT NULL,
       last_name          VARCHAR(40)        NOT NULL,
       card_id            BIGINT             NOT NULL,
       password           TEXT               NOT NULL,
       deleted            BOOL               NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_employee_card_id UNIQUE(card_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       CONSTRAINT chk_employee_position CHECK(position in ('SUPER_USER', 'USER'))
);

ALTER TABLE department
ADD FOREIGN KEY(boss_id)
REFERENCES employee(id)
ON DELETE CASCADE;

CREATE TABLE IF NOT EXISTS cd_version(
       version_number              BIGSERIAL             PRIMARY KEY,
       updater_id                  UUID                  NOT NULL,
       time_stamp                  TIMESTAMP             NOT NULL,
       target_id                   UUID                  NOT NULL,
       target_table                VARCHAR(30)           NOT NULL,
       cd                          VARCHAR(30)           NOT NULL
);

CREATE INDEX IF NOT EXISTS cd_version_target_id ON cd_version(target_id);
CREATE INDEX IF NOT EXISTS cd_version_updater_id ON cd_version(updater_id);

CREATE TABLE IF NOT EXISTS update_version(
       version_number              BIGSERIAL             PRIMARY KEY,
       updater_id                  UUID                  NOT NULL,
       time_stamp                  TIMESTAMP             NOT NULL,
       target_id                   UUID                  NOT NULL,
       json                        JSONB                 NOT NULL,
       FOREIGN KEY(updater_id) REFERENCES employee(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS update_version_target_id ON update_version(target_id);
CREATE INDEX IF NOT EXISTS update_version_updater_id ON update_version(updater_id);

CREATE TABLE IF NOT EXISTS permissions(
       employee_id          UUID               NOT NULL,
       permission           VARCHAR(50)        NOT NULL,
       PRIMARY KEY(employee_id,permission),
       FOREIGN KEY(employee_id) REFERENCES employee(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS machine(
       id                   UUID              PRIMARY KEY,
       name                 VARCHAR(100)      NOT NULL,
       deleted              BOOL              NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_machine_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS spare_part(
       id                   UUID              PRIMARY KEY,
       name                 VARCHAR(100)      NOT NULL,
       deleted              BOOL              NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_spare_part_name UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS problem(
       id                         UUID                         PRIMARY KEY,
       department_id              UUID                         NOT NULL,
       title                      VARCHAR(70)                  NOT NULL,
       description                VARCHAR(350)                 NOT NULL,
       deleted                    BOOL                         NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_department_problem UNIQUE(title,department_id),
       FOREIGN KEY(department_id) REFERENCES department(id)    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift (
       id                   UUID              PRIMARY KEY,
       shift_order          VARCHAR(20)       NOT NULL,
       shift_date           DATE              NOT NULL,
       deleted              BOOL              NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_shift_identity UNIQUE(shift_order,shift_date)
);

CREATE TABLE IF NOT EXISTS department_shift (
       id                   UUID              PRIMARY KEY,
       department_id        UUID              NOT NULL,
       shift_id             UUID              NOT NULL,
       deleted              BOOL              NOT NULL DEFAULT FALSE,
       CONSTRAINT unique_department_shift_id UNIQUE(department_id,shift_id),
       FOREIGN KEY(department_id) REFERENCES department(id) ON DELETE CASCADE,
       FOREIGN KEY(shift_id)      REFERENCES shift(id)      ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_note(
       id                  UUID                  PRIMARY KEY,
       shift_id            UUID                  NOT NULL,
       content             VARCHAR(500)          NOT NULL,
       FOREIGN KEY(shift_id) REFERENCES department_shift(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS department_shift_employee (
       department_shift_id  UUID              NOT NULL,
       employee_id          UUID              NOT NULL,
       PRIMARY KEY(department_shift_id,employee_id),
       FOREIGN KEY(employee_id) REFERENCES employee(id) ON DELETE CASCADE,
       FOREIGN KEY(department_shift_id) REFERENCES department_shift(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_problem(
       id                  UUID                 PRIMARY KEY,
       shift_id            UUID                 NOT NULL,
       maintainer_id       UUID                 NOT NULL,
       machine_id          UUID                 NOT NULL,
       begin_time          TIME                 NOT NULL,
       end_time            TIME                 NOT NULL,
       FOREIGN             KEY(maintainer_id)   REFERENCES employee(id)          ON DELETE CASCADE,
       FOREIGN             KEY(machine_id)      REFERENCES machine(id)           ON DELETE CASCADE,
       FOREIGN             KEY(shift_id)        REFERENCES department_shift(id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shift_problem_note(
       id                  UUID                  PRIMARY KEY,
       content             VARCHAR(500)          NOT NULL,
       FOREIGN KEY(id) REFERENCES shift_problem(id) ON DELETE CASCADE
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
