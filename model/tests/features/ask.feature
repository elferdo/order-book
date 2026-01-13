Feature: Seller send ask orders
    Background:
        Given a seller named Susan

    Scenario: Susan sends one ask order to an empty market
        Given an empty market
        When  Susan sends an ask order not below 2.34
        Then  the market has 1 ask order
        And   sell price equals 2.34

    Scenario: Susan sends three ask orders to an empty market
        Given an empty market
        When  Susan sends an ask order not below 2.34
        And   Susan sends an ask order not below 1.78
        And   Susan sends an ask order not below 5.67
        Then  the market has 3 ask orders
        And   sell price equals 1.78

