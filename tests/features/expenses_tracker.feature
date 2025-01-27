Feature: migrations

    Scenario: Run a migration
        Given a connection pool
        Then migrations run
