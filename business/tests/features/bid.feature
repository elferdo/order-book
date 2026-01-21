Feature: Buyer send bid orders
    Background:
        Given a buyer named Bob

    Scenario: Bob sends one bid order to an empty market
        Given an empty market
        When  Bob sends a bid order not above 2.34
        Then  the market has 1 bid order
        And   buy price equals 2.34

    Scenario: Bob sends three bid orders to an empty market
        Given an empty market
        When  Bob sends a bid order not above 2.34
        And   Bob sends a bid order not above 1.78
        And   Bob sends a bid order not above 5.67
        Then  the market has 3 bid orders
        And   buy price equals 5.67

