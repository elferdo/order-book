Feature: Managing the lifecycle of a user
    Scenario: Onboard a new user
        Given an empty world
        When a new user is onboarded
        Then a Uuid is returned


